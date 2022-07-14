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
