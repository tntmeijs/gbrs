use log::error;

use crate::{
    memory::memory::Memory,
    utility::bits::{is_nth_bit_set_u8, set_bit_n_state_u8},
};

use super::opcodes::{
    add_a_a, add_a_b, add_a_c, add_a_d, add_a_d8, add_a_e, add_a_h, add_a_hl, add_a_l, add_hl_bc,
    add_hl_de, add_hl_hl, add_hl_sp, add_sp_s8, and_a, and_b, and_c, and_d, and_d8, and_e, and_h,
    and_hl, and_l, call_a16, call_c_a16, call_nc_a16, call_nz_a16, call_z_a16, ccf, cp_a, cp_b,
    cp_c, cp_d, cp_d8, cp_e, cp_h, cp_hl, cp_l, cpl, dec_a, dec_b, dec_bc, dec_c, dec_d, dec_de,
    dec_e, dec_h, dec_hl, dec_hl_address, dec_l, dec_sp, di, ei, halt, inc_a, inc_b, inc_bc, inc_c,
    inc_d, inc_de, inc_e, inc_h, inc_hl, inc_hl_address, inc_l, inc_sp, jp_a16, jp_c_a16, jp_hl,
    jp_nc_a16, jp_nz_a16, jp_z_a16, jr_c_s8, jr_nc_s8, jr_nz_s8, jr_s8, jr_z_s8, ld_a16_a,
    ld_a16_sp, ld_a8_a, ld_a_a, ld_a_a16, ld_a_a8, ld_a_b, ld_a_bc, ld_a_c, ld_a_d, ld_a_d8,
    ld_a_de, ld_a_e, ld_a_h, ld_a_hl, ld_a_hl_dec, ld_a_hl_inc, ld_a_l, ld_a_port_c, ld_b_a,
    ld_b_b, ld_b_c, ld_b_d, ld_b_d8, ld_b_e, ld_b_h, ld_b_hl, ld_b_l, ld_bc_a, ld_bc_d16, ld_c_a,
    ld_c_b, ld_c_c, ld_c_d, ld_c_d8, ld_c_e, ld_c_h, ld_c_hl, ld_c_l, ld_d_a, ld_d_b, ld_d_c,
    ld_d_d, ld_d_d8, ld_d_e, ld_d_h, ld_d_hl, ld_d_l, ld_de_a, ld_de_d16, ld_e_a, ld_e_b, ld_e_c,
    ld_e_d, ld_e_d8, ld_e_e, ld_e_h, ld_e_hl, ld_e_l, ld_h_a, ld_h_b, ld_h_c, ld_h_d, ld_h_d8,
    ld_h_e, ld_h_h, ld_h_hl, ld_h_l, ld_hl_a, ld_hl_b, ld_hl_c, ld_hl_d, ld_hl_d16, ld_hl_d8,
    ld_hl_dec_a, ld_hl_e, ld_hl_h, ld_hl_inc_a, ld_hl_l, ld_hl_sp_plus_s8, ld_l_a, ld_l_b, ld_l_c,
    ld_l_d, ld_l_d8, ld_l_e, ld_l_h, ld_l_hl, ld_l_l, ld_port_c_a, ld_sp_d16, ld_sp_hl, nop, or_a,
    or_b, or_c, or_d, or_d8, or_e, or_h, or_hl, or_l, pop_af, pop_bc, pop_de, pop_hl, push_af,
    push_bc, push_de, push_hl, ret, ret_c, ret_nc, ret_nz, ret_z, reti, rla, rlca, rra, rrca, scf,
    stop, sub_a, sub_b, sub_c, sub_d, sub_d8, sub_e, sub_h, sub_hl, sub_l, xor_a, xor_b, xor_c,
    xor_d, xor_d8, xor_e, xor_h, xor_hl, xor_l,
};

/// Program execution starts at this address
const PROGRAM_COUNTER_START_ADDRESS: u16 = 0x0100;

/// Represents all special states the CPU can be in
pub enum CpuState {
    /// Stop mode
    Stop,

    /// Halt mode
    Halt,
}

