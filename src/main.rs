#![no_std]
#![no_main]

use core::{arch::asm, panic::PanicInfo};
use cortex_m::{
    asm::{self, nop},
    register::{self, msp, psp},
};
use cortex_m_rt::{entry, exception, ExceptionFrame};
use rtt_target::{rprintln, rtt_init_print};
// extern "C" {
//     fn toggle_led() -> u32;
// }

#[no_mangle]
#[inline(never)]
fn foo(_r0: u32, _r1: u32, _r2: u32, _r3: u32) -> u32 {
    // this function should put those args on the stack
    let mut r0_1: u32;
    let mut r1_1: u32;
    let mut r2_1: u32;
    let mut r3_1: u32;
    let mut r12_1: u32;
    let mut sp_1: u32;
    let mut lr_1: u32;
    let mut pc_1: u32;
    let mut xpsr_1: u32;

    let mut r0_2: u32;
    let mut r1_2: u32;
    let mut r2_2: u32;
    let mut r3_2: u32;
    let mut r12_2: u32;
    let mut sp_2: u32;
    let mut lr_2: u32;
    let mut pc_2: u32;
    let mut xpsr_2: u32;

    let sp: u32;

    rprintln!("In foo");

    unsafe {
        // Inline assembly to push all registers to the stack
        asm!(
            "mov {0}, r0",
            "mov {1}, r1",
            "mov {2}, r2",
            "mov {3}, r3",
            "mov {4}, r12",
            "mov {5}, sp",
            "mov {6}, lr",
            "mov {7}, pc",
            "mrs {8}, xPSR",
            out(reg) r0_1,
            out(reg) r1_1,
            out(reg) r2_1,
            out(reg) r3_1,
            out(reg) r12_1,
            out(reg) sp_1,
            out(reg) lr_1,
            out(reg) pc_1,
            out(reg) xpsr_1,
        );
        asm!(
            "mov {9}, sp",
            "svc 0x02",
            "mov {0}, r0",
            "mov {1}, r1",
            "mov {2}, r2",
            "mov {3}, r3",
            "mov {4}, r12",
            "mov {5}, sp",
            "mov {6}, lr",
            "mov {7}, pc",
            "mrs {8}, xPSR",
            out(reg) r0_2,
            out(reg) r1_2,
            out(reg) r2_2,
            out(reg) r3_2,
            out(reg) r12_2,
            out(reg) sp_2,
            out(reg) lr_2,
            out(reg) pc_2,
            out(reg) xpsr_2,
            out(reg) sp,

        );
        rprintln!("SP: {}", sp);
        rprintln!(
            "Before SVC: R0: {}, R1: {}, R2: {}, R3: {}, R12: {}, SP: {}, LR: {}, PC: {}, xPSR: {}",
            r0_1,
            r1_1,
            r2_1,
            r3_1,
            r12_1,
            sp_1,
            lr_1,
            pc_1,
            xpsr_1
        );
        rprintln!(
            "After SVC:  R0: {}, R1: {}, R2: {}, R3: {}, R12: {}, SP: {}, LR: {}, PC: {}, xPSR: {}",
            r0_2,
            r1_2,
            r2_2,
            r3_2,
            r12_2,
            sp_2,
            lr_2,
            pc_2,
            xpsr_2
        );
    }
    return 1;
}

#[entry]
fn main() -> ! {
    use_psp();
    rtt_init_print!();

    loop {
        // call a system call every so often
        // unsafe { asm!("svc 0x01") };

        for _ in 0..200_000 {
            nop();
        }
        let x = foo(10, 20, 30, 40);
        rprintln!("\n\n")
    }
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn SVCall() {
    let psp: u32;
    let offset1;
    let offset2;
    let offset3;
    let offset4;
    let offset5;
    let offset6;
    let offset7;
    let offset8;
    let offset9;

    unsafe {
        asm!("mrs {}, psp", out(reg) psp);
        offset1 = *(psp as *const u32).offset(0);
        offset2 = *(psp as *const u32).offset(1);
        offset3 = *(psp as *const u32).offset(2);
        offset4 = *(psp as *const u32).offset(3);
        offset5 = *(psp as *const u32).offset(4);
        offset6 = *(psp as *const u32).offset(5);
        offset7 = *(psp as *const u32).offset(6);
        offset8 = *(psp as *const u32).offset(7);
        offset9 = *(psp as *const u32).offset(8);
    }
    rprintln!("PSP: {}", psp);
    rprintln!(
        "Values: 1: {}, 2: {}, 3: {}, 4: {}, 5: {}, 6: {}, 7: {}, 8: {}, 9: {}",
        offset1,
        offset2,
        offset3,
        offset4,
        offset5,
        offset6,
        offset7,
        offset8,
        offset9
    );
}

fn use_psp() {
    unsafe {
        let psp_stack: u32 = 0x20004000; // Example PSP stack address
        core::arch::asm!(
            "msr PSP, {}",       // Set Process Stack Pointer
            "mrs r0, CONTROL",   // Read CONTROL register
            "orr r0, r0, #2",    // Set bit[1] to 1 (PSP active)
            "msr CONTROL, r0",   // Write CONTROL register
            "isb",               // Ensure changes take effect
            in(reg) psp_stack,
            options(nostack)
        );
    }
}

// #[exception]
// fn SVCall() {
//     let sp: u32;

//     let msp2: u32;
//     // let psp: u32 = psp::read();
//     // let svc_number: u8;
//     unsafe {
//         asm!("mov {}, sp", out(reg) sp);
//         asm!("mrs {}, msp", out(reg) msp2);
//         // asm!("mrs {}, psp", out(reg) psp);
//     }

//     let r0 = unsafe { *(msp2 as *const u32).offset(7) }; // R0
//                                                          // rprintln!("SVCall {} with xPSR: {:#010X}, PC: {:#010X}, LR: {:#010X}, R12: {:#010X}, R3: {:#010X}, R2: {:#010X}, R1: {:#010X}, R0: {:#010X}", svc_number, xpsr, pc, lr, r12, r3, r2, r1, r0);
//                                                          // rprintln!(
//                                                          //     "In exception: SP: {}, MSP1: {}, MSP2: {}, R0: {}",
//                                                          //     sp,
//                                                          //     msp1,
//                                                          //     msp2,
//                                                          //     r0
//                                                          // );
// }
#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    rprintln!("PANIC");
    loop {}
}

#[exception]
unsafe fn HardFault(_sf: &ExceptionFrame) -> ! {
    rprintln!("HardFault!");
    loop {}
}
