#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use defmt_rtt as _;

//use embedded_hal::{delay::DelayNs, digital::OutputPin};
use microbit::{board::Board, display::blocking::Display, hal::Timer, pac::TIMER0};

#[entry]
fn main() -> ! {
    if let Some(board) = Board::take() {
        let mut display = Display::new(board.display_pins);
        let mut timer = Timer::new(board.TIMER0);

        #[allow(non_snake_case)]
        let letter_R: [[u8; 5]; 5] = [
            [0, 1, 1, 1, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 1, 1, 0],
            [0, 1, 1, 0, 0],
            [0, 1, 0, 1, 0],
        ];

        #[allow(non_snake_case)]
        let letter_U: [[u8; 5]; 5] = [
            [0, 1, 0, 1, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 1, 1, 0],
        ];

        #[allow(non_snake_case)]
        let letter_S: [[u8; 5]; 5] = [
            [0, 0, 0, 0, 0],
            [0, 0, 1, 1, 0],
            [0, 1, 0, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 1, 1, 1, 0],
        ];

        #[allow(non_snake_case)]
        let letter_T: [[u8; 5]; 5] = [
            [0, 1, 1, 1, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
        ];

        loop {
            display.show(&mut timer, letter_R, 1000);
            display.show(&mut timer, letter_U, 1000);
            display.show(&mut timer, letter_S, 1000);
            display.show(&mut timer, letter_T, 1000);
            display.clear();
        }
    
    }

    loop {

    }
}