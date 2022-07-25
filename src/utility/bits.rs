/// Utility function to help convert u16 into two u8
pub fn u16_to_lsb_msb(value: u16) -> (u8, u8) {
    let lsb = (0xFF & value) as u8;
    let msb = ((0xFF00 & value) >> 8) as u8;

    (lsb, msb)
}

/// Utility function to help convert two u8 into u16
pub fn lsb_msb_to_u16(lsb: u8, msb: u8) -> u16 {
    (u16::from(msb) << 8) | u16::from(lsb)
}

/// Utility function to help check if bit N has been set
pub fn is_nth_bit_set_u8(byte: u8, position: u8) -> bool {
    (1 << position) & byte == 1
}

/// Utility function to help check if bit N has not been set
pub fn is_nth_bit_unset_u8(byte: u8, position: u8) -> bool {
    (1 << position) & byte == 0
}

/// Utility function to help check if bit N has been set
pub fn is_nth_bit_set_u16(word: u16, position: u16) -> bool {
    (1 << position) & word == 1
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
