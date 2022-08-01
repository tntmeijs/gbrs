/// Utility function to help convert u16 into two u8
pub fn u16_to_lsb_msb(value: u16) -> (u8, u8) {
    let lsb = (0x00FF & value) as u8;
    let msb = ((0xFF00 & value) >> 8) as u8;

    (lsb, msb)
}

/// Utility function to help convert two u8 into u16
pub fn lsb_msb_to_u16(lsb: u8, msb: u8) -> u16 {
    (u16::from(msb) << 8) | u16::from(lsb)
}

/// Utility function to help check if bit N has been set
pub fn is_nth_bit_set_u8(byte: u8, position: u8) -> bool {
    (1 << position) & byte != 0
}

/// Utility function to help check if bit N has not been set
pub fn is_nth_bit_unset_u8(byte: u8, position: u8) -> bool {
    (1 << position) & byte == 0
}

/// Utility function to help check if bit N has been set
pub fn is_nth_bit_set_u16(word: u16, position: u16) -> bool {
    (1 << position) & word != 0
}

/// Utility function to help check if bit N has not been set
pub fn is_nth_bit_unset_u16(word: u16, position: u16) -> bool {
    (1 << position) & word == 0
}

/// Utility function to help (un)set a specific bit
pub fn set_bit_n_state_u8(byte: u8, position: u8, state: bool) -> u8 {
    if state {
        byte | (1 << position)
    } else {
        byte & !(1 << position)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u16_to_lsb_msb_expect_success() {
        let (lsb, msb) = u16_to_lsb_msb(0xABDE);

        assert_eq!(lsb, 0b1101_1110);
        assert_eq!(msb, 0b1010_1011);
    }

    #[test]
    fn lsb_msb_to_u16_expect_success() {
        assert_eq!(lsb_msb_to_u16(0b1101_1110, 0b1010_1011), 0xABDE);
    }

    #[test]
    fn is_nth_bit_set_u8_test_bit_2_3_4_5_is_set() {
        let byte = 0b0011_1100;

        // These bits have been set and should therefore return true
        assert!(is_nth_bit_set_u8(byte, 2));
        assert!(is_nth_bit_set_u8(byte, 3));
        assert!(is_nth_bit_set_u8(byte, 4));
        assert!(is_nth_bit_set_u8(byte, 5));

        // These bits have not been set and should therefore return false
        assert!(!is_nth_bit_set_u8(byte, 0));
        assert!(!is_nth_bit_set_u8(byte, 1));
        assert!(!is_nth_bit_set_u8(byte, 6));
        assert!(!is_nth_bit_set_u8(byte, 7));
    }

    #[test]
    fn is_nth_bit_unset_u8_test_bit_2_3_4_5_is_set() {
        let byte = 0b0011_1100;

        // These bits have not been set and should therefore return true
        assert!(is_nth_bit_unset_u8(byte, 0));
        assert!(is_nth_bit_unset_u8(byte, 1));
        assert!(is_nth_bit_unset_u8(byte, 6));
        assert!(is_nth_bit_unset_u8(byte, 7));

        // These bits have been set and should therefore return false
        assert!(!is_nth_bit_unset_u8(byte, 2));
        assert!(!is_nth_bit_unset_u8(byte, 3));
        assert!(!is_nth_bit_unset_u8(byte, 4));
        assert!(!is_nth_bit_unset_u8(byte, 5));
    }

    #[test]
    fn is_nth_bit_set_u16_test_bit_2_3_4_5_is_set() {
        let word = 0b0011_1100_0000_0000;

        // These bits have been set and should therefore return true
        assert!(is_nth_bit_set_u16(word, 10));
        assert!(is_nth_bit_set_u16(word, 11));
        assert!(is_nth_bit_set_u16(word, 12));
        assert!(is_nth_bit_set_u16(word, 13));

        // These bits have not been set and should therefore return false
        assert!(!is_nth_bit_set_u16(word, 8));
        assert!(!is_nth_bit_set_u16(word, 9));
        assert!(!is_nth_bit_set_u16(word, 14));
        assert!(!is_nth_bit_set_u16(word, 15));
    }

    #[test]
    fn is_nth_bit_unset_u16_test_bit_2_3_4_5_is_set() {
        let word = 0b0011_1100_0000_0000;

        // These bits have not been set and should therefore return true
        assert!(is_nth_bit_unset_u16(word, 8));
        assert!(is_nth_bit_unset_u16(word, 9));
        assert!(is_nth_bit_unset_u16(word, 14));
        assert!(is_nth_bit_unset_u16(word, 15));

        // These bits have been set and should therefore return false
        assert!(!is_nth_bit_unset_u16(word, 10));
        assert!(!is_nth_bit_unset_u16(word, 11));
        assert!(!is_nth_bit_unset_u16(word, 12));
        assert!(!is_nth_bit_unset_u16(word, 13));
    }

    #[test]
    fn set_bit_n_state_u8_set_bit_3_4() {
        let byte = 0b0000_0000;

        assert_eq!(set_bit_n_state_u8(byte, 3, true), 8);
        assert_eq!(set_bit_n_state_u8(byte, 4, true), 16);
    }

    #[test]
    fn set_bit_n_state_u8_unset_bit_3_4() {
        let byte = 0b0001_1000;

        assert_eq!(set_bit_n_state_u8(byte, 3, false), 16);
        assert_eq!(set_bit_n_state_u8(byte, 4, false), 8);
    }
}
