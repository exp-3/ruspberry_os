use ::gpio;

const PIN: u32 = 47;

pub fn init() {
    gpio::init(PIN, gpio::GPFSEL::OUTPUT);
}

pub fn on() {
    gpio::write(PIN, true);
}

pub fn off() {
    gpio::write(PIN, false);
}
