// kernel_main.rs

// use periph_driver;
extern crate periph_driver;
use periph_driver::*;

pub fn kernel_main() {
    led::init();
    uart::init();
    loop {
        periph_driver::led::on();
        let c = uart::read_byte();
        led::off();
        uart::write_byte(c);
        uart::write_byte(c);
    }
}
