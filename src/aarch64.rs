use crate::uw;

/// Register context from which to capture a backtrace.
#[derive(Copy, Clone, Debug)]
pub struct Context {
    /// Program counter
    pub pc: u64,

    /// Stack pointer
    pub sp: u64,

    /// General-purpose registers (excluding the stack pointer)
    pub regs: [u64; 31],

    /// FP/SIMD registers
    pub vregs: [u128; 32],
}

impl Context {
    pub(crate) unsafe fn apply(&self, cursor: *mut uw::unw_cursor_t) {
        uw::unw_set_reg(cursor, uw::UNW_REG_IP, self.pc as usize);
        uw::unw_set_reg(cursor, uw::UNW_REG_SP, self.sp as usize);
        for i in 0..31 {
            uw::unw_set_reg(
                cursor,
                (uw::UNW_ARM64_X0 + i) as i32,
                self.regs[i as usize] as usize,
            );
        }
        for i in 0..32 {
            // libunwind doesn't track the upper bits of SIMD registers
            let fval = f64::from_bits(self.vregs[i as usize] as u64);
            uw::unw_set_fpreg(cursor, (uw::UNW_ARM64_D0 + i) as i32, fval);
        }
    }

    pub(crate) fn ip(&self) -> usize {
        self.pc as usize
    }
}
