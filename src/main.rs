#![no_std]
#![no_main]

use core::convert::TryFrom;
use core::{arch::asm, panic::PanicInfo};
use cortex_m_rt::{entry, exception, ExceptionFrame};
use rtt_target::{rprintln, rtt_init_print};
// extern "C" {
//     fn toggle_led() -> u32;
// }

#[inline(never)]
extern "C" fn foo(_r0: u32, _r1: u32, _r2: u32, _r3: u32) -> u32 {
    // this function should put those args on the stack
    let sp: u32;
    let xpsr: u32;
    let pc: u32;
    let lr: u32;
    let _x = 0;

    unsafe {
        asm!(
            "mov r4, sp",
            "mrs r5, xPSR",
            "mov r7, lr",
            "mov r0, #0",
            "mov r1, #1",
            "mov r2, #2",
            "mov r3, #3",
            "mov r12, #12",
            "mov r6, pc",
            "svc #2",
            "mov {0}, r4",
            "mov {1}, r5",
            "mov {2}, r6",
            "mov {3}, r7",
            out(reg) sp,
            out(reg) xpsr,
            out(reg) pc,
            out(reg) lr,

        );
        let icsr = 0xE000ED04 as *mut u32; // Address of SCB_ICSR
        core::ptr::write_volatile(icsr, 1 << 28); // Set PENDSVSET (bit 28)
        rprintln!("In foo: SP: {}, xPSR: {}, PC: {}, LR: {}", sp, xpsr, pc, lr,);
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
            unsafe { asm!("nop") };
        }
        let _x = foo(10, 20, 30, 40);
        rprintln!("\n\n");
    }
}

#[exception]
fn SVCall() {
    let sf = StackFrame::load();
    unsafe {
        rprintln!(
            "StackFrame ({}): {:?}",
            ProgramStatus::load().get_interrupt_program_status(),
            sf.unwrap().get_svc_number()
        )
    };
}

// the same (ish) as
// #[no_mangle]
// #[inline(never)]
// pub extern "C" fn SVCall() {...}

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

#[exception]
unsafe fn PendSV() {
    rprintln!(
        "PendSV: {}",
        ProgramStatus::load().get_interrupt_program_status()
    );
}

#[derive(Debug)]
struct ProgramStatus {
    xpsr: u32,
}

impl ProgramStatus {
    pub unsafe fn load() -> Self {
        let xpsr: u32;
        unsafe {
            asm!("mrs {}, xPSR", out(reg) xpsr);
        };
        ProgramStatus { xpsr }
    }

    pub fn get_interrupt_program_status(&self) -> u8 {
        (self.xpsr & 0x1FF) as u8
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct StackFrame {
    r0: u32,
    r1: u32,
    r2: u32,
    r3: u32,
    r12: u32,
    lr: u32,
    pc: u32,
    xpsr: ProgramStatus,
}

impl StackFrame {
    pub fn load() -> Option<Self> {
        let ps = unsafe { ProgramStatus::load() };
        if ps.get_interrupt_program_status() == 0 {
            return None;
        }

        let psp: *const u32;
        unsafe {
            asm!("mrs {}, psp", out(reg) psp);
            Some(StackFrame {
                r0: *psp.offset(0),
                r1: *psp.offset(1),
                r2: *psp.offset(2),
                r3: *psp.offset(3),
                r12: *psp.offset(4),
                lr: *psp.offset(5),
                pc: *psp.offset(6),
                xpsr: ProgramStatus {
                    xpsr: *psp.offset(7),
                },
            })
        }
    }

    pub fn get_svc_number(&self) -> Option<u8> {
        unsafe {
            match Interrupt::try_from(ProgramStatus::load().get_interrupt_program_status()) {
                Ok(Interrupt::SVC) => Some({
                    let ptr = self.pc as *const u8;
                    *ptr.offset(-2)
                }),
                _ => None,
            }
        }
    }
}

enum Interrupt {
    SVC = 11,
    PendSV = 14,
}

impl TryFrom<u8> for Interrupt {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            11 => Ok(Interrupt::SVC),
            14 => Ok(Interrupt::PendSV),
            _ => Err(()),
        }
    }
}
