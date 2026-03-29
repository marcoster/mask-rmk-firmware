#![no_std]
#![no_main]

#[macro_use]
mod macros;

use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;

use embassy_nrf::bind_interrupts;
use embassy_nrf::buffered_uarte;
use embassy_nrf::peripherals::UARTE0;
use embassy_nrf::uarte::Config as UarteConfig;
use panic_probe as _;
use rmk::debounce::default_debouncer::DefaultDebouncer;
use rmk::futures::future::join;
use rmk::matrix::Matrix;
use rmk::run_all;
use rmk::split::SPLIT_MESSAGE_MAX_SIZE;
use rmk::split::peripheral::run_rmk_split_peripheral;
use static_cell::StaticCell;

bind_interrupts!(struct Irqs {
    UARTE0 => buffered_uarte::InterruptHandler<UARTE0>;
});

const ROW: usize = 5;
const PERIPHERAL_COL: usize = 6;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("RMK start!");
    let mut nrf_config = embassy_nrf::config::Config::default();
    nrf_config.dcdc.reg0_voltage = Some(embassy_nrf::config::Reg0Voltage::_3V3);
    nrf_config.dcdc.reg0 = true;
    nrf_config.dcdc.reg1 = true;
    let p = embassy_nrf::init(nrf_config);

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
        p.P1_13,
        p.P1_10,
        Irqs,
        UarteConfig::default(),
        rx_buf,
        tx_buf,
    );

    //let (row_pins, col_pins) = config_matrix_pins_nrf!(peripherals: p, input: [P0_04, P0_28, P1_12, P0_03, P1_00], output:  [P1_01, P0_05, P0_27, P0_26, P1_15, P0_02]);
    let (row_pins, col_pins) = config_matrix_pins_nrf!(peripherals: p, input: [P0_08, P0_04, P0_28, P1_12, P0_03], output:  [P0_06, P0_05, P0_27, P0_26, P1_15, P0_02]);

    let debouncer = DefaultDebouncer::new();
    let mut matrix =
        Matrix::<_, _, _, ROW, PERIPHERAL_COL, true>::new(row_pins, col_pins, debouncer);

    join(run_all!(matrix), run_rmk_split_peripheral(uart)).await;
}
