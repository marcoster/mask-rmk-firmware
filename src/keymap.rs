use rmk::types::action::KeyAction;
use rmk::{a, k, kbctrl, layer, mo};
pub(crate) const COL: usize = 12;
pub(crate) const ROW: usize = 5;
pub(crate) const NUM_LAYER: usize = 5;

// LAYER NUMBERS (cannot be used in mo! macro sadly)
// const GRAPHITE: usize = 0;
// const SYMBOL: usize = 1;
// const MOVE: usize = 2;
// const SPECIAL: usize = 3;
// const GAME: usize = 4;

#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        [
            [k!(Escape), k!(Kc1), k!(Kc2), k!(Kc3), k!(Kc4), k!(Kc5),                 k!(Kc6),             k!(Kc7),  k!(Kc8),   k!(Kc9), k!(Kc0),       k!(Backspace)], 
            [k!(Tab),    k!(B),   k!(L),   k!(D),   k!(W),   k!(Z),                   k!(J),               k!(F),    k!(O),     k!(U),   k!(Semicolon), k!(Delete)], 
            [k!(LShift), k!(N),   k!(R),   k!(T),   k!(S),   k!(G),                   k!(Y),               k!(H),    k!(A),     k!(E),   k!(I),         k!(Quote)], 
            [k!(LCtrl),  k!(Q),   k!(X),   k!(M),   k!(C),   k!(V),                   k!(K),               k!(P),    k!(Comma), k!(Dot), k!(Slash),     k!(RShift)], 
            [k!(LCtrl),  k!(LAlt),k!(LGui),mo!(1),  mo!(2),  kbctrl!(Bootloader),     kbctrl!(Bootloader), k!(Enter),k!(Space), k!(RGui),k!(RAlt),      k!(RCtrl)]
        ],
        [
            [k!(Tab), k!(B), k!(L), k!(D), k!(W), k!(Z), k!(Tab), k!(B), k!(L), k!(D), k!(W), k!(Z)], 
            [k!(Tab), k!(B), k!(L), k!(D), k!(W), k!(Z), k!(Tab), k!(B), k!(L), k!(D), k!(W), k!(Z)], 
            [k!(Tab), k!(B), k!(L), k!(D), k!(W), k!(Z), k!(Tab), k!(B), k!(L), k!(D), k!(W), k!(Z)], 
            [k!(Tab), k!(B), k!(L), k!(D), k!(W), k!(Z), k!(Tab), k!(B), k!(L), k!(D), k!(W), k!(Z)], 
            [k!(Tab), k!(B), k!(L), k!(D), k!(W), k!(Z), k!(Tab), k!(B), k!(L), k!(D), k!(W), k!(Z)]
        ],
        [
            [k!(Tab), k!(B), k!(L), k!(D), k!(W), k!(Z), k!(Tab), k!(B), k!(L), k!(D), k!(W), k!(Z)], 
            [k!(Tab), k!(B), k!(L), k!(D), k!(W), k!(Z), k!(Tab), k!(B), k!(L), k!(D), k!(W), k!(Z)], 
            [k!(Tab), k!(B), k!(L), k!(D), k!(W), k!(Z), k!(Tab), k!(B), k!(L), k!(D), k!(W), k!(Z)], 
            [k!(Tab), k!(B), k!(L), k!(D), k!(W), k!(Z), k!(Tab), k!(B), k!(L), k!(D), k!(W), k!(Z)], 
            [k!(Tab), k!(B), k!(L), k!(D), k!(W), k!(Z), k!(Tab), k!(B), k!(L), k!(D), k!(W), k!(Z)]
        ],
        [
            [k!(Tab), k!(B), k!(L), k!(D), k!(W), k!(Z), k!(Tab), k!(B), k!(L), k!(D), k!(W), k!(Z)], 
            [k!(Tab), k!(B), k!(L), k!(D), k!(W), k!(Z), k!(Tab), k!(B), k!(L), k!(D), k!(W), k!(Z)], 
            [k!(Tab), k!(B), k!(L), k!(D), k!(W), k!(Z), k!(Tab), k!(B), k!(L), k!(D), k!(W), k!(Z)], 
            [k!(Tab), k!(B), k!(L), k!(D), k!(W), k!(Z), k!(Tab), k!(B), k!(L), k!(D), k!(W), k!(Z)], 
            [k!(Tab), k!(B), k!(L), k!(D), k!(W), k!(Z), k!(Tab), k!(B), k!(L), k!(D), k!(W), k!(Z)]
        ],
        [
            [k!(Tab), k!(B), k!(L), k!(D), k!(W), k!(Z), k!(Tab), k!(B), k!(L), k!(D), k!(W), k!(Z)], 
            [k!(Tab), k!(B), k!(L), k!(D), k!(W), k!(Z), k!(Tab), k!(B), k!(L), k!(D), k!(W), k!(Z)], 
            [k!(Tab), k!(B), k!(L), k!(D), k!(W), k!(Z), k!(Tab), k!(B), k!(L), k!(D), k!(W), k!(Z)], 
            [k!(Tab), k!(B), k!(L), k!(D), k!(W), k!(Z), k!(Tab), k!(B), k!(L), k!(D), k!(W), k!(Z)], 
            [k!(Tab), k!(B), k!(L), k!(D), k!(W), k!(Z), k!(Tab), k!(B), k!(L), k!(D), k!(W), k!(Z)]
        ]

    ]
}
