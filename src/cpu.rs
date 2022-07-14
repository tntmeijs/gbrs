use log::error;

use crate::{
    memory::Memory,
    opcodes::{
        add_a_a, add_a_b, add_a_c, add_a_d, add_a_e, add_a_h, add_a_hl, add_a_l, add_hl_bc,
        add_hl_de, add_hl_hl, add_hl_sp, call_a16, ccf, cpl, dec_a, dec_b, dec_bc, dec_c, dec_d,
        dec_de, dec_e, dec_h, dec_hl, dec_hl_address, dec_l, dec_sp, di, inc_a, inc_b, inc_bc,
        inc_c, inc_d, inc_de, inc_e, inc_h, inc_hl, inc_hl_address, inc_l, inc_sp, jp_a16, jr_c_s8,
        jr_nc_s8, jr_nz_s8, jr_s8, jr_z_s8, ld_a16_a, ld_a16_sp, ld_a8_a, ld_a_a, ld_a_b, ld_a_bc,
        ld_a_c, ld_a_d, ld_a_d8, ld_a_de, ld_a_e, ld_a_h, ld_a_hl, ld_a_hl_dec, ld_a_hl_inc,
        ld_a_l, ld_b_a, ld_b_b, ld_b_c, ld_b_d, ld_b_d8, ld_b_e, ld_b_h, ld_b_hl, ld_b_l, ld_bc_a,
        ld_bc_d16, ld_c_a, ld_c_b, ld_c_c, ld_c_d, ld_c_d8, ld_c_e, ld_c_h, ld_c_hl, ld_c_l,
        ld_d_a, ld_d_b, ld_d_c, ld_d_d, ld_d_d8, ld_d_e, ld_d_h, ld_d_hl, ld_d_l, ld_de_a,
        ld_de_d16, ld_e_a, ld_e_b, ld_e_c, ld_e_d, ld_e_d8, ld_e_e, ld_e_h, ld_e_hl, ld_e_l,
        ld_h_a, ld_h_b, ld_h_c, ld_h_d, ld_h_d8, ld_h_e, ld_h_h, ld_h_hl, ld_h_l, ld_hl_a, ld_hl_b,
        ld_hl_c, ld_hl_d, ld_hl_d16, ld_hl_d8, ld_hl_dec_a, ld_hl_e, ld_hl_h, ld_hl_inc_a, ld_hl_l,
        ld_l_a, ld_l_b, ld_l_c, ld_l_d, ld_l_d8, ld_l_e, ld_l_h, ld_l_hl, ld_l_l, ld_sp_d16, nop,
        ret, scf,
    },
};

/// Represents the GameBoy's CPU (a Sharp CPU based on Intel's 8080 CPU)
pub struct Cpu {
    /// Register A
    pub a: u8,

    /// Register B
    pub b: u8,

    /// Register C
    pub c: u8,

    /// Register D
    pub d: u8,

    /// Register E
    pub e: u8,

    /// Register H
    pub h: u8,

    /// Register L
    pub l: u8,

    /// Stack pointer
    pub stack_pointer: u16,

    /// Program counter
    pub program_counter: u16,

    /// Current cycle
    pub cycle: u64,

    /// Indicates whether interrupts are allowed
    pub disable_interrupts: bool,

    /// Zero flag (Z)
    pub zero: bool,

    /// Subtraction flag (N)
    pub negative: bool,

    /// Half carry flag (H)
    pub half_carry: bool,

    /// Carry flag (C)
    pub carry: bool,
}

