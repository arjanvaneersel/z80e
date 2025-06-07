/// Z80 CPU Emulator
/// 
/// ## Register Set
/// - Main registers: A, F, B, C, D, E, H, L (can be paired as AF, BC, DE, HL)
/// - Shadow registers: A', F', B', C', D', E', H', L' 
/// - Index registers: IX, IY
/// - System: SP, PC, I, R
/// 
/// ## 16-bit registers
/// The original Z80 combined two 8 byte registers to "mimic" 16 byte registers,
/// but we will use u16 to make our life a bit easier.
///
/// ## System registers
///
/// ### I register
/// Purpose: Forms the high byte of the interrupt vector table address in Interrupt Mode 2.
/// Usage: In IM 2, when an interrupt occurs, the Z80 reads a byte from the interrupting device, combines it with the I register to form a 16-bit address, and jumps to that address
/// Example: If I=0x80 and the device provides 0x34, the CPU jumps to address 0x8034
/// CP/M relevance: CP/M uses this for sophisticated interrupt handling
///
/// ### R register
/// Usage: Increments after each instruction fetch to provide addresses for DRAM refresh
/// Modern relevance: Not needed for emulation accuracy, but some programs read it for timing/randomness
/// Note: Some copy-protection schemes check this register!
///
/// ## Interrupt state
/// 
/// ### Interrupt flip-flops: Control whether interrupts are enabled or disabled.
/// 
/// #### IFF1: Primary interrupt enable flag. 
/// - true = interrupts enabled (EI instruction)
/// - false = interrupts disabled (DI instruction)
///
/// #### IFF2: Backup of IFF1 value
/// - Stores IFF1's state during non-maskable interrupts (NMI)
/// - When NMI occurs: IFF2 = IFF1, then IFF1 = false
/// - When returning from NMI (RETN): IFF1 = IFF2
/// ### IM (Interrupt Mode)
/// Purpose: Determines how the CPU responds to maskable interrupts.
///
/// #### IM 0: 8080-compatible mode
/// - Interrupting device places instruction on data bus
/// - CPU executes that instruction (usually RST)
///
/// #### IM 1: Simple mode
/// - CPU automatically executes RST 38h (jump to 0x0038)
/// - Most common mode in simple systems
///
/// #### IM 2: Vectored mode
/// - Uses I register + device-provided byte for indirect jump
/// - Most sophisticated, used in complex systems

/// Z80 CPU state containing all registers and flags
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Z80 {
    // Main 8-bit registers
    pub(crate) a: u8, pub(crate) f: u8,  // Accumulator and Flags
    pub(crate) b: u8, pub(crate) c: u8,  // BC register pair
    pub(crate) d: u8, pub(crate) e: u8,  // DE register pair  
    pub(crate) h: u8, pub(crate) l: u8,  // HL register pair

    // Alternative register set (shadow registers)
    pub(crate) a_prime: u8, pub(crate) f_prime: u8,
    pub(crate) b_prime: u8, pub(crate) c_prime: u8,
    pub(crate) d_prime: u8, pub(crate) e_prime: u8,
    pub(crate) h_prime: u8, pub(crate) l_prime: u8,

    // True 16-bit registers
    pub(crate) ix: u16,  // Index register X
    pub(crate) iy: u16,  // Index register Y
    pub(crate) sp: u16,  // Stack pointer
    pub(crate) pc: u16,  // Program counter

    // System registers
    pub(crate) i: u8,    // Interrupt vector high byte
    pub(crate) r: u8,    // Memory refresh register

    // Interrupt control
    pub(crate) iff1: bool,  // Interrupt enable flag
    pub(crate) iff2: bool,  // Interrupt backup flag
    pub(crate) im: u8,      // Interrupt mode (0, 1, or 2)
}

impl Z80 {
    /// Create a new Z80 CPU in reset state
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_z80_initialization() {
        let cpu = Z80::new();
        
        // Test main register set initialization
        assert_eq!(cpu.a, 0x00);
        assert_eq!(cpu.f, 0x00);
        assert_eq!(cpu.b, 0x00);
        assert_eq!(cpu.c, 0x00);
        assert_eq!(cpu.d, 0x00);
        assert_eq!(cpu.e, 0x00);
        assert_eq!(cpu.h, 0x00);
        assert_eq!(cpu.l, 0x00);
        
        // Test alternative register set initialization
        assert_eq!(cpu.a_prime, 0x00);
        assert_eq!(cpu.f_prime, 0x00);
        assert_eq!(cpu.b_prime, 0x00);
        assert_eq!(cpu.c_prime, 0x00);
        assert_eq!(cpu.d_prime, 0x00);
        assert_eq!(cpu.e_prime, 0x00);
        assert_eq!(cpu.h_prime, 0x00);
        assert_eq!(cpu.l_prime, 0x00);
        
        // Test 16-bit registers
        assert_eq!(cpu.ix, 0x0000);
        assert_eq!(cpu.iy, 0x0000);
        assert_eq!(cpu.sp, 0x0000);
        assert_eq!(cpu.pc, 0x0000);
        
        // Test system registers
        assert_eq!(cpu.i, 0x00);
        assert_eq!(cpu.r, 0x00);
        
        // Test interrupt state
        assert_eq!(cpu.iff1, false);
        assert_eq!(cpu.iff2, false);
        assert_eq!(cpu.im, 0);
    }
}