#![no_std]

pub mod result;

use result::Result;

pub use riscv_pac_macros::*;

/// Trait for enums of target-specific exception numbers.
///
/// This trait should be implemented by a peripheral access crate (PAC) on its enum of available
/// exceptions for a specific device. Alternatively, the [`riscv`] crate provides a default
/// implementation for the RISC-V ISA. Each variant must convert to a `u16` of its exception number.
///
/// # Safety
///
/// * This trait must only be implemented on the `riscv` crate or on a PAC of a RISC-V target.
/// * This trait must only be implemented on enums of exceptions.
/// * Each enum variant must represent a distinct value (no duplicates are permitted),
/// * Each enum variant must always return the same value (do not change at runtime).
/// * All the exception numbers must be less than or equal to `MAX_EXCEPTION_NUMBER`.
/// * `MAX_EXCEPTION_NUMBER` must coincide with the highest allowed exception number.
pub unsafe trait ExceptionNumber: Copy {
    /// Highest number assigned to an exception.
    const MAX_EXCEPTION_NUMBER: u16;

    /// Converts an exception to its corresponding number.
    fn number(self) -> u16;

    /// Tries to convert a number to a valid exception.
    /// If the conversion fails, it returns an error with the number back.
    fn from_number(value: u16) -> Result<Self, u16>;
}

/// Trait for enums of target-specific interrupt numbers.
///
/// This trait should be implemented by a peripheral access crate (PAC) on its enum of available
// interrupts for a specific device. Alternatively, the [`riscv`] crate provides a default
/// implementation for the RISC-V ISA. Each variant must convert to a `u16` of its interrupt number.
///
/// # Safety
///
/// * This trait must only be implemented on the `riscv` crate or on a PAC of a RISC-V target.
/// * This trait must only be implemented on enums of interrupts.
/// * Each enum variant must represent a distinct value (no duplicates are permitted),
/// * Each enum variant must always return the same value (do not change at runtime).
/// * All the interrupt numbers must be less than or equal to `MAX_INTERRUPT_NUMBER`.
/// * `MAX_INTERRUPT_NUMBER` must coincide with the highest allowed interrupt number.
pub unsafe trait InterruptNumber: Copy {
    /// Highest number assigned to an interrupt source.
    const MAX_INTERRUPT_NUMBER: u16;

    /// Converts an interrupt source to its corresponding number.
    fn number(self) -> u16;

    /// Tries to convert a number to a valid interrupt source.
    /// If the conversion fails, it returns an error with the number back.
    fn from_number(value: u16) -> Result<Self>;
}

/// Trait for enums of target-specific core interrupt numbers.
///
/// Core interrupts are interrupts are retrieved from the `mcause` CSR.
/// Usually, vectored mode is only available for core interrupts.
/// The `riscv` crate provides a default implementation for the RISC-V ISA.
/// However, a PAC may override the default implementation if the target has a
/// different interrupt numbering scheme (e.g., ESP32C3).
///
/// # Safety
///
/// Each enum variant must represent a valid core interrupt number read from the `mcause` CSR.
pub unsafe trait CoreInterruptNumber: InterruptNumber {}

/// Trait for enums of target-specific external interrupt numbers.
///
/// External interrupts are interrupts caused by external sources (e.g., GPIO, UART, SPI).
/// External interrupts are **not** retrieved from the `mcause` CSR.
/// Instead, RISC-V processors have a single core interrupt for all external interrupts.
/// An additional peripheral (e.g., PLIC) is used to multiplex the external interrupts.
///
/// # Safety
///
/// Each enum variant must represent a valid external interrupt number.
pub unsafe trait ExternalInterruptNumber: InterruptNumber {}

/// Trait for enums of priority levels.
///
/// This trait should be implemented by a peripheral access crate (PAC)
/// on its enum of available priority numbers for a specific device.
/// Each variant must convert to a `u8` of its priority level.
///
/// # Safety
///
/// * This trait must only be implemented on a PAC of a RISC-V target.
/// * This trait must only be implemented on enums of priority levels.
/// * Each enum variant must represent a distinct value (no duplicates are permitted).
/// * Each enum variant must always return the same value (do not change at runtime).
/// * All the priority level numbers must be less than or equal to `MAX_PRIORITY_NUMBER`.
/// * `MAX_PRIORITY_NUMBER` must coincide with the highest allowed priority number.
pub unsafe trait PriorityNumber: Copy {
    /// Number assigned to the highest priority level.
    const MAX_PRIORITY_NUMBER: u8;

    /// Converts a priority level to its corresponding number.
    fn number(self) -> u8;

    /// Tries to convert a number to a valid priority level.
    /// If the conversion fails, it returns an error with the number back.
    fn from_number(value: u8) -> Result<Self>;
}

/// Trait for enums of HART identifiers.
///
/// This trait should be implemented by a peripheral access crate (PAC)
/// on its enum of available HARTs for a specific device.
/// Each variant must convert to a `u16` of its HART ID number.
///
/// # Safety
///
/// * This trait must only be implemented on a PAC of a RISC-V target.
/// * This trait must only be implemented on enums of HART IDs.
/// * Each enum variant must represent a distinct value (no duplicates are permitted),
/// * Each anum variant must always return the same value (do not change at runtime).
/// * All the HART ID numbers must be less than or equal to `MAX_HART_ID_NUMBER`.
/// * `MAX_HART_ID_NUMBER` must coincide with the highest allowed HART ID number.
pub unsafe trait HartIdNumber: Copy {
    /// Highest number assigned to a context.
    const MAX_HART_ID_NUMBER: u16;

