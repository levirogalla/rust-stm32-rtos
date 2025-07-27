use core::arch::asm;
use core::panic::PanicInfo;
use cortex_m_rt::{exception, ExceptionFrame};
use rtt_target::rprintln;

use crate::{
    kernel, registers, state, syscalls,
    utils::ScratchRegisters,
};

#[no_mangle] // make sure the rust compiler does not mangle the name
#[allow(non_snake_case)]
extern "C" fn SVCall_Handler(sp: *const u32) {
    // make as extgern "C" so it can be called from assembly using AAPCS, this is so rust knows to use the c standard for calling functions, that is which registers are callee, caller, etc.
    // if using msp do this, other wise use regular load TODO
    let sf = unsafe { ScratchRegisters::load_at(sp) };

    let svc_number = sf.unwrap().get_svc_number().unwrap();
    match svc_number {
        // 0 is the start scheduler syscall
        0 => {
            syscalls::start_scheduler();
        }
        // 1 is the yield cpu syscall
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

// Scheduler interrupt handler
#[no_mangle]
#[allow(non_snake_case)]
extern "C" fn PendSV_Handler(sp: *const u32) {
    critical_section::with(|cs_token| {
        // Re queue the currently running task
        let mut running_task = state::RUNNING_TASK.borrow(cs_token).take().unwrap(); // unwrap is okay since we should always have a running task at this point
        running_task.stack_ptr = sp as u32;

        state::READY_TASKS[running_task.priority as usize]
            .borrow(cs_token)
            .borrow_mut()
            .enqueue(running_task);

        // Dequeue the next task to run
        let next_task = kernel::get_next_task();
        // rprintln!("Next task to run: {:?}", next_task);
        unsafe {
            registers::set::psp(next_task.stack_ptr as u32);
        } //set the psp so that PendSV can pop the state and use it
        state::RUNNING_TASK
            .borrow(cs_token)
            .replace(Some(next_task));
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
    critical_section::with(|cs_token| {
        let running_task = state::RUNNING_TASK.borrow(cs_token).borrow();

        if running_task.is_none() {
            return;
        }

        SYSTICK_COUNT += 1;
        if SYSTICK_COUNT > running_task.unwrap().timeout {
            SYSTICK_COUNT = 0;
            // Trigger a context switch by setting the PendSV bit
            *registers::scb::ICSR::ADDR |= registers::scb::ICSR::PENDSVSET;
            asm!("isb"); // sync cpu instructions with new state
        }
    });
}

#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    rprintln!("PANIC: {}", _info);
    loop {}
}
