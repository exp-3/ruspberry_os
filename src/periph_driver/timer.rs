const SYSTIME_BASE: *const u32 = 0x3f003000 as *const u32;
// const systime: *const u32 = SYSTIME_BASE as *const u32;

pub fn get_systime()->u64 {
    let systime_clo = unsafe{ SYSTIME_BASE.offset(1) as *const u32 };
    let systime_chi = unsafe{ SYSTIME_BASE.offset(2) as *const u32 };

    let mut chi = unsafe { *systime_chi as u64 };
    let mut clo = unsafe { *systime_clo as u64 };

    if chi != unsafe { *systime_chi as u64 } {
        chi = unsafe { *systime_chi as u64 };
        clo = unsafe { *systime_clo as u64 };
    }

    (chi << 32 | clo)
}

pub fn delay_ms(delay: u32) {
    let alerm_time = get_systime() + (delay as u64) * 1000;
    while get_systime() < alerm_time {
        unsafe {asm!("");}
    }
    // for _ in 1 .. 500000 {
    //     unsafe {asm!("");}
    // }
}