    /// Converts a HART ID to its corresponding number.
    fn number(self) -> u16;

    /// Tries to convert a number to a valid HART ID.
    /// If the conversion fails, it returns an error with the number back.
    fn from_number(value: u16) -> Result<Self>;
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Clone, Copy, Debug, Eq, PartialEq, ExceptionNumber)]
    #[repr(u16)]
    enum Exception {
        E1 = 1,
        E3 = 3,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq, InterruptNumber)]
    #[repr(u16)]
    enum Interrupt {
        I1 = 1,
        I2 = 2,
        I4 = 4,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq, PriorityNumber)]
    #[repr(u8)]
    enum Priority {
        P0 = 0,
        P1 = 1,
        P2 = 2,
        P3 = 3,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq, HartIdNumber)]
    #[repr(u16)]
    enum Context {
        C0 = 0,
        C1 = 1,
        C2 = 2,
    }

    unsafe impl ExceptionNumber for Exception {
        const MAX_EXCEPTION_NUMBER: u16 = Self::E3 as u16;

        #[inline]
        fn number(self) -> u16 {
            self as _
        }

        #[inline]
        fn from_number(number: u16) -> Result<Self> {
            if number > Self::MAX_EXCEPTION_NUMBER || number == 0 {
                Err(number)
            } else if number == 1 || number == 3 {
                // SAFETY: valid exception number
                Ok(unsafe { core::mem::transmute(number) })
            } else {
                Err(number)
            }
        }
    }

    unsafe impl InterruptNumber for Interrupt {
        const MAX_INTERRUPT_NUMBER: u16 = Self::I4 as u16;

        #[inline]
        fn number(self) -> u16 {
            self as _
        }

        #[inline]
        fn from_number(number: u16) -> Result<Self> {
            if number > Self::MAX_INTERRUPT_NUMBER || number == 0 {
                Err(number)
            } else {
                // SAFETY: valid interrupt number
                Ok(unsafe { core::mem::transmute(number) })
            }
        }
    }

    unsafe impl PriorityNumber for Priority {
        const MAX_PRIORITY_NUMBER: u8 = Self::P3 as u8;

        #[inline]
        fn number(self) -> u8 {
            self as _
        }

        #[inline]
        fn from_number(number: u8) -> Result<Self, u8> {
            if number > Self::MAX_PRIORITY_NUMBER {
                Err(number)
            } else {
                // SAFETY: valid priority number
                Ok(unsafe { core::mem::transmute(number) })
            }
        }
    }

    unsafe impl HartIdNumber for Context {
        const MAX_HART_ID_NUMBER: u16 = Self::C2 as u16;

        #[inline]
        fn number(self) -> u16 {
            self as _
        }

        #[inline]
        fn from_number(number: u16) -> Result<Self, u16> {
            if number > Self::MAX_HART_ID_NUMBER {
                Err(number)
            } else {
                // SAFETY: valid context number
                Ok(unsafe { core::mem::transmute(number) })
            }
        }
    }

    #[test]
    fn check_exception_enum() {
        assert_eq!(Exception::E1.number(), 1);
        assert_eq!(Exception::E3.number(), 3);

        assert_eq!(Exception::from_number(0), Err(0));
        assert_eq!(Exception::from_number(1), Ok(Exception::E1));
        assert_eq!(Exception::from_number(2), Err(2));
        assert_eq!(Exception::from_number(3), Ok(Exception::E3));
        assert_eq!(Exception::from_number(4), Err(4));
    }

    #[test]
    fn check_interrupt_enum() {
        assert_eq!(Interrupt::I1.number(), 1);
        assert_eq!(Interrupt::I2.number(), 2);
        assert_eq!(Interrupt::I4.number(), 4);

        assert_eq!(Interrupt::from_number(0), Err(0));
        assert_eq!(Interrupt::from_number(1), Ok(Interrupt::I1));
        assert_eq!(Interrupt::from_number(2), Ok(Interrupt::I2));
        assert_eq!(Interrupt::from_number(3), Err(3));
        assert_eq!(Interrupt::from_number(4), Ok(Interrupt::I4));
        assert_eq!(Interrupt::from_number(5), Err(5));
    }

    #[test]
    fn check_priority_enum() {
        assert_eq!(Priority::P0.number(), 0);
        assert_eq!(Priority::P1.number(), 1);
        assert_eq!(Priority::P2.number(), 2);
        assert_eq!(Priority::P3.number(), 3);

        assert_eq!(Priority::from_number(0), Ok(Priority::P0));
        assert_eq!(Priority::from_number(1), Ok(Priority::P1));
        assert_eq!(Priority::from_number(2), Ok(Priority::P2));
        assert_eq!(Priority::from_number(3), Ok(Priority::P3));
        assert_eq!(Priority::from_number(4), Err(4));
    }

    #[test]
    fn check_hart_id_enum() {
        assert_eq!(HartId::H0.number(), 0);
        assert_eq!(HartId::H1.number(), 1);
        assert_eq!(HartId::H2.number(), 2);

        assert_eq!(HartId::from_number(0), Ok(HartId::H0));
        assert_eq!(HartId::from_number(1), Ok(HartId::H1));
        assert_eq!(HartId::from_number(2), Ok(HartId::H2));
        assert_eq!(HartId::from_number(3), Err(3));
    }
}
