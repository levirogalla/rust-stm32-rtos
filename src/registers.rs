pub mod get {
    #![allow(unused)]
    use core::arch::asm;

    /// Returns the value of the Program Counter (PC)
    #[inline]
    pub fn pc() -> u32 {
        let pc: u32;
        unsafe {
            asm!("mov {}, pc", out(reg) pc);
        }
        pc
    }

    /// Returns the value of the Stack Pointer (SP)
    #[inline]
    pub fn sp() -> u32 {
        let sp: u32;
        unsafe {
            asm!("mov {}, sp", out(reg) sp);
        }
        sp
    }

    /// Returns the value of the Link Register (LR)
    #[inline]
    pub fn lr() -> u32 {
        let lr: u32;
        unsafe {
            asm!("mov {}, lr", out(reg) lr);
        }
        lr
    }

    /// Returns the value of the Program Status Register (xPSR)
    #[inline]
    pub fn xpsr() -> u32 {
        let xpsr: u32;
        unsafe {
            asm!("mrs {}, xpsr", out(reg) xpsr);
        }
        xpsr
    }

    /// Returns the value of the CONTROL register
    #[inline]
    pub fn control() -> u32 {
        let control: u32;
        unsafe {
            asm!("mrs {}, control", out(reg) control);
        }
        control
    }

    /// Returns the value of the PRIMASK register
    #[inline]
    pub fn primask() -> u32 {
        let primask: u32;
        unsafe {
            asm!("mrs {}, primask", out(reg) primask);
        }
        primask
    }

    /// Returns the value of the BASEPRI register
    #[inline]
    pub fn basepri() -> u32 {
        let basepri: u32;
        unsafe {
            asm!("mrs {}, basepri", out(reg) basepri);
        }
        basepri
    }

    /// Returns the value of the FAULTMASK register
    #[inline]
    pub fn faultmask() -> u32 {
        let faultmask: u32;
        unsafe {
            asm!("mrs {}, faultmask", out(reg) faultmask);
        }
        faultmask
    }

    /// Returns the value of the r0 register
    #[inline]
    pub fn r0() -> u32 {
        let r0: u32;
        unsafe {
            asm!("mov {}, r0", out(reg) r0);
        }
        r0
    }

    /// Returns the value of the r1 register
    #[inline]
    pub fn r1() -> u32 {
        let r1: u32;
        unsafe {
            asm!("mov {}, r1", out(reg) r1);
        }
        r1
    }

    /// Returns the value of the r2 register
    #[inline]
    pub fn r2() -> u32 {
        let r2: u32;
        unsafe {
            asm!("mov {}, r2", out(reg) r2);
        }
        r2
    }

    /// Returns the value of the r3 register
    #[inline]
    pub fn r3() -> u32 {
        let r3: u32;
        unsafe {
            asm!("mov {}, r3", out(reg) r3);
        }
        r3
    }

    /// Returns the value of the r4 register
    #[inline]
    pub fn r4() -> u32 {
        let r4: u32;
        unsafe {
            asm!("mov {}, r4", out(reg) r4);
        }
        r4
    }

    /// Returns the value of the r5 register
    #[inline]
    pub fn r5() -> u32 {
        let r5: u32;
        unsafe {
            asm!("mov {}, r5", out(reg) r5);
        }
        r5
    }

    /// Returns the value of the r6 register
    #[inline]
    pub fn r6() -> u32 {
        let r6: u32;
        unsafe {
            asm!("mov {}, r6", out(reg) r6);
        }
        r6
    }

    /// Returns the value of the r7 register
    #[inline]
    pub fn r7() -> u32 {
        let r7: u32;
        unsafe {
            asm!("mov {}, r7", out(reg) r7);
        }
        r7
    }

    /// Returns the value of the r8 register
    #[inline]
    pub fn r8() -> u32 {
        let r8: u32;
        unsafe {
            asm!("mov {}, r8", out(reg) r8);
        }
        r8
    }

    /// Returns the value of the r9 register
    #[inline]
    pub fn r9() -> u32 {
        let r9: u32;
        unsafe {
            asm!("mov {}, r9", out(reg) r9);
        }
        r9
    }

    /// Returns the value of the r10 register
    #[inline]
    pub fn r10() -> u32 {
        let r10: u32;
        unsafe {
            asm!("mov {}, r10", out(reg) r10);
        }
        r10
    }

    /// Returns the value of the r11 register
    #[inline]
    pub fn r11() -> u32 {
        let r11: u32;
        unsafe {
            asm!("mov {}, r11", out(reg) r11);
        }
        r11
    }

    /// Returns the value of the r12 register
    #[inline]
    pub fn r12() -> u32 {
        let r12: u32;
        unsafe {
            asm!("mov {}, r12", out(reg) r12);
        }
        r12
    }
}