impl Cpu {
    /// Create a new Cpu instance, reset all registers, and set the program counter to the starting
    /// location (0x0100). This emulator will skip the built-in ROM and immediately jumps to the
    /// memory at which the game's logic is located.
    pub fn new() -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            stack_pointer: 0,
            program_counter: 0x0100,
            cycle: 0,
            disable_interrupts: false,
            zero: false,
            negative: false,
            half_carry: false,
            carry: false,
        }
    }

    /// Execute the next instruction in memory
    pub fn tick(&mut self, memory: &mut Memory) {
        self.process_opcode(memory);
    }

    /// Process the current opcode
    fn process_opcode(&mut self, memory: &mut Memory) {
        let opcode = self.read_opcode(memory);

        match opcode {
            0x00 => nop(self),
            0x01 => ld_bc_d16(self, memory),
            0x02 => ld_bc_a(self, memory),
            0x03 => inc_bc(self),
            0x04 => inc_b(self),
            0x05 => dec_b(self),
            0x06 => ld_b_d8(self, memory),
            0x07 => {
                error!("RLCA not implemented");
                self.program_counter += 1;
                self.cycle += 1;
            }
            0x08 => ld_a16_sp(self, memory),
            0x09 => add_hl_bc(self),
            0x0A => ld_a_bc(self, memory),
            0x0B => dec_bc(self),
            0x0C => inc_c(self),
            0x0D => dec_c(self),
            0x0E => ld_c_d8(self, memory),
            0x0F => {
                error!("RRCA not implemented");
                self.program_counter += 1;
                self.cycle += 1;
            }
            0x10 => {
                error!("STOP not implemented");
                self.program_counter += 1;
                self.cycle += 1;
            }
            0x11 => ld_de_d16(self, memory),
            0x12 => ld_de_a(self, memory),
            0x13 => inc_de(self),
            0x14 => inc_d(self),
            0x15 => dec_d(self),
            0x16 => ld_d_d8(self, memory),
            0x17 => {
                error!("RLA not implemented");
                self.program_counter += 1;
                self.cycle += 1;
            }
            0x18 => jr_s8(self, memory),
            0x19 => add_hl_de(self),
            0x1A => ld_a_de(self, memory),
            0x1B => dec_de(self),
            0x1C => inc_e(self),
            0x1D => dec_e(self),
            0x1E => ld_e_d8(self, memory),
            0x1F => {
                error!("RRA not implemented");
                self.program_counter += 1;
                self.cycle += 1;
            }
            0x20 => jr_nz_s8(self, memory),
            0x21 => ld_hl_d16(self, memory),
            0x22 => ld_hl_inc_a(self, memory),
            0x23 => inc_hl(self),
            0x24 => inc_h(self),
            0x25 => dec_h(self),
            0x26 => ld_h_d8(self, memory),
            0x27 => {
                error!("DAA not implemented");
                self.program_counter += 1;
                self.cycle += 1;
            }
            0x28 => jr_z_s8(self, memory),
            0x29 => add_hl_hl(self),
            0x2A => ld_a_hl_inc(self, memory),
            0x2B => dec_hl(self),
            0x2C => inc_l(self),
            0x2D => dec_l(self),
            0x2E => ld_l_d8(self, memory),
            0x2F => cpl(self),
            0x30 => jr_nc_s8(self, memory),
            0x31 => ld_sp_d16(self, memory),
            0x32 => ld_hl_dec_a(self, memory),
            0x33 => inc_sp(self),
            0x34 => inc_hl_address(self, memory),
            0x35 => dec_hl_address(self, memory),
            0x36 => ld_hl_d8(self, memory),
            0x37 => scf(self),
            0x38 => jr_c_s8(self, memory),
            0x39 => add_hl_sp(self),
            0x3A => ld_a_hl_dec(self, memory),
            0x3B => dec_sp(self),
            0x3C => inc_a(self),
            0x3D => dec_a(self),
            0x3E => ld_a_d8(self, memory),
            0x3F => ccf(self),
            0x40 => ld_b_b(self),
            0x41 => ld_b_c(self),
            0x42 => ld_b_d(self),
            0x43 => ld_b_e(self),
            0x44 => ld_b_h(self),
            0x45 => ld_b_l(self),
            0x46 => ld_b_hl(self, memory),
            0x47 => ld_b_a(self),
            0x48 => ld_c_b(self),
            0x49 => ld_c_c(self),
            0x4A => ld_c_d(self),
            0x4B => ld_c_e(self),
            0x4C => ld_c_h(self),
            0x4D => ld_c_l(self),
            0x4E => ld_c_hl(self, memory),
            0x4F => ld_c_a(self),
            0x50 => ld_d_b(self),
            0x51 => ld_d_c(self),
            0x52 => ld_d_d(self),
            0x53 => ld_d_e(self),
            0x54 => ld_d_h(self),
            0x55 => ld_d_l(self),
            0x56 => ld_d_hl(self, memory),
            0x57 => ld_d_a(self),
            0x58 => ld_e_b(self),
            0x59 => ld_e_c(self),
            0x5A => ld_e_d(self),
            0x5B => ld_e_e(self),
            0x5C => ld_e_h(self),
            0x5D => ld_e_l(self),
            0x5E => ld_e_hl(self, memory),
            0x5F => ld_e_a(self),
            0x60 => ld_h_b(self),
            0x61 => ld_h_c(self),
            0x62 => ld_h_d(self),
            0x63 => ld_h_e(self),
            0x64 => ld_h_h(self),
            0x65 => ld_h_l(self),
            0x66 => ld_h_hl(self, memory),
            0x67 => ld_h_a(self),
            0x68 => ld_l_b(self),
            0x69 => ld_l_c(self),
            0x6A => ld_l_d(self),
            0x6B => ld_l_e(self),
            0x6C => ld_l_h(self),
            0x6D => ld_l_l(self),
            0x6E => ld_l_hl(self, memory),
            0x6F => ld_l_a(self),
            0x70 => ld_hl_b(self, memory),
            0x71 => ld_hl_c(self, memory),
            0x72 => ld_hl_d(self, memory),
            0x73 => ld_hl_e(self, memory),
            0x74 => ld_hl_h(self, memory),
            0x75 => ld_hl_l(self, memory),
            0x76 => {
                error!("HALT not implemented");
                self.program_counter += 1;
                self.cycle += 1;
            }
            0x77 => ld_hl_a(self, memory),
            0x78 => ld_a_b(self),
            0x79 => ld_a_c(self),
            0x7A => ld_a_d(self),
            0x7B => ld_a_e(self),
            0x7C => ld_a_h(self),
            0x7D => ld_a_l(self),
            0x7E => ld_a_hl(self, memory),
            0x7F => ld_a_a(self),
            0x80 => add_a_b(self),
            0x81 => add_a_c(self),
            0x82 => add_a_d(self),
            0x83 => add_a_e(self),
            0x84 => add_a_h(self),
            0x85 => add_a_l(self),
            0x86 => add_a_hl(self, memory),
            0x87 => add_a_a(self),
            0x88 => {}
            0x89 => {}
            0x8A => {}
            0x8B => {}
            0x8C => {}
            0x8D => {}
            0x8E => {}
            0x8F => {}
            0x90 => {}
            0x91 => {}
            0x92 => {}
            0x93 => {}
            0x94 => {}
            0x95 => {}
            0x96 => {}
            0x97 => {}
            0x98 => {}
            0x99 => {}
            0x9A => {}
            0x9B => {}
            0x9C => {}
            0x9D => {}
            0x9E => {}
            0x9F => {}
            0xA0 => {}
            0xA1 => {}
            0xA2 => {}
            0xA3 => {}
            0xA4 => {}
            0xA5 => {}
            0xA6 => {}
            0xA7 => {}
            0xA8 => {}
            0xA9 => {}
            0xAA => {}
            0xAB => {}
            0xAC => {}
            0xAD => {}
            0xAE => {}
            0xAF => {}
            0xB0 => {}
            0xB1 => {}
            0xB2 => {}
            0xB3 => {}
            0xB4 => {}
            0xB5 => {}
            0xB6 => {}
            0xB7 => {}
            0xB8 => {}
            0xB9 => {}
            0xBA => {}
            0xBB => {}
            0xBC => {}
            0xBD => {}
            0xBE => {}
            0xBF => {}
            0xC0 => {}
            0xC1 => {}
            0xC2 => {}
            0xC3 => jp_a16(self, memory),
            0xC4 => {}
            0xC5 => {}
            0xC6 => {}
            0xC7 => {}
            0xC8 => {}
            0xC9 => ret(self, memory),
            0xCA => {}
            0xCB => self.process_16_bit_opcode(memory),
            0xCC => {}
            0xCD => call_a16(self, memory),
            0xCE => {}
            0xCF => {}
            0xD0 => {}
            0xD1 => {}
            0xD2 => {}
            0xD4 => {}
            0xD5 => {}
            0xD6 => {}
            0xD7 => {}
            0xD8 => {}
            0xD9 => {}
            0xDA => {}
            0xDC => {}
            0xDE => {}
            0xDF => {}
            0xE0 => ld_a8_a(self, memory),
            0xE1 => {}
            0xE2 => {}
            0xE5 => {}
            0xE6 => {}
            0xE7 => {}
            0xE8 => {}
            0xE9 => {}
            0xEA => ld_a16_a(self, memory),
            0xEE => {}
            0xEF => {}
            0xF0 => {}
            0xF1 => {}
            0xF2 => {}
            0xF3 => di(self),
            0xF5 => {}
            0xF6 => {}
            0xF7 => {}
            0xF8 => {}
            0xF9 => {}
            0xFA => {}
            0xFB => {}
            0xFE => {}
            0xFF => {}
            _ => error!(
                "Unknown opcode \"{:#04X}\" at address \"{:#06X}\"",
                opcode, self.program_counter
            ),
        };
    }

    fn process_16_bit_opcode(&mut self, memory: &mut Memory) {
        error!(
            "{:#06X}: 16-bit opcode parsing has not been implemented yet!",
            memory.read_byte_at(self.program_counter)
        );
    }

    /// Read an opcode from the location in memory to which the program counter points
    fn read_opcode(&self, memory: &mut Memory) -> u8 {
        memory.read_byte_at(self.program_counter)
    }
}
