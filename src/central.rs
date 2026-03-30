#![no_std]
#![no_main]

mod vial;
#[macro_use]
mod macros;
mod keymap;

use defmt::info;
use defmt_rtt as _;
use embassy_embedded_hal::adapter::BlockingAsync;
use embassy_executor::Spawner;
use embassy_nrf::buffered_uarte;
use embassy_nrf::gpio::Output;
use embassy_nrf::peripherals::{UARTE0, USBD};
use embassy_nrf::uarte::Config as UarteConfig;
use embassy_nrf::usb::Driver;
use embassy_nrf::usb::vbus_detect::HardwareVbusDetect;
use embassy_nrf::{bind_interrupts, usb};
use panic_probe as _;
use rmk::builtin_processor::led_indicator::KeyboardIndicatorProcessor;
use rmk::config::{
    BehaviorConfig, DeviceConfig, PositionalConfig, RmkConfig, StorageConfig, VialConfig,
};
use rmk::debounce::default_debouncer::DefaultDebouncer;
use rmk::futures::future::join5;
use rmk::input_device::Runnable;
use rmk::keyboard::Keyboard;
use rmk::matrix::Matrix;
use rmk::split::SPLIT_MESSAGE_MAX_SIZE;
use rmk::split::central::run_peripheral_manager;
use rmk::{KeymapData, initialize_keymap_and_storage, run_all, run_rmk};
use static_cell::StaticCell;
use vial::{VIAL_KEYBOARD_DEF, VIAL_KEYBOARD_ID};

bind_interrupts!(struct Irqs {
    USBD => usb::InterruptHandler<USBD>;
    CLOCK_POWER => usb::vbus_detect::InterruptHandler;
    UARTE0 => buffered_uarte::InterruptHandler<UARTE0>;
});

const ROW: usize = 5;
const COL: usize = 12;
const NUM_LAYER: usize = 5;
const NUM_ENCODER: usize = 0;
const CENTRAL_COL: usize = 6;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("RMK start!");
    let mut nrf_config = embassy_nrf::config::Config::default();
    nrf_config.dcdc.reg0_voltage = Some(embassy_nrf::config::Reg0Voltage::_3V3);
    nrf_config.dcdc.reg0 = true;
    nrf_config.dcdc.reg1 = true;
    let p = embassy_nrf::init(nrf_config);

    let driver = Driver::new(p.USBD, Irqs, HardwareVbusDetect::new(Irqs));

    let keyboard_device_config = DeviceConfig {
        vid: 0x4c4b,
        pid: 0x4643,
        manufacturer: "marcoster",
        product_name: "MASK Keyboard",
        serial_number: "vial:f64c2b3c:000001",
    };
    let vial_config = VialConfig::new(VIAL_KEYBOARD_ID, VIAL_KEYBOARD_DEF, &[(0, 0), (1, 1)]);
    let storage_config = StorageConfig {
        // clear_storage: true,
        start_addr: 0xA0000,
        num_sectors: 6,
        ..Default::default()
    };
    let rmk_config = RmkConfig {
        device_config: keyboard_device_config,
        vial_config,
        storage_config,
        ..Default::default()
    };

    let mut keymap_data = KeymapData::new(keymap::get_default_keymap());
    let mut behavior_config = BehaviorConfig::default();
    behavior_config.morse.enable_flow_tap = true;
    let key_config = PositionalConfig::default();

    let flash = embassy_nrf::nvmc::Nvmc::new(p.NVMC);
    let flash = BlockingAsync::new(flash);

    let (keymap, mut storage) =
        initialize_keymap_and_storage::<_, ROW, COL, NUM_LAYER, NUM_ENCODER>(
            &mut keymap_data,
            flash,
            &storage_config,
            &mut behavior_config,
            &key_config,
        )
        .await;

    let mut keyboard = Keyboard::new(&keymap);

    static TX_BUF: StaticCell<[u8; SPLIT_MESSAGE_MAX_SIZE]> = StaticCell::new();
    let tx_buf = &mut TX_BUF.init([0; SPLIT_MESSAGE_MAX_SIZE])[..];
    static RX_BUF: StaticCell<[u8; SPLIT_MESSAGE_MAX_SIZE]> = StaticCell::new();
    let rx_buf = &mut RX_BUF.init([0; SPLIT_MESSAGE_MAX_SIZE])[..];

    let uart = buffered_uarte::BufferedUarte::new(
        p.UARTE0,
        p.TIMER0,
        p.PPI_CH0,
        p.PPI_CH1,
        p.PPI_GROUP0,
        p.P1_10,
        p.P1_13,
        Irqs,
        UarteConfig::default(),
        rx_buf,
        tx_buf,
    );

    //let (row_pins, col_pins) = config_matrix_pins_nrf!(peripherals: p, input: [P0_04, P0_28, P1_12, P0_03, P1_00], output:  [P0_02, P1_15, P0_26, P0_27, P0_05, P1_01]);
    let (row_pins, col_pins) = config_matrix_pins_nrf!(peripherals: p, input: [P0_08, P0_04, P0_28, P1_12, P0_03], output:  [P0_02, P1_15, P0_26, P0_27, P0_05, P0_06]);

    let debouncer = DefaultDebouncer::new();
    let mut matrix = Matrix::<_, _, _, ROW, CENTRAL_COL, true>::new(row_pins, col_pins, debouncer);

    let mut capslock_led = KeyboardIndicatorProcessor::new(
        Output::new(
            p.P0_00,
            embassy_nrf::gpio::Level::Low,
            embassy_nrf::gpio::OutputDrive::Standard,
        ),
        false,
        rmk::types::led_indicator::LedIndicatorType::CapsLock,
    );

    join5(
        run_all!(matrix),
        keyboard.run(),
        run_peripheral_manager::<ROW, CENTRAL_COL, 0, CENTRAL_COL, _>(0, uart),
        run_rmk(&keymap, driver, &mut storage, rmk_config),
        capslock_led.run(),
    )
    .await;
}
