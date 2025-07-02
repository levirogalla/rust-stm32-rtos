use core::{iter::Scan, panic::PanicInfo};
use cortex_m_rt::{exception, ExceptionFrame};
use rtt_target::rprintln;
use core::{arch::asm};

use crate::{registers, syscalls, utils::{CalleeRegisters, InterruptCPUState, ScratchRegisters}, state};


#[no_mangle] // make sure the rust compiler does not mangle the name
#[allow(non_snake_case)]
extern "C" fn SVCall_Handler(sp: *const u32) { // make as extgern "C" so it can be called from assembly using AAPCS
    // if using msp do this, other wise use regular load TODO
    let sf = unsafe { ScratchRegisters::load_at(sp) };

    let svc_number = sf.unwrap().get_svc_number().unwrap();
    match svc_number {
        0 => {
            syscalls::start_scheduler();
        }
        1 => {
            unsafe { 
                *registers::mm::ICSR::ADDR |= registers::mm::ICSR::PENDSVSET; // set the PendSV bit to trigger a context switch
                asm!("isb"); // sync cpu instructions with new state, this is because the cpu pre fetches instructions, but how those instructions are executed depending on status/control registers, so if we set a control register we want the cpu to discard the pre fetched instructions and execute the new ones with the new state
            } 
        }
        _ => {
            rprintln!("Unknown SVC number: {}", svc_number);
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
extern "C" fn PendSV_Handler(sp: *const u32) {
    critical_section::with(|cs_token| {
        let mut threads_queue = state::THREADS.borrow(cs_token).borrow_mut();
        // Re queue the currently running task
        let mut running_task = state::RUNNING_TASK.borrow(cs_token).take();
        if let Some(tcb) = &mut running_task {
            // Save the current task's stack pointer
            tcb.stack_ptr = sp as u32;
        }
        let qd1 = *threads_queue.data();
        let qs1 = threads_queue.length();
        threads_queue.enqueue(running_task.unwrap());
        let qs2 = threads_queue.length();
        let qd2 = *threads_queue.data();

        // Dequeue the next task to run
        let next_task = threads_queue.dequeue();
        let qs3 = threads_queue.length();
        let qd3 = *threads_queue.data();
        unsafe { registers::set::psp(next_task.unwrap().stack_ptr as u32); } //set the psp so that PendSV can pop the state and use it
        state::RUNNING_TASK.borrow(cs_token).replace(next_task);
        let x = 0x20015f98;
    });
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

