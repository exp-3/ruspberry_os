// kernel_main.rs

use ::periph_driver;

pub fn kernel_main() {
    periph_driver::led::led_init();
    loop {
        periph_driver::led::led_on();
        periph_driver::timer::delay_ms(100);
        periph_driver::led::led_off();
        periph_driver::timer::delay_ms(100);
    }
}
