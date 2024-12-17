#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    gpio::{Input, Io, Level, Output, Pull},
    prelude::*,
};

#[entry]
fn main() -> ! {
    #[allow(unused)]
    let peripherals = esp_hal::init(esp_hal::Config::default());

    esp_println::logger::init_logger_from_env();

    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led = Output::new(io.pins.gpio4, Level::Low);
    let button = Input::new(io.pins.gpio21, Pull::Up);

    let mut last_button_state = button.is_high();
    let mut led_state = false;

    loop {
        let current_button_state = button.is_high();
        if current_button_state && !last_button_state {
            led_state = !led_state;
            if led_state {
                led.set_high();
            } else {
                led.set_low();
            }
        }
        last_button_state = current_button_state;
    }
}
