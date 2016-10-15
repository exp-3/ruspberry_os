// kernel_main.rs

// use periph_driver;
use periph_driver;

pub fn kernel_main() {
    periph_driver::led::init();
    // periph_driver::uart::init();
    loop {
        periph_driver::led::on();
        // let c = periph_driver::uart::read_byte();
        periph_driver::timer::delay_ms(100);
        periph_driver::led::off();
        // periph_driver::uart::write_byte(c);
        periph_driver::timer::delay_ms(100);
    }
}
