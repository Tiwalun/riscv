//! stvec register

/// stvec register
#[derive(Clone, Copy, Debug)]
pub struct Stvec {
    bits: usize,
}

/// Trap mode
pub enum TrapMode {
    Direct = 0,
    Vectored = 1,
}

impl Stvec {
    /// Returns the contents of the register as raw bits
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// Returns the trap-vector base-address
    pub fn address(&self) -> usize {
        self.bits - (self.bits & 0b11)
    }

    /// Returns the trap-vector mode
    pub fn trap_mode(&self) -> TrapMode {
        let mode = self.bits & 0b11;
        match mode {
            0 => TrapMode::Direct,
            1 => TrapMode::Vectored,
            _ => unimplemented!()
        }
    }
}

read_csr_as!(Stvec, 0x105);
write_csr!(0x105);

/// Writes the CSR
#[inline]
#[cfg_attr(not(any(target_arch = "riscv32", target_arch = "riscv64")), allow(unused_variables))]
pub unsafe fn write(addr: usize, mode: TrapMode) {
    _write(addr + mode as usize);
}