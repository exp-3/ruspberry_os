extern crate core;

use core::intrinsics::volatile_store;

pub fn kernel_startup() {
    extern {
        static __bss_start: *const u32;
        static __bss_end: *const u32;
    }
    let mut p = unsafe{__bss_start as *mut u32};
    while {p as *const u32} < unsafe{__bss_end} {
        unsafe {
            volatile_store(p, 0);
            p = p.offset(1);
        }
    }
}
