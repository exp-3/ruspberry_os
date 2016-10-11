extern crate core;

use core::intrinsics::volatile_store;

const GPIO_BASE: *const u32 = 0x3f200000 as *const u32;
// const gpio: *const u32 = GPIO_BASE as *const u32;

pub fn led_init() {
    let led_func_sel = unsafe { GPIO_BASE.offset(4) as *mut u32 };
    unsafe { volatile_store(led_func_sel, 1 << 21); }
}

pub fn led_on() {
    let led_set = unsafe { GPIO_BASE.offset(8) as *mut u32 };
    unsafe { volatile_store(led_set, 1 << 15); }
}

pub fn led_off() {
    let led_clear = unsafe { GPIO_BASE.offset(11) as *mut u32 };
    unsafe { volatile_store(led_clear, 1 << 15); }
}
