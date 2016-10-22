#![feature(asm, core_intrinsics, lang_items)]
#![no_std]

#![crate_type="staticlib"]

extern crate periph_driver;

mod runtime;

mod kernel_main;
mod kernel_startup;

#[no_mangle]
pub fn main() {
    kernel_startup::kernel_startup();
    kernel_main::kernel_main();
    loop{}
}

#[no_mangle]
pub unsafe fn __aeabi_unwind_cpp_pr0() {loop{}}
