#![feature(asm, core_intrinsics, lang_items)]
#![no_std]

#![crate_type="staticlib"]

mod runtime;

mod kernel_main;
mod kernel_startup;
mod periph_driver;


#[no_mangle]
pub fn _main() {
    kernel_startup::kernel_startup();
    kernel_main::kernel_main();
    loop{}
}

#[no_mangle]
pub unsafe fn __aeabi_unwind_cpp_pr0() {loop{}}