pub mod set {
    #![allow(unused)]
    use core::arch::asm;

    /// Sets the value of the Program Counter (PC)
    #[inline]
    pub unsafe fn pc(value: u32) {
        asm!("mov pc, {}", in(reg) value);
    }

    /// Sets the value of the Stack Pointer (SP)
    #[inline]
    pub unsafe fn sp(value: u32) {
        asm!("mov sp, {}", in(reg) value);
    }

    /// Sets the value of the Link Register (LR)
    #[inline]
    pub unsafe fn lr(value: u32) {
        asm!("mov lr, {}", in(reg) value);
    }

    /// Sets the value of the CONTROL register
    #[inline]
    pub unsafe fn control(value: u32) {
        asm!("msr control, {}", in(reg) value);
    }

    /// Sets the value of the PRIMASK register
    #[inline]
    pub unsafe fn primask(value: u32) {
        asm!("msr primask, {}", in(reg) value);
    }

    /// Sets the value of the BASEPRI register
    #[inline]
    pub unsafe fn basepri(value: u32) {
        asm!("msr basepri, {}", in(reg) value);
    }

    /// Sets the value of the FAULTMASK register
    #[inline]
    pub unsafe fn faultmask(value: u32) {
        asm!("msr faultmask, {}", in(reg) value);
    }

    /// Sets the value of the Process Stack Pointer (PSP)
    #[inline]
    pub unsafe fn psp(value: u32) {
        asm!("msr psp, {}", in(reg) value);
    }

    /// Sets the value of the Main Stack Pointer (MSP)
    #[inline]
    pub unsafe fn msp(value: u32) {
        asm!("msr msp, {}", in(reg) value);
    }

    /// Sets the value of the r0 register
    #[inline]
    pub unsafe fn r0(value: u32) {
        asm!("mov r0, {}", in(reg) value);
    }

    /// Sets the value of the r1 register
    #[inline]
    pub unsafe fn r1(value: u32) {
        asm!("mov r1, {}", in(reg) value);
    }

    /// Sets the value of the r2 register
    #[inline]
    pub unsafe fn r2(value: u32) {
        asm!("mov r2, {}", in(reg) value);
    }

    /// Sets the value of the r3 register
    #[inline]
    pub unsafe fn r3(value: u32) {
        asm!("mov r3, {}", in(reg) value);
    }

    /// Sets the value of the r4 register
    #[inline]
    pub unsafe fn r4(value: u32) {
        asm!("mov r4, {}", in(reg) value);
    }

    /// Sets the value of the r5 register
    #[inline]
    pub unsafe fn r5(value: u32) {
        asm!("mov r5, {}", in(reg) value);
    }

    /// Sets the value of the r6 register
    #[inline]
    pub unsafe fn r6(value: u32) {
        asm!("mov r6, {}", in(reg) value);
    }

    /// Sets the value of the r7 register
    #[inline]
    pub unsafe fn r7(value: u32) {
        asm!("mov r7, {}", in(reg) value);
    }

    /// Sets the value of the r8 register
    #[inline]
    pub unsafe fn r8(value: u32) {
        asm!("mov r8, {}", in(reg) value);
    }

