use core::panic::PanicInfo;
use cortex_m_rt::{exception, ExceptionFrame};
use rtt_target::rprintln;
use core::{arch::asm};

use crate::syscalls;
use super::utils::{InterruptContext, ProgramStatus};



#[no_mangle]
fn SVCall_Handler(sp: *const u32) {
    // if using msp do this, other wise use regular load TODO
    let sf = unsafe { InterruptContext::load_at(sp) };

    let svc_number = sf.unwrap().get_svc_number().unwrap();
    match svc_number {
        0 => {
            syscalls::start_scheduler();
        }
        1 => {
            syscalls::hello_world();
        }
        _ => {
            rprintln!("Unknown SVC number: {}", svc_number);
        }
    }
}



#[exception]
#[allow(non_snake_case)]
unsafe fn PendSV() {
    rprintln!(
        "PendSV: {}",
        ProgramStatus::load().get_interrupt_program_status()
    );
}

#[exception]
unsafe fn HardFault(_sf: &ExceptionFrame) -> ! {
    rprintln!("HardFault: {:?}", _sf);
    // get psp, sp, and msp and rprint them
    let psp: u32;
    let msp: u32;
    let sp: u32;
    asm!(
        "mrs {0}, psp",
        "mrs {1}, msp",
        "mov {2}, sp",
        out(reg) psp,
        out(reg) msp,
        out(reg) sp
    );
    rprintln!("PSP: {:#010X}, MSP: {:#010X}, SP: {:#010X}", psp, msp, sp);
    loop {}
}

#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    rprintln!("PANIC: {}", _info);
    loop {}
}

