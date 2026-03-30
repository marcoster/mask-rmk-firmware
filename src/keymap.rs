use rmk::types::action::KeyAction;
use rmk::{a, df, k, kbctrl, mo, shifted};

pub(crate) const COL: usize = 12;
pub(crate) const ROW: usize = 5;
pub(crate) const NUM_LAYER: usize = 5;

#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        // Layer 0: GRAPHITE_MOD (base layout)
        [
            [k!(Escape), k!(Kc1), k!(Kc2), k!(Kc3), k!(Kc4), k!(Kc5),                 k!(Kc6), k!(Kc7),  k!(Kc8),   k!(Kc9), k!(Kc0),       k!(Backspace)],
            [k!(Tab),    k!(B),   k!(L),   k!(D),   k!(W),   k!(Z),                   k!(J),   k!(F),    k!(O),     k!(U),   k!(Semicolon), k!(Delete)],
            [k!(LShift), k!(N),   k!(R),   k!(T),   k!(S),   k!(G),                   k!(Y),   k!(H),    k!(A),     k!(E),   k!(I),         k!(Quote)],
            [k!(LCtrl),  k!(Q),   k!(X),   k!(M),   k!(C),   k!(V),                   k!(K),   k!(P),    k!(Comma), k!(Dot), k!(Slash),     k!(RShift)],
            [k!(LCtrl),  k!(LAlt),k!(LGui),mo!(1),  mo!(2),  k!(KbMute),              df!(4),  k!(Enter),k!(Space), k!(RGui),k!(RAlt),      k!(RCtrl)]

        ],

        // Layer 1: LOWER (numbers, symbols, F-keys)
        [
            [a!(No),          k!(Kc1),         k!(Kc2),         k!(Kc3),         k!(Kc4),               k!(Kc5),                    k!(Kc6),         k!(Kc7),          k!(Kc8),             k!(Kc9),             k!(Kc0),         k!(Backspace)],
            [shifted!(Grave), k!(Kc1),         k!(Kc2),         k!(Kc3),         k!(Kc4),               k!(Kc5),                    k!(Kc6),         k!(Kc7),          k!(Kc8),             k!(Kc9),             k!(Kc0),         k!(Minus)],
            [k!(Grave),       shifted!(Kc1),   shifted!(Kc2),   shifted!(Kc3),   shifted!(Kc4),         shifted!(Kc5),              shifted!(Kc6),   shifted!(Kc7),    shifted!(Kc8),       shifted!(Kc9),       shifted!(Kc0),   shifted!(Minus)],
            [a!(Transparent), k!(Equal),       k!(Minus),       k!(Equal),       shifted!(LeftBracket), shifted!(RightBracket),     k!(LeftBracket), k!(RightBracket), shifted!(Backslash), shifted!(Semicolon), k!(Backslash),   a!(No)],
            [a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), mo!(3),                a!(No),                     a!(No),          a!(Transparent),  a!(Transparent),     a!(Transparent),     a!(Transparent), a!(Transparent)],
        ],

        // Layer 2: RAISE (functions, navigation, media)
        [
            [a!(Transparent), k!(F1),   k!(F2),   k!(F3),   k!(F4),           k!(F5),           k!(F6),     k!(F7),       k!(F8),     k!(F9),     k!(F10),  k!(F11)],
            [k!(CapsLock),    a!(No),   k!(Up),   a!(No),   k!(KbVolumeUp),   a!(No),           k!(PageUp), k!(PageDown), k!(Delete), k!(Insert), a!(No),   k!(F12)],
            [a!(Transparent), k!(Left), k!(Down), k!(Right),k!(KbVolumeDown), a!(No),           k!(Left),   k!(Down),     k!(Up),     k!(Right),  k!(Home), k!(End)],
            [a!(Transparent), a!(No),   a!(No),   a!(No),   k!(KbMute),       a!(No),           a!(No),     a!(No),       a!(No),     a!(No),     a!(No),   a!(Transparent)],
            [a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), mo!(3), a!(Transparent),      a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent)],
        ],

        // Layer 3: ADJUST (bootloader, layer switches)
        // New design: QK_BOOT at col 0, df!(0) to GRAPHITE, to!(4) to GAME
        [
            [kbctrl!(Bootloader), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), df!(0), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), df!(0), df!(4), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(Transparent)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
        ],

        // Layer 4: GAME (gaming layout, exclusive layer switch)
        [
            [k!(Escape), k!(Kc1), k!(Kc2), k!(Kc3), k!(Kc4), k!(Kc5),        k!(Kc6), k!(Kc7),  k!(Kc8),   k!(Kc9), k!(Kc0),       k!(Backspace)],
            [k!(Tab),    k!(Q),   k!(W),   k!(E),   k!(R),   k!(T),          k!(Y),   k!(U),    k!(I),     k!(O),   k!(P),         k!(Enter)],
            [k!(LShift), k!(A),   k!(S),   k!(D),   k!(F),   k!(G),          k!(H),   k!(J),    k!(K),     k!(L),   k!(Semicolon), k!(Quote)],
            [k!(LCtrl),  k!(Z),   k!(X),   k!(C),   k!(V),   k!(B),          k!(N),   k!(M),    k!(Comma), k!(Dot), k!(Slash),     k!(RShift)],
            [shifted!(Kc6), k!(M),k!(LAlt),k!(Space),k!(Enter),k!(KbMute),   df!(0),  mo!(2),   mo!(1),    k!(RGui),k!(RAlt),      k!(RCtrl)]


        ],
    ]
}