    /// Sets the value of the r9 register
    #[inline]
    pub unsafe fn r9(value: u32) {
        asm!("mov r9, {}", in(reg) value);
    }

    /// Sets the value of the r10 register
    #[inline]
    pub unsafe fn r10(value: u32) {
        asm!("mov r10, {}", in(reg) value);
    }

    /// Sets the value of the r11 register
    #[inline]
    pub unsafe fn r11(value: u32) {
        asm!("mov r11, {}", in(reg) value);
    }

    /// Sets the value of the r12 register
    #[inline]
    pub unsafe fn r12(value: u32) {
        asm!("mov r12, {}", in(reg) value);
    }
}

// memory mappped registers
pub mod mm {
    pub struct ICSR;

    impl ICSR {
        pub const ADDR: *mut u32 = 0xe000ed04 as *mut u32; // Interrupt Control and State Register (RW)

        // TODO: a lot of these are RO or WO so maybe implement setter, getter functions for convience
        pub const NMIPENDSET: u32 = 1 << 31; // NMI pending set bit
        pub const PENDSVSET: u32 = 1 << 28; // PendSV set bit
        pub const PENDSVCLR: u32 = 1 << 27; // PendSV clear bit
        pub const PENDSTSET: u32 = 1 << 26; // Interrupt pending bit
        pub const PENDSTCLR: u32 = 1 << 25; // Interrupt clear bit
        pub const ISRPENDING: u32 = 1 << 22; // Interrupt pending bit
        pub const VECTPENDING: u32 = 0b111111 << 12; // Vector pending bits
        pub const RETTOBASE: u32 = 1 << 11; // Return to base bit
        pub const VECTACTIVE: u32 = 0b11111111 << 0; // Active vector bits
    }
    

    // TODO: implement the struct version for the rest of these at some point
    pub const ACTLR: *mut u32 = 0xe000e008 as *mut u32; // Auxiliary Control Register (RW)
    pub const CPUID: *const u32 = 0xe000ed00 as *const u32; // CPUID Base Register (RO)
    pub const VTOR: *mut u32 = 0xe000ed08 as *mut u32; // Vector Table Offset Register (RW)
    pub const AIRCR: *mut u32 = 0xe000ed0c as *mut u32; // Application Interrupt and Reset Control Register (RW)
    pub const SCR: *mut u32 = 0xe000ed10 as *mut u32; // System Control Register (RW)
    pub const CCR: *mut u32 = 0xe000ed14 as *mut u32; // Configuration and Control Register (RW)
    pub const SHPR1: *mut u32 = 0xe000ed18 as *mut u32; // System Handler Priority Register 1 (RW)
    pub const SHPR2: *mut u32 = 0xe000ed1c as *mut u32; // System Handler Priority Register 2 (RW)
    pub const SHPR3: *mut u32 = 0xe000ed20 as *mut u32; // System Handler Priority Register 3 (RW)
    pub const SHCRS: *mut u32 = 0xe000ed24 as *mut u32; // System Handler Control and State Register (RW)
    pub const CFSR: *mut u32 = 0xe000ed28 as *mut u32; // Configurable Fault Status Register (RW)
    pub const MMSR: *mut u8 = 0xe000ed28 as *mut u8; // MemManage Fault Status Register (RW)
    pub const BFSR: *mut u8 = 0xe000ed29 as *mut u8; // BusFault Status Register (RW)
    pub const UFSR: *mut u16 = 0xe000ed2a as *mut u16; // UsageFault Status Register (RW)
    pub const HFSR: *mut u32 = 0xe000ed2c as *mut u32; // HardFault Status Register (RW)
    pub const MMAR: *mut u32 = 0xe000ed34 as *mut u32; // MemManage Fault Address Register (RW)
    pub const BFAR: *mut u32 = 0xe000ed38 as *mut u32; // BusFault Address Register (RW)
    pub const AFSR: *mut u32 = 0xe000ed3c as *mut u32; // Auxiliary Fault Status Register (RW)
}
