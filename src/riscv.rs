use crate::uw;

/// Register context from which to capture a backtrace.
#[derive(Copy, Clone, Debug)]
pub struct Context {
    /// Program counter
    pub pc: usize,

    /// General-purpose registers, starting from x1
    pub regs: [usize; 31],

    /// Floating-point registers
    pub fregs: [u64; 32],
}

impl Context {
    pub(crate) unsafe fn apply(&self, cursor: *mut uw::unw_cursor_t) {
        uw::unw_set_reg(cursor, uw::UNW_REG_IP, self.pc);
        for i in 0..31 {
            uw::unw_set_reg(cursor, (uw::UNW_RISCV_X1 + i) as i32, self.regs[i as usize]);
        }
        for i in 0..32 {
            let fval = f64::from_bits(self.fregs[i as usize]);
            uw::unw_set_fpreg(cursor, (uw::UNW_RISCV_F0 + i) as i32, fval);
        }
    }

    pub(crate) fn ip(&self) -> usize {
        self.pc
    }
}
