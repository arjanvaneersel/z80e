#[derive(Clone, Debug)]
pub(crate) struct Memory{
    ram: [u8; 65536],  // Full 64KB address space
}

impl Memory {
    pub fn new() -> Self {
        Self {
            ram: [0; 65536],
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.ram[address as usize]
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.ram[address as usize] = value;
    }

    pub fn read_word(&self, address: u16) -> u16 {
        let lo = self.read_byte(address) as u16;
        let hi = self.read_byte(address.wrapping_add(1)) as u16;
        (hi << 8) | lo
    }

    pub fn write_word(&mut self, address: u16, value: u16) {
        let lo = (value & 0x00FF) as u8;
        let hi = (value >> 8) as u8;
        self.write_byte(address, lo);
        self.write_byte(address.wrapping_add(1), hi);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
fn test_memory_initialization() {
    let memory = Memory::new();
    
    // All memory should start as zero
    assert_eq!(memory.read_byte(0x0000), 0x00);
    assert_eq!(memory.read_byte(0x8000), 0x00);
    assert_eq!(memory.read_byte(0xFFFF), 0x00);
}

#[test]
fn test_memory_byte_operations() {
    let mut memory = Memory::new();
    
    // Test writing and reading bytes
    memory.write_byte(0x1000, 0x42);
    assert_eq!(memory.read_byte(0x1000), 0x42);
    
    memory.write_byte(0x8000, 0xFF);
    assert_eq!(memory.read_byte(0x8000), 0xFF);
    
    // Test that other locations weren't affected
    assert_eq!(memory.read_byte(0x1001), 0x00);
    assert_eq!(memory.read_byte(0x7FFF), 0x00);
}

#[test]
fn test_memory_word_operations() {
    let mut memory = Memory::new();
    
    // Test 16-bit word operations (little-endian)
    memory.write_word(0x2000, 0x1234);
    assert_eq!(memory.read_word(0x2000), 0x1234);
    
    // Verify the bytes were stored correctly (little-endian)
    assert_eq!(memory.read_byte(0x2000), 0x34); // Low byte first
    assert_eq!(memory.read_byte(0x2001), 0x12); // High byte second
    
    // Test edge case near end of memory
    memory.write_word(0xFFFE, 0xABCD);
    assert_eq!(memory.read_word(0xFFFE), 0xABCD);
    assert_eq!(memory.read_byte(0xFFFE), 0xCD);
    assert_eq!(memory.read_byte(0xFFFF), 0xAB);
}

#[test]
fn test_memory_address_wraparound() {
    let mut memory = Memory::new();
    
    // Test what happens when we try to write a word at the very end
    // This should wrap around (0xFFFF + 1 = 0x0000)
    memory.write_word(0xFFFF, 0x5678);
    
    assert_eq!(memory.read_byte(0xFFFF), 0x78); // Low byte at 0xFFFF
    assert_eq!(memory.read_byte(0x0000), 0x56); // High byte wraps to 0x0000
    assert_eq!(memory.read_word(0xFFFF), 0x5678);
}
}