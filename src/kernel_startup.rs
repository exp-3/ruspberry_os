extern crate core;

use core::intrinsics::volatile_store;

pub fn kernel_startup() {
    extern {
        static __bss_start: u32;
        static __bss_end: u32;
    }

    unsafe {
        for p in __bss_start .. __bss_end {
                volatile_store(p as *mut u32, 0);
        }
    }
}
