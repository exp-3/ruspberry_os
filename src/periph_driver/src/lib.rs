#![feature(asm, core_intrinsics, lang_items)]
#![no_std]

pub const PHY_IO_PERIPH_BASE: u32 = 0x3f000000;

pub mod gpio;
pub mod led;
pub mod timer;
pub mod uart;
