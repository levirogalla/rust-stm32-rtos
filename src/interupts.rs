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
                *registers::scb::ICSR::ADDR |= registers::scb::ICSR::PENDSVSET; // set the PendSV bit to trigger a context switch
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

        threads_queue.enqueue(running_task.unwrap());

        // Dequeue the next task to run
        let next_task = threads_queue.dequeue();

        unsafe { registers::set::psp(next_task.unwrap().stack_ptr as u32); } //set the psp so that PendSV can pop the state and use it
        state::RUNNING_TASK.borrow(cs_token).replace(next_task);

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


static mut SYSTICK_COUNT: u32 = 0;
#[exception]
unsafe fn SysTick() {
    SYSTICK_COUNT += 1;
    let interval = critical_section::with(|cs_token| {
        let running_task = state::RUNNING_TASK.borrow(cs_token).borrow();
        running_task.unwrap().timeout
            
    });
    if SYSTICK_COUNT > interval {
        SYSTICK_COUNT = 0;
        // Trigger a context switch by setting the PendSV bit
        *registers::scb::ICSR::ADDR |= registers::scb::ICSR::PENDSVSET;
        asm!("isb"); // sync cpu instructions with new state
    }
}


#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    rprintln!("PANIC: {}", _info);
    loop {}
}

