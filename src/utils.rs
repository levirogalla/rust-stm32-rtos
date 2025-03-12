use core::{arch::asm, fmt::Debug, iter::empty, ptr};

use rtt_target::{rprint, rprintln};

pub struct VectorTable;

impl VectorTable {
    const BASE: *const u32 = 0x08000000 as *const u32;

    pub fn get_sp() -> u32 {
        unsafe { *VectorTable::BASE }
    }

    pub fn get_reset() -> u32 {
        unsafe { *VectorTable::BASE.offset(1) }
    }
}

#[derive(Debug)]
pub struct ProgramStatus {
    xpsr: u32,
}

impl ProgramStatus {
    pub fn load() -> Self {
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
pub struct InterruptContext {
    r0: u32,
    r1: u32,
    r2: u32,
    r3: u32,
    r12: u32,
    lr: u32,
    pc: u32,
    xpsr: ProgramStatus,
}

impl InterruptContext {
    pub unsafe fn load_at(sp: *const u32) -> Option<Self> {
        let ps = ProgramStatus::load();
        if ps.get_interrupt_program_status() == 0 {
            return None;
        }
        unsafe {
            Some(InterruptContext {
                r0: *sp.offset(0),
                r1: *sp.offset(1),
                r2: *sp.offset(2),
                r3: *sp.offset(3),
                r12: *sp.offset(4),
                lr: *sp.offset(5),
                pc: *sp.offset(6),
                xpsr: ProgramStatus {
                    xpsr: *sp.offset(7),
                },
            })
        }     
    }


    pub fn load() -> Option<Self> {
        let ps = ProgramStatus::load();
        if ps.get_interrupt_program_status() == 0 {
            return None;
        }

        let psp: *const u32;
        unsafe {
            asm!("mrs {}, psp", out(reg) psp);
            Some(InterruptContext {
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
                    core::ptr::read_volatile(ptr.offset(-2))
                }),
                _ => None,
            }
        }
    }

    pub fn new_fake(pc: u32) -> Self {
        InterruptContext {
            r0: 0,
            r1: 1,
            r2: 2,
            r3: 3,
            r12: 12,
            lr: 0xFFFFFFFD,
            pc: pc,
            xpsr: ProgramStatus { xpsr: 1 << 24 },
        }
    }

    /// Pushes the context to the active stack return new stack pointer
    pub unsafe fn push_to_sp(self, sp: *mut u32) -> u32 {
        ptr::write_volatile(sp.offset(-8), self.r0);
        ptr::write_volatile(sp.offset(-7), self.r1);
        ptr::write_volatile(sp.offset(-6), self.r2);
        ptr::write_volatile(sp.offset(-5), self.r3);
        ptr::write_volatile(sp.offset(-4), self.r12);
        ptr::write_volatile(sp.offset(-3), self.lr);
        ptr::write_volatile(sp.offset(-2), self.pc);
        ptr::write_volatile(sp.offset(-1), self.xpsr.xpsr);

    
        sp.offset(-8) as u32 // Ensure 8-byte alignment
    }
}

pub enum Interrupt {
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

#[derive(Debug)]
pub struct Queue<T, const N: usize> {
    data: [Option<T>; N],
    head: u8,
    tail: u8,
    max_size: u8,
    /// Flag for when the queue is empty only after it was initialized, once an item is added, this will never be true again
    empty: bool,
    size: u8,
}

impl<T: ::core::marker::Copy + ::core::fmt::Debug, const N: usize> Queue<T, N> {
    pub const fn new() -> Self {
        Queue {
            data: [None; N],
            head: 0,
            tail: 0,
            max_size: N as u8,
            empty: true,
            size: 0,
        }
    }

    /// Add item to the queue and return size
    pub fn enqueue(&mut self, item: T) -> Option<u8> {
        let new_head = self.increment_size()?;

        self.data[new_head as usize] = Some(item);

        self.empty = false;
        Some(self.length())
    }

    pub fn dequeue(&mut self) -> Option<T> {
        let tail = self.tail;
        self.decrement_size()?;
        let item = self.data[tail as usize];
        self.data[tail as usize] = None;
        item
    }

    pub fn length(&self) -> u8 {
        self.size
    }

    pub fn data(&self) -> &[Option<T>; N] {
        &self.data
    }

    pub fn peek(&self) -> Option<&T> {
        self.data[self.tail as usize].as_ref()
    }

    /// Increments the head of the queue and returns the new head index
    fn increment_size(&mut self) -> Option<u8> {
        if self.empty {
            self.empty = false;
            self.size += 1;
            Some(0)
        } else if self.length() == self.max_size {
            // case where array is full
            None
        } else if self.head == self.max_size - 1 {
            // case where head reaches the end of the array
            self.head = 0;
            self.size += 1;
            Some(self.head)
        } else {
            // regular case
            self.head += 1;
            self.size += 1;
            Some(self.head)
        }
    }

    /// Decrements the tail of the queue and returns the new tail index
    fn decrement_size(&mut self) -> Option<u8> {
        if self.length() == 0 {
            // case where array is empty
            None
        } else if self.tail == self.max_size - 1 {
            // case where tail reaches the end of the array
            self.tail = 0;
            self.size -= 1;
            Some(self.tail)
        } else {
            // regular case
            self.tail += 1;
            self.size -= 1;
            Some(self.tail)
        }
    }
}

pub struct CalleeRegisters {
    r4: u32,
    r5: u32,
    r6: u32,
    r7: u32,
    r8: u32,
    r9: u32,
    r10: u32,
    r11: u32,
}

impl CalleeRegisters {
    /// Save the current struct to the passed stack pointer and return updated stack pointer
    pub unsafe fn push_to_sp(self, sp: *mut u32) -> u32 {
        ptr::write_volatile(sp.offset(-8), self.r4);
        ptr::write_volatile(sp.offset(-7), self.r5);
        ptr::write_volatile(sp.offset(-6), self.r6);
        ptr::write_volatile(sp.offset(-5), self.r7);
        ptr::write_volatile(sp.offset(-4), self.r8);
        ptr::write_volatile(sp.offset(-3), self.r9);
        ptr::write_volatile(sp.offset(-2), self.r10);
        ptr::write_volatile(sp.offset(-1), self.r11);

        sp.offset(-7) as u32
    }

    pub fn load_from_stack() {
        todo!();
    }

    pub fn load_from_register() {
        todo!();
    }

    pub fn new_fake() -> Self {
        CalleeRegisters {
            r4: 61,
            r5: 62,
            r6: 63,
            r7: 64,
            r8: 65,
            r9: 66,
            r10: 68,
            r11: 69,
        }
    }

}