/// Represents all interrupt bit flags
pub enum InterruptFlag {
    /// V-blank interrupt flag
    VBlank(Option<bool>),

    /// LCD STAT interrupt flag
    LcdStat(Option<bool>),

    /// Timer interrupt flag
    Timer(Option<bool>),

    /// Serial interrupt bit
    Serial(Option<bool>),

    /// Joypad interrupt bit
    Joypad(Option<bool>),
}

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

    /// Interrupt master enable flag - used to disable all interrupts
    pub ime: bool,

    /// Zero flag (Z)
    pub zero: bool,

    /// Subtraction flag (N)
    pub negative: bool,

    /// Half carry flag (H)
    pub half_carry: bool,

    /// Carry flag (C)
    pub carry: bool,

    /// CPU state
    pub state: Option<CpuState>,
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
            program_counter: PROGRAM_COUNTER_START_ADDRESS,
            cycle: 0,
            ime: false,
            zero: false,
            negative: false,
            half_carry: false,
            carry: false,
            state: None,
        }
    }

    /// Reset the program counter to the entry point address
    pub fn reset_program_counter(&mut self) {
        self.program_counter = PROGRAM_COUNTER_START_ADDRESS;
    }

    /// Execute the next instruction in memory
    pub fn tick(&mut self, memory: &mut Memory) {
        self.process_opcode(memory);
    }

    /// Update an interrupt bit flag state
    pub fn set_interrupt_flag_state(&mut self, memory: &mut Memory, flag: InterruptFlag) {
        let mut flags = memory.read_byte_at(0xFFFF);

        flags = match flag {
            InterruptFlag::VBlank(state) => set_bit_n_state_u8(flags, 0, state.unwrap_or(false)),
            InterruptFlag::LcdStat(state) => set_bit_n_state_u8(flags, 1, state.unwrap_or(false)),
            InterruptFlag::Timer(state) => set_bit_n_state_u8(flags, 2, state.unwrap_or(false)),
            InterruptFlag::Serial(state) => set_bit_n_state_u8(flags, 3, state.unwrap_or(false)),
            InterruptFlag::Joypad(state) => set_bit_n_state_u8(flags, 4, state.unwrap_or(false)),
        };

        memory.write_byte_at(0xFFFF, flags);
    }

    /// Update an interrupt request bit flag state
    pub fn set_interrupt_request_flag_state(&mut self, memory: &mut Memory, flag: InterruptFlag) {
        let mut flags = memory.read_byte_at(0xFF0F);

        flags = match flag {
            InterruptFlag::VBlank(state) => set_bit_n_state_u8(flags, 0, state.unwrap_or(false)),
            InterruptFlag::LcdStat(state) => set_bit_n_state_u8(flags, 1, state.unwrap_or(false)),
            InterruptFlag::Timer(state) => set_bit_n_state_u8(flags, 2, state.unwrap_or(false)),
            InterruptFlag::Serial(state) => set_bit_n_state_u8(flags, 3, state.unwrap_or(false)),
            InterruptFlag::Joypad(state) => set_bit_n_state_u8(flags, 4, state.unwrap_or(false)),
        };

        memory.write_byte_at(0xFF0F, flags);
    }

    /// Check if an interrupt bit flag has been set
    pub fn is_interrupt_flag_set(&self, memory: &Memory, flag: InterruptFlag) -> bool {
        let flags = memory.read_byte_at(0xFFFF);

        let bit_position = match flag {
            InterruptFlag::VBlank(_) => 0,
            InterruptFlag::LcdStat(_) => 1,
            InterruptFlag::Timer(_) => 2,
            InterruptFlag::Serial(_) => 3,
            InterruptFlag::Joypad(_) => 4,
        };

        is_nth_bit_set_u8(flags, bit_position)
    }

    /// Check if a request interrupt bit flag has been set
    pub fn is_request_interrupt_flag_set(&self, memory: &Memory, flag: InterruptFlag) -> bool {
        let flags = memory.read_byte_at(0xFF0F);

        let bit_position = match flag {
            InterruptFlag::VBlank(_) => 0,
            InterruptFlag::LcdStat(_) => 1,
            InterruptFlag::Timer(_) => 2,
            InterruptFlag::Serial(_) => 3,
            InterruptFlag::Joypad(_) => 4,
        };

        is_nth_bit_set_u8(flags, bit_position)
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
            0x07 => rlca(self),
            0x08 => ld_a16_sp(self, memory),
            0x09 => add_hl_bc(self),
            0x0A => ld_a_bc(self, memory),
            0x0B => dec_bc(self),
            0x0C => inc_c(self),
            0x0D => dec_c(self),
            0x0E => ld_c_d8(self, memory),
            0x0F => rrca(self),
            0x10 => stop(self, memory),
            0x11 => ld_de_d16(self, memory),
            0x12 => ld_de_a(self, memory),
            0x13 => inc_de(self),
            0x14 => inc_d(self),
            0x15 => dec_d(self),
            0x16 => ld_d_d8(self, memory),
            0x17 => rla(self),
            0x18 => jr_s8(self, memory),
            0x19 => add_hl_de(self),
            0x1A => ld_a_de(self, memory),
            0x1B => dec_de(self),
            0x1C => inc_e(self),
            0x1D => dec_e(self),
            0x1E => ld_e_d8(self, memory),
            0x1F => rra(self),
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
            0x76 => halt(self),
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
            0x88 => {
                error!("ADC A, B not implemented");
                self.program_counter += 1;
                self.cycle += 1;
            }
            0x89 => {
                error!("ADC A, C not implemented");
                self.program_counter += 1;
                self.cycle += 1;
            }
            0x8A => {
                error!("ADC A, D not implemented");
                self.program_counter += 1;
                self.cycle += 1;
            }
            0x8B => {
                error!("ADC A, E not implemented");
                self.program_counter += 1;
                self.cycle += 1;
            }
            0x8C => {
                error!("ADC A, H not implemented");
                self.program_counter += 1;
                self.cycle += 1;
            }
            0x8D => {
                error!("ADC A, L not implemented");
                self.program_counter += 1;
                self.cycle += 1;
            }
            0x8E => {
                error!("ADC A, (HL) not implemented");
                self.program_counter += 1;
                self.cycle += 1;
            }
            0x8F => {
                error!("ADC A, A not implemented");
                self.program_counter += 1;
                self.cycle += 1;
            }
            0x90 => sub_b(self),
            0x91 => sub_c(self),
            0x92 => sub_d(self),
            0x93 => sub_e(self),
            0x94 => sub_h(self),
            0x95 => sub_l(self),
            0x96 => sub_hl(self, memory),
            0x97 => sub_a(self),
            0x98 => {
                error!("SBC A, B not implemented");
                self.program_counter += 1;
                self.cycle += 1;
            }
            0x99 => {
                error!("SBC A, C not implemented");
                self.program_counter += 1;
                self.cycle += 1;
            }
            0x9A => {
                error!("SBC A, D not implemented");
                self.program_counter += 1;
                self.cycle += 1;
            }
            0x9B => {
                error!("SBC A, E not implemented");
                self.program_counter += 1;
                self.cycle += 1;
            }
            0x9C => {
                error!("SBC A, H not implemented");
                self.program_counter += 1;
                self.cycle += 1;
            }
            0x9D => {
                error!("SBC A, L not implemented");
                self.program_counter += 1;
                self.cycle += 1;
            }
            0x9E => {
                error!("SBC A, (HL) not implemented");
                self.program_counter += 1;
                self.cycle += 1;
            }
            0x9F => {
                error!("SBC A, A not implemented");
                self.program_counter += 1;
                self.cycle += 1;
            }
            0xA0 => and_b(self),
            0xA1 => and_c(self),
            0xA2 => and_d(self),
            0xA3 => and_e(self),
            0xA4 => and_h(self),
            0xA5 => and_l(self),
            0xA6 => and_hl(self, memory),
            0xA7 => and_a(self),
            0xA8 => xor_b(self),
            0xA9 => xor_c(self),
            0xAA => xor_d(self),
            0xAB => xor_e(self),
            0xAC => xor_h(self),
            0xAD => xor_l(self),
            0xAE => xor_hl(self, memory),
            0xAF => xor_a(self),
            0xB0 => or_b(self),
            0xB1 => or_c(self),
            0xB2 => or_d(self),
            0xB3 => or_e(self),
            0xB4 => or_h(self),
            0xB5 => or_l(self),
            0xB6 => or_hl(self, memory),
            0xB7 => or_a(self),
            0xB8 => cp_b(self),
            0xB9 => cp_c(self),
            0xBA => cp_d(self),
            0xBB => cp_e(self),
            0xBC => cp_h(self),
            0xBD => cp_l(self),
            0xBE => cp_hl(self, memory),
            0xBF => cp_a(self),
            0xC0 => ret_nz(self, memory),
            0xC1 => pop_bc(self, memory),
            0xC2 => jp_nz_a16(self, memory),
            0xC3 => jp_a16(self, memory),
            0xC4 => call_nz_a16(self, memory),
            0xC5 => push_bc(self, memory),
            0xC6 => add_a_d8(self, memory),
            0xC7 => {
                error!("RST 0 not implemented");
                self.program_counter += 1;
                self.cycle += 4;
            }
            0xC8 => ret_z(self, memory),
            0xC9 => ret(self, memory),
            0xCA => jp_z_a16(self, memory),
            0xCB => self.process_16_bit_opcode(memory),
            0xCC => call_z_a16(self, memory),
            0xCD => call_a16(self, memory),
            0xCE => {
                error!("ADC A, d8 not implemented");
                self.program_counter += 2;
                self.cycle += 2;
            }
            0xCF => {
                error!("RST 1 not implemented");
                self.program_counter += 1;
                self.cycle += 4;
            }
            0xD0 => ret_nc(self, memory),
            0xD1 => pop_de(self, memory),
            0xD2 => jp_nc_a16(self, memory),
            0xD4 => call_nc_a16(self, memory),
            0xD5 => push_de(self, memory),
            0xD6 => sub_d8(self, memory),
            0xD7 => {
                error!("RST 2 not implemented");
                self.program_counter += 1;
                self.cycle += 4;
            }
            0xD8 => ret_c(self, memory),
            0xD9 => reti(self, memory),
            0xDA => jp_c_a16(self, memory),
            0xDC => call_c_a16(self, memory),
            0xDE => {
                error!("ADC A, d8 not implemented");
                self.program_counter += 2;
                self.cycle += 2;
            }
            0xDF => {
                error!("RST 1 not implemented");
                self.program_counter += 1;
                self.cycle += 4;
            }
            0xE0 => ld_a8_a(self, memory),
            0xE1 => pop_hl(self, memory),
            0xE2 => ld_port_c_a(self, memory),
            0xE5 => push_hl(self, memory),
            0xE6 => and_d8(self, memory),
            0xE7 => {
                error!("RST 4 not implemented");
                self.program_counter += 1;
                self.cycle += 4;
            }
            0xE8 => add_sp_s8(self, memory),
            0xE9 => jp_hl(self),
            0xEA => ld_a16_a(self, memory),
            0xEE => xor_d8(self, memory),
            0xEF => {
                error!("RST 5 not implemented");
                self.program_counter += 1;
                self.cycle += 4;
            }
            0xF0 => ld_a_a8(self, memory),
            0xF1 => pop_af(self, memory),
            0xF2 => ld_a_port_c(self, memory),
            0xF3 => di(self, memory),
            0xF5 => push_af(self, memory),
            0xF6 => or_d8(self, memory),
            0xF7 => {
                error!("RST 6 not implemented");
                self.program_counter += 1;
                self.cycle += 4;
            }
            0xF8 => ld_hl_sp_plus_s8(self, memory),
            0xF9 => ld_sp_hl(self),
            0xFA => ld_a_a16(self, memory),
            0xFB => ei(self),
            0xFE => cp_d8(self, memory),
            0xFF => {
                error!("RST 7 not implemented");
                self.program_counter += 1;
                self.cycle += 4;
            }
            _ => panic!(
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

impl Default for Cpu {
    /// Create a new Cpu instance with default values
    fn default() -> Self {
        Self::new()
    }
}
