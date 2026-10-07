#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Flag {
    Zero = 0b1000_0000,
    Subtract = 0b0100_0000,
    HalfCarry = 0b0010_0000,
    Carry = 0b0001_0000,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Flags{
    bits: u8
}

impl Flags{
    pub fn is_set(&self, flag: Flag) -> bool {
        let mask = flag as u8;
        (self.bits & mask) != 0
    }

    pub fn set(&mut self, flag: Flag, enabled: bool) {
        let mask = flag as u8;
        if enabled {
            self.bits |= mask;
        } else {
            self.bits &= !mask;
        }
    }

    pub fn bits(&self) -> u8 {
        self.bits
    }

    pub fn load(&mut self, value: u8) {
        self.bits = value & 0b1111_0000;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_with_all_flags_clear() {
        let flags = Flags::default();

        assert_eq!(flags.bits(), 0);
    }

    #[test]
    fn setting_and_clearing_preserves_other_flags() {
        let mut flags = Flags::default();

        flags.set(Flag::Carry, true);
        flags.set(Flag::Zero, true);

        assert_eq!(flags.bits(), 0x90);

        flags.set(Flag::Zero, false);

        assert!(!flags.is_set(Flag::Zero));
        assert!(flags.is_set(Flag::Carry));
        assert_eq!(flags.bits(), 0x10);
    }

    #[test]
    fn loading_discards_the_lower_four_bits() {
        let mut flags = Flags::default();

        flags.load(0xff);

        assert_eq!(flags.bits(), 0xf0);
    }
}
