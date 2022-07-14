use crate::{
    bits::{
        is_nth_bit_set_u16, is_nth_bit_set_u8, lsb_msb_to_u16, set_bit_n_state_u8, u16_to_lsb_msb,
    },
    cpu::{Cpu, CpuState, InterruptFlag},
    memory::Memory,
};

/// Only advances the program counter by 1
pub fn nop(cpu: &mut Cpu) {
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the 2 bytes of immediate data into register pair BC
pub fn ld_bc_d16(cpu: &mut Cpu, memory: &Memory) {
    let value = memory.read_16_bit_value_at(cpu.program_counter + 1);
    let (lsb, msb) = u16_to_lsb_msb(value);

    cpu.b = lsb;
    cpu.c = msb;

    cpu.program_counter += 3;
    cpu.cycle += 3;
}

/// Store the contents of register A in the memory location specified by register pair BC
pub fn ld_bc_a(cpu: &mut Cpu, memory: &mut Memory) {
    memory.write_byte_at(lsb_msb_to_u16(cpu.b, cpu.c), cpu.a);

    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Increment the contents of register pair BC by 1
pub fn inc_bc(cpu: &mut Cpu) {
    let value = lsb_msb_to_u16(cpu.b, cpu.c).wrapping_add(1);
    let (lsb, msb) = u16_to_lsb_msb(value);

    cpu.b = lsb;
    cpu.c = msb;

    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Increment the contents of register B by 1
pub fn inc_b(cpu: &mut Cpu) {
    cpu.half_carry = is_nth_bit_set_u8(cpu.b, 7);
    cpu.b = cpu.b.wrapping_add(1);
    cpu.zero = cpu.b == 0;
    cpu.negative = false;

    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Decrement the contents of register B by 1
pub fn dec_b(cpu: &mut Cpu) {
    cpu.half_carry = is_nth_bit_set_u8(cpu.b, 7);
    cpu.b = cpu.b.wrapping_sub(1);
    cpu.zero = cpu.b == 0;
    cpu.negative = true;

    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the 8-bit immediate operand d8 into register B
pub fn ld_b_d8(cpu: &mut Cpu, memory: &Memory) {
    cpu.b = memory.read_byte_at(cpu.program_counter + 1);

    cpu.program_counter += 2;
    cpu.cycle += 2;
}

/// Rotate the contents of register A to the left. That is, the contents of bit 0 are copied to bit
/// 1, and the previous contents of bit 1 (before the copy operation) are copied to bit 2. The same
/// operation is repeated in sequence for the rest of the register. The contents of bit 7 are
/// placed in both the CY flag and bit 0 of register A.
pub fn rlca(cpu: &mut Cpu) {
    cpu.zero = false;
    cpu.negative = false;
    cpu.half_carry = false;
    cpu.carry = is_nth_bit_set_u8(cpu.a, 7);

    // Rotate all to the left
    cpu.a <<= 1;
    cpu.a = set_bit_n_state_u8(cpu.a, 0, cpu.carry);

    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Store the lower byte of stack pointer SP at the address specified by the 16-bit immediate
/// operand a16, and store the upper byte of SP at address a16 + 1
pub fn ld_a16_sp(cpu: &mut Cpu, memory: &mut Memory) {
    let (lsb, msb) = u16_to_lsb_msb(cpu.stack_pointer);
    let address = memory.read_16_bit_value_at(cpu.program_counter + 1);
    memory.write_byte_at(address, lsb);
    memory.write_byte_at(address + 1, msb);

    cpu.program_counter += 3;
    cpu.cycle += 5;
}

/// Add the contents of register pair BC to the contents of register pair HL, and store the results
/// in register pair HL
pub fn add_hl_bc(cpu: &mut Cpu) {
    let bc = lsb_msb_to_u16(cpu.b, cpu.c);
    let hl = lsb_msb_to_u16(cpu.h, cpu.l);
    let result = bc.wrapping_add(hl);
    let (lsb, msb) = u16_to_lsb_msb(result);

    cpu.half_carry = is_nth_bit_set_u16(hl, 15);
    cpu.carry = is_nth_bit_set_u16(hl, 15);
    cpu.h = lsb;
    cpu.l = msb;
    cpu.negative = false;

    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Load the 8-bit contents of memory specified by register pair BC into register A
pub fn ld_a_bc(cpu: &mut Cpu, memory: &Memory) {
    let address = lsb_msb_to_u16(cpu.b, cpu.c);
    let value = memory.read_byte_at(address);

    cpu.a = value;

    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Decrement the contents of register pair BC by 1
pub fn dec_bc(cpu: &mut Cpu) {
    let value = lsb_msb_to_u16(cpu.b, cpu.c).wrapping_sub(1);
    let (lsb, msb) = u16_to_lsb_msb(value);

    cpu.b = lsb;
    cpu.c = msb;

    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Increment the contents of register C by 1
pub fn inc_c(cpu: &mut Cpu) {
    cpu.half_carry = is_nth_bit_set_u8(cpu.c, 7);
    cpu.c += 1;
    cpu.zero = cpu.c == 0;
    cpu.negative = false;

    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Decrement the contents of register C by 1
pub fn dec_c(cpu: &mut Cpu) {
    cpu.half_carry = is_nth_bit_set_u8(cpu.c, 7);
    cpu.c -= 1;
    cpu.zero = cpu.c == 0;
    cpu.negative = true;

    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the 8-bit immediate operand d8 into register C
pub fn ld_c_d8(cpu: &mut Cpu, memory: &Memory) {
    cpu.c = memory.read_byte_at(cpu.program_counter + 1);

    cpu.program_counter += 2;
    cpu.cycle += 2;
}

/// Rotate the contents of register A to the right. That is, the contents of bit 7 are copied to
/// bit 6, and the previous contents of bit 6 (before the copy) are copied to bit 5. The same
/// operation is repeated in sequence for the rest of the register. The contents of bit 0 are
/// placed in both the CY flag and bit 7 of register A.
pub fn rrca(cpu: &mut Cpu) {
    cpu.zero = false;
    cpu.negative = false;
    cpu.half_carry = false;
    cpu.carry = is_nth_bit_set_u8(cpu.a, 0);

    // Rotate all to the right
    cpu.a >>= 1;
    cpu.a = set_bit_n_state_u8(cpu.a, 7, cpu.carry);

    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Execution of a STOP instruction stops both the system clock and oscillator circuit. STOP mode
/// is entered and the LCD controller also stops. However, the status of the internal RAM register
/// ports remains unchanged.
///
/// STOP mode can be cancelled by a reset signal.
///
/// If the RESET terminal goes LOW in STOP mode, it becomes that of a normal reset status.
pub fn stop(cpu: &mut Cpu, memory: &Memory) {
    let vblank_unset = !cpu.is_interrupt_flag_set(memory, InterruptFlag::VBlank(None));
    let lcd_stat_unset = !cpu.is_interrupt_flag_set(memory, InterruptFlag::LcdStat(None));
    let timer_unset = !cpu.is_interrupt_flag_set(memory, InterruptFlag::Timer(None));
    let serial_unset = !cpu.is_interrupt_flag_set(memory, InterruptFlag::Serial(None));
    let joypad_unset = !cpu.is_interrupt_flag_set(memory, InterruptFlag::Joypad(None));

    if vblank_unset && lcd_stat_unset && timer_unset && serial_unset && joypad_unset {
        cpu.state = Some(CpuState::Stop);
    }

    cpu.program_counter += 2;
    cpu.cycle += 1;
}

/// Load the 2 bytes of immediate data into register pair DE
pub fn ld_de_d16(cpu: &mut Cpu, memory: &Memory) {
    let value = memory.read_16_bit_value_at(cpu.program_counter + 1);
    let (lsb, msb) = u16_to_lsb_msb(value);

    cpu.d = lsb;
    cpu.e = msb;

    cpu.program_counter += 3;
    cpu.cycle += 3;
}

/// Store the contents of register A in the memory location specified by register pair DE
pub fn ld_de_a(cpu: &mut Cpu, memory: &mut Memory) {
    let address = lsb_msb_to_u16(cpu.d, cpu.e);
    memory.write_byte_at(address, cpu.a);

    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Increment the contents of register pair DE by 1
pub fn inc_de(cpu: &mut Cpu) {
    let value = lsb_msb_to_u16(cpu.d, cpu.e).wrapping_add(1);
    let (lsb, msb) = u16_to_lsb_msb(value);

    cpu.d = lsb;
    cpu.e = msb;
    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Increment the contents of register D by 1
pub fn inc_d(cpu: &mut Cpu) {
    cpu.half_carry = is_nth_bit_set_u8(cpu.d, 7);
    cpu.d = cpu.d.wrapping_add(1);
    cpu.zero = cpu.d == 0;
    cpu.negative = false;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Decrement the contents of register D by 1
pub fn dec_d(cpu: &mut Cpu) {
    cpu.half_carry = is_nth_bit_set_u8(cpu.d, 7);
    cpu.d = cpu.d.wrapping_sub(1);
    cpu.zero = cpu.d == 0;
    cpu.negative = true;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the 8-bit immediate operand d8 into register D
pub fn ld_d_d8(cpu: &mut Cpu, memory: &Memory) {
    cpu.d = memory.read_byte_at(cpu.program_counter + 1);
    cpu.program_counter += 2;
    cpu.cycle += 2;
}

/// Rotate the contents of register A to the left, through the carry (CY) flag. That is, the
/// contents of bit 0 are copied to bit 1, and the previous contents of bit 1
/// (before the copy operation) are copied to bit 2. The same operation is repeated in sequence for
/// the rest of the register. The previous contents of the carry flag are copied to bit 0.
pub fn rla(cpu: &mut Cpu) {
    cpu.zero = false;
    cpu.negative = false;
    cpu.half_carry = false;

    let carry = is_nth_bit_set_u8(cpu.a, 7);

    // Rotate all to the left
    cpu.a <<= 1;
    cpu.a = set_bit_n_state_u8(cpu.a, 0, carry);

    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Jump s8 steps from the current address in the program counter (PC). (Jump relative.)
pub fn jr_s8(cpu: &mut Cpu, memory: &Memory) {
    let relative_jump = u16::from(memory.read_byte_at(cpu.program_counter + 1));

    // Jump relative to the current program counter's address
    cpu.program_counter += relative_jump;
    cpu.cycle += 3;
}

/// Add the contents of register pair DE to the contents of register pair HL, and store the results
/// in register pair HL
pub fn add_hl_de(cpu: &mut Cpu) {
    let de = lsb_msb_to_u16(cpu.d, cpu.e);
    let hl = lsb_msb_to_u16(cpu.h, cpu.l);
    let result = de.wrapping_add(hl);
    let (lsb, msb) = u16_to_lsb_msb(result);

    cpu.half_carry = is_nth_bit_set_u16(hl, 15);
    cpu.carry = is_nth_bit_set_u16(hl, 15);
    cpu.h = lsb;
    cpu.l = msb;
    cpu.zero = result == 0;
    cpu.negative = false;
    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Load the 8-bit contents of memory specified by register pair DE into register A
pub fn ld_a_de(cpu: &mut Cpu, memory: &Memory) {
    let address = lsb_msb_to_u16(cpu.d, cpu.e);
    cpu.a = memory.read_byte_at(address);
    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Decrement the contents of register pair DE by 1
pub fn dec_de(cpu: &mut Cpu) {
    let value = lsb_msb_to_u16(cpu.d, cpu.e).wrapping_sub(1);
    let (lsb, msb) = u16_to_lsb_msb(value);

    cpu.d = lsb;
    cpu.e = msb;
    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Increment the contents of register E by 1
pub fn inc_e(cpu: &mut Cpu) {
    cpu.half_carry = is_nth_bit_set_u8(cpu.e, 7);
    cpu.e += 1;
    cpu.zero = cpu.e == 0;
    cpu.negative = false;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Decrement the contents of register E by 1
pub fn dec_e(cpu: &mut Cpu) {
    cpu.half_carry = is_nth_bit_set_u8(cpu.e, 7);
    cpu.e = cpu.e.wrapping_sub(1);
    cpu.zero = cpu.e == 0;
    cpu.negative = true;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the 8-bit immediate operand d8 into register E
pub fn ld_e_d8(cpu: &mut Cpu, memory: &Memory) {
    cpu.e = memory.read_byte_at(cpu.program_counter + 1);
    cpu.program_counter += 2;
    cpu.cycle += 2;
}

/// Rotate the contents of register A to the right, through the carry (CY) flag. That is, the
/// contents of bit 7 are copied to bit 6, and the previous contents of bit 6 (before the copy) are
/// copied to bit 5. The same operation is repeated in sequence for the rest of the register.
/// The previous contents of the carry flag are copied to bit 7.
pub fn rra(cpu: &mut Cpu) {
    cpu.zero = false;
    cpu.negative = false;
    cpu.half_carry = false;

    let carry = is_nth_bit_set_u8(cpu.a, 0);

    // Rotate all to the right
    cpu.a >>= 1;
    cpu.a = set_bit_n_state_u8(cpu.a, 7, carry);

    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// If the Z flag is 0, jump s8 steps from the current address stored in the program counter (PC).
/// If not, the instruction following the current JP instruction is executed (as usual).
pub fn jr_nz_s8(cpu: &mut Cpu, memory: &Memory) {
    if !cpu.zero {
        cpu.program_counter += u16::from(memory.read_byte_at(cpu.program_counter + 1));
        cpu.cycle += 3;
    } else {
        cpu.program_counter += 2;
        cpu.cycle += 2;
    }
}

/// Load the 2 bytes of immediate data into register pair HL
pub fn ld_hl_d16(cpu: &mut Cpu, memory: &Memory) {
    let value = memory.read_16_bit_value_at(cpu.program_counter + 1);
    let (lsb, msb) = u16_to_lsb_msb(value);

    cpu.h = lsb;
    cpu.l = msb;
    cpu.program_counter += 3;
    cpu.cycle += 3;
}

/// Store the contents of register A into the memory location specified by register pair HL, and
/// simultaneously increment the contents of HL
pub fn ld_hl_inc_a(cpu: &mut Cpu, memory: &mut Memory) {
    // Store contents of A
    let address_in_hl = lsb_msb_to_u16(cpu.h, cpu.l);
    memory.write_byte_at(address_in_hl, cpu.a);

    // Increment contents of HL
    let (lsb, msb) = u16_to_lsb_msb(address_in_hl.wrapping_add(1));
    cpu.h = lsb;
    cpu.l = msb;
    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Increment the contents of register pair HL by 1
pub fn inc_hl(cpu: &mut Cpu) {
    let value = lsb_msb_to_u16(cpu.h, cpu.l).wrapping_add(1);
    let (lsb, msb) = u16_to_lsb_msb(value);

    cpu.h = lsb;
    cpu.l = msb;
    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Increment the contents of register H by 1
pub fn inc_h(cpu: &mut Cpu) {
    cpu.half_carry = is_nth_bit_set_u8(cpu.h, 7);
    cpu.h += 1;
    cpu.zero = cpu.h == 0;
    cpu.negative = false;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Decrement the contents of register H by 1
pub fn dec_h(cpu: &mut Cpu) {
    cpu.half_carry = is_nth_bit_set_u8(cpu.h, 7);
    cpu.h -= 1;
    cpu.zero = cpu.h == 0;
    cpu.negative = true;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the 8-bit immediate operand d8 into register H
pub fn ld_h_d8(cpu: &mut Cpu, memory: &Memory) {
    cpu.h = memory.read_byte_at(cpu.program_counter + 1);
    cpu.program_counter += 2;
    cpu.cycle += 2;
}

/// If the Z flag is 1, jump s8 steps from the current address stored in the program counter (PC).
/// If not, the instruction following the current JP instruction is executed (as usual).
pub fn jr_z_s8(cpu: &mut Cpu, memory: &Memory) {
    if cpu.zero {
        cpu.program_counter += u16::from(memory.read_byte_at(cpu.program_counter + 1));
        cpu.cycle += 3;
    } else {
        cpu.program_counter += 2;
        cpu.cycle += 2;
    }
}

/// Add the contents of register pair HL to the contents of register pair HL, and store the results
/// in register pair HL
pub fn add_hl_hl(cpu: &mut Cpu) {
    let hl = lsb_msb_to_u16(cpu.h, cpu.l);
    let result = hl.wrapping_add(hl);
    let (lsb, msb) = u16_to_lsb_msb(result);

    cpu.half_carry = is_nth_bit_set_u16(hl, 15);
    cpu.carry = is_nth_bit_set_u16(hl, 15);
    cpu.h = lsb;
    cpu.l = msb;
    cpu.zero = result == 0;
    cpu.negative = false;
    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Load the contents of memory specified by register pair HL into register A, and simultaneously
/// increment the contents of HL
pub fn ld_a_hl_inc(cpu: &mut Cpu, memory: &mut Memory) {
    // Load contents of memory address
    let address_in_hl = lsb_msb_to_u16(cpu.h, cpu.l);
    cpu.a = memory.read_byte_at(address_in_hl);

    // Increment HL
    let (lsb, msb) = u16_to_lsb_msb(address_in_hl.wrapping_add(1));
    cpu.h = lsb;
    cpu.l = msb;
    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Decrement the contents of register pair HL by 1
pub fn dec_hl(cpu: &mut Cpu) {
    let value = lsb_msb_to_u16(cpu.h, cpu.l).wrapping_sub(1);
    let (lsb, msb) = u16_to_lsb_msb(value);

    cpu.h = lsb;
    cpu.l = msb;
    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Increment the contents of register L by 1
pub fn inc_l(cpu: &mut Cpu) {
    cpu.half_carry = is_nth_bit_set_u8(cpu.l, 7);
    cpu.l += 1;
    cpu.zero = cpu.l == 0;
    cpu.negative = false;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Decrement the contents of register L by 1
pub fn dec_l(cpu: &mut Cpu) {
    cpu.half_carry = is_nth_bit_set_u8(cpu.l, 7);
    cpu.l -= 1;
    cpu.zero = cpu.l == 0;
    cpu.negative = true;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the 8-bit immediate operand d8 into register L
pub fn ld_l_d8(cpu: &mut Cpu, memory: &Memory) {
    cpu.l = memory.read_byte_at(cpu.program_counter + 1);
    cpu.program_counter += 2;
    cpu.cycle += 2;
}

/// Take the one's complement (i.e., flip all bits) of the contents of register A
pub fn cpl(cpu: &mut Cpu) {
    cpu.a = !cpu.a;
}

/// If the CY flag is 0, jump s8 steps from the current address stored in the program counter (PC).
/// If not, the instruction following the current JP instruction is executed (as usual).
pub fn jr_nc_s8(cpu: &mut Cpu, memory: &Memory) {
    if cpu.carry {
        cpu.program_counter += u16::from(memory.read_byte_at(cpu.program_counter + 1));
        cpu.cycle += 3;
    } else {
        cpu.program_counter += 2;
        cpu.cycle += 2;
    }
}

/// Load the 2 bytes of immediate data into register pair SP
pub fn ld_sp_d16(cpu: &mut Cpu, memory: &Memory) {
    cpu.stack_pointer = memory.read_16_bit_value_at(cpu.program_counter + 1);
    cpu.cycle += 3;
    cpu.program_counter += 3;
}

/// Store the contents of register A into the memory location specified by register pair HL, and
/// simultaneously decrement the contents of HL.
pub fn ld_hl_dec_a(cpu: &mut Cpu, memory: &mut Memory) {
    // Store contents of A
    let address_in_hl = lsb_msb_to_u16(cpu.h, cpu.l);
    memory.write_byte_at(address_in_hl, cpu.a);

    // Decrement contents of HL
    let (lsb, msb) = u16_to_lsb_msb(address_in_hl.wrapping_sub(1));
    cpu.h = lsb;
    cpu.l = msb;
    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Increment the contents of register pair SP by 1
pub fn inc_sp(cpu: &mut Cpu) {
    cpu.stack_pointer += 1;
    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Increment the contents of memory specified by register pair HL by 1
pub fn inc_hl_address(cpu: &mut Cpu, memory: &mut Memory) {
    let address = lsb_msb_to_u16(cpu.h, cpu.l);
    let value = memory.read_byte_at(address) + 1;
    memory.write_byte_at(address, value);

    cpu.zero = value == 0;
    cpu.negative = false;
    cpu.half_carry = is_nth_bit_set_u8(value, 7);
    cpu.program_counter += 1;
    cpu.cycle += 3;
}

/// Decrement the contents of memory specified by register pair HL by 1
pub fn dec_hl_address(cpu: &mut Cpu, memory: &mut Memory) {
    let address = lsb_msb_to_u16(cpu.h, cpu.l);
    let value = memory.read_byte_at(address) - 1;
    memory.write_byte_at(address, value);

    cpu.zero = value == 0;
    cpu.negative = true;
    cpu.half_carry = is_nth_bit_set_u8(value, 7);
    cpu.program_counter += 1;
    cpu.cycle += 3;
}

/// Store the contents of 8-bit immediate operand d8 in the memory location specified by register
/// pair HL
pub fn ld_hl_d8(cpu: &mut Cpu, memory: &mut Memory) {
    let value = memory.read_byte_at(cpu.program_counter + 1);
    let address = lsb_msb_to_u16(cpu.h, cpu.l);
    memory.write_byte_at(address, value);

    cpu.program_counter += 2;
    cpu.cycle += 3;
}

/// Set the carry flag CY
pub fn scf(cpu: &mut Cpu) {
    cpu.carry = true;
    cpu.negative = false;
    cpu.half_carry = false;
    cpu.carry = true;
}

/// If the CY flag is 1, jump s8 steps from the current address stored in the program counter (PC).
/// If not, the instruction following the current JP instruction is executed (as usual).
pub fn jr_c_s8(cpu: &mut Cpu, memory: &Memory) {
    if cpu.carry {
        cpu.program_counter += u16::from(memory.read_byte_at(cpu.program_counter + 1));
        cpu.cycle += 3;
    } else {
        cpu.program_counter += 2;
        cpu.cycle += 2;
    }
}

/// Add the contents of register pair SP to the contents of register pair HL, and store the results
/// in register pair HL.
pub fn add_hl_sp(cpu: &mut Cpu) {
    let value = lsb_msb_to_u16(cpu.h, cpu.l);

    cpu.negative = false;
    cpu.half_carry = is_nth_bit_set_u16(value, 15);
    cpu.carry = is_nth_bit_set_u16(value, 15);

    let (lsb, msb) = u16_to_lsb_msb(value.wrapping_add(cpu.stack_pointer));
    cpu.h = lsb;
    cpu.l = msb;
    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Load the contents of memory specified by register pair HL into register A, and simultaneously
/// decrement the contents of HL
pub fn ld_a_hl_dec(cpu: &mut Cpu, memory: &Memory) {
    // Load data into register A
    let address = lsb_msb_to_u16(cpu.h, cpu.l);
    cpu.a = memory.read_byte_at(address);

    // Decrement value in register pair HL
    let (lsb, msb) = u16_to_lsb_msb(address);
    cpu.h = lsb;
    cpu.l = msb;
    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Decrement the contents of register pair SP by 1
pub fn dec_sp(cpu: &mut Cpu) {
    cpu.stack_pointer -= 1;
    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Increment the contents of register A by 1
pub fn inc_a(cpu: &mut Cpu) {
    cpu.half_carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.a += 1;
    cpu.zero = cpu.a == 0;
    cpu.negative = false;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Decrement the contents of register A by 1
pub fn dec_a(cpu: &mut Cpu) {
    cpu.half_carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.a -= 1;
    cpu.zero = cpu.a == 0;
    cpu.negative = false;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the 8-bit immediate operand d8 into register A
pub fn ld_a_d8(cpu: &mut Cpu, memory: &Memory) {
    cpu.a = memory.read_byte_at(cpu.program_counter + 1);
    cpu.program_counter += 2;
    cpu.cycle += 2;
}

/// Flip the carry flag CY
pub fn ccf(cpu: &mut Cpu) {
    cpu.negative = false;
    cpu.half_carry = false;
    cpu.carry = !cpu.carry;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the contents of register B into register B
pub fn ld_b_b(cpu: &mut Cpu) {
    cpu.b = cpu.b;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the contents of register C into register B
pub fn ld_b_c(cpu: &mut Cpu) {
    cpu.b = cpu.c;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the contents of register D into register B
pub fn ld_b_d(cpu: &mut Cpu) {
    cpu.b = cpu.d;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the contents of register E into register B
pub fn ld_b_e(cpu: &mut Cpu) {
    cpu.b = cpu.e;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the contents of register H into register B
pub fn ld_b_h(cpu: &mut Cpu) {
    cpu.b = cpu.h;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the contents of register L into register B
pub fn ld_b_l(cpu: &mut Cpu) {
    cpu.b = cpu.l;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the 8-bit contents of memory specified by register pair HL into register B
pub fn ld_b_hl(cpu: &mut Cpu, memory: &Memory) {
    let address = lsb_msb_to_u16(cpu.h, cpu.l);
    cpu.b = memory.read_byte_at(address);
    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Load the contents of register A into register B
pub fn ld_b_a(cpu: &mut Cpu) {
    cpu.b = cpu.a;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the contents of register B into register C
pub fn ld_c_b(cpu: &mut Cpu) {
    cpu.c = cpu.b;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the contents of register C into register C
pub fn ld_c_c(cpu: &mut Cpu) {
    cpu.c = cpu.c;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the contents of register D into register C
pub fn ld_c_d(cpu: &mut Cpu) {
    cpu.c = cpu.d;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the contents of register E into register C
pub fn ld_c_e(cpu: &mut Cpu) {
    cpu.c = cpu.b;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the contents of register H into register C
pub fn ld_c_h(cpu: &mut Cpu) {
    cpu.c = cpu.b;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the contents of register L into register C
pub fn ld_c_l(cpu: &mut Cpu) {
    cpu.c = cpu.l;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the 8-bit contents of memory specified by register pair HL into register C
pub fn ld_c_hl(cpu: &mut Cpu, memory: &Memory) {
    let address = lsb_msb_to_u16(cpu.h, cpu.l);
    cpu.c = memory.read_byte_at(address);
    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Load the contents of register A into register C
pub fn ld_c_a(cpu: &mut Cpu) {
    cpu.c = cpu.a;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the contents of register B into register D
pub fn ld_d_b(cpu: &mut Cpu) {
    cpu.d = cpu.b;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the contents of register C into register D
pub fn ld_d_c(cpu: &mut Cpu) {
    cpu.d = cpu.c;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the contents of register D into register D
pub fn ld_d_d(cpu: &mut Cpu) {
    cpu.d = cpu.d;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the contents of register E into register D
pub fn ld_d_e(cpu: &mut Cpu) {
    cpu.d = cpu.e;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the contents of register H into register D
pub fn ld_d_h(cpu: &mut Cpu) {
    cpu.d = cpu.h;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the contents of register L into register D
pub fn ld_d_l(cpu: &mut Cpu) {
    cpu.d = cpu.l;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the 8-bit contents of memory specified by register pair HL into register D
pub fn ld_d_hl(cpu: &mut Cpu, memory: &Memory) {
    let address = lsb_msb_to_u16(cpu.h, cpu.l);
    cpu.d = memory.read_byte_at(address);
    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Load the contents of register A into register D
pub fn ld_d_a(cpu: &mut Cpu) {
    cpu.d = cpu.a;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the contents of register B into register E
pub fn ld_e_b(cpu: &mut Cpu) {
    cpu.e = cpu.b;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the contents of register C into register E
pub fn ld_e_c(cpu: &mut Cpu) {
    cpu.e = cpu.c;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the contents of register D into register E
pub fn ld_e_d(cpu: &mut Cpu) {
    cpu.e = cpu.d;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the contents of register E into register E
pub fn ld_e_e(cpu: &mut Cpu) {
    cpu.e = cpu.e;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the contents of register H into register E
pub fn ld_e_h(cpu: &mut Cpu) {
    cpu.e = cpu.h;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the contents of register L into register E
pub fn ld_e_l(cpu: &mut Cpu) {
    cpu.e = cpu.l;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the 8-bit contents of memory specified by register pair HL into register E
pub fn ld_e_hl(cpu: &mut Cpu, memory: &Memory) {
    let address = lsb_msb_to_u16(cpu.h, cpu.l);
    cpu.e = memory.read_byte_at(address);
    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Load the contents of register A into register E
pub fn ld_e_a(cpu: &mut Cpu) {
    cpu.e = cpu.a;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the contents of register B into register H
pub fn ld_h_b(cpu: &mut Cpu) {
    cpu.h = cpu.b;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the contents of register C into register H
pub fn ld_h_c(cpu: &mut Cpu) {
    cpu.h = cpu.c;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the contents of register D into register H
pub fn ld_h_d(cpu: &mut Cpu) {
    cpu.h = cpu.d;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the contents of register E into register H
pub fn ld_h_e(cpu: &mut Cpu) {
    cpu.h = cpu.e;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the contents of register H into register H
pub fn ld_h_h(cpu: &mut Cpu) {
    cpu.h = cpu.h;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the contents of register L into register H
pub fn ld_h_l(cpu: &mut Cpu) {
    cpu.h = cpu.l;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the 8-bit contents of memory specified by register pair HL into register H
pub fn ld_h_hl(cpu: &mut Cpu, memory: &Memory) {
    let address = lsb_msb_to_u16(cpu.h, cpu.l);
    cpu.h = memory.read_byte_at(address);
    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Load the contents of register A into register H
pub fn ld_h_a(cpu: &mut Cpu) {
    cpu.h = cpu.a;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the contents of register B into register L
pub fn ld_l_b(cpu: &mut Cpu) {
    cpu.l = cpu.b;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the contents of register C into register L
pub fn ld_l_c(cpu: &mut Cpu) {
    cpu.l = cpu.c;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the contents of register D into register L
pub fn ld_l_d(cpu: &mut Cpu) {
    cpu.l = cpu.d;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the contents of register E into register L
pub fn ld_l_e(cpu: &mut Cpu) {
    cpu.l = cpu.e;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the contents of register H into register L
pub fn ld_l_h(cpu: &mut Cpu) {
    cpu.l = cpu.h;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the contents of register L into register L
pub fn ld_l_l(cpu: &mut Cpu) {
    cpu.l = cpu.l;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the 8-bit contents of memory specified by register pair HL into register L
pub fn ld_l_hl(cpu: &mut Cpu, memory: &Memory) {
    let address = lsb_msb_to_u16(cpu.h, cpu.l);
    cpu.l = memory.read_byte_at(address);
    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Load the contents of register A into register L
pub fn ld_l_a(cpu: &mut Cpu) {
    cpu.l = cpu.a;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Store the contents of register B in the memory location specified by register pair HL
pub fn ld_hl_b(cpu: &mut Cpu, memory: &mut Memory) {
    let address = lsb_msb_to_u16(cpu.h, cpu.l);
    memory.write_byte_at(address, cpu.b);

    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Store the contents of register C in the memory location specified by register pair HL
pub fn ld_hl_c(cpu: &mut Cpu, memory: &mut Memory) {
    let address = lsb_msb_to_u16(cpu.h, cpu.l);
    memory.write_byte_at(address, cpu.c);

    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Store the contents of register D in the memory location specified by register pair HL
pub fn ld_hl_d(cpu: &mut Cpu, memory: &mut Memory) {
    let address = lsb_msb_to_u16(cpu.h, cpu.l);
    memory.write_byte_at(address, cpu.d);

    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Store the contents of register E in the memory location specified by register pair HL
pub fn ld_hl_e(cpu: &mut Cpu, memory: &mut Memory) {
    let address = lsb_msb_to_u16(cpu.h, cpu.l);
    memory.write_byte_at(address, cpu.e);

    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Store the contents of register H in the memory location specified by register pair HL
pub fn ld_hl_h(cpu: &mut Cpu, memory: &mut Memory) {
    let address = lsb_msb_to_u16(cpu.h, cpu.l);
    memory.write_byte_at(address, cpu.h);

    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Store the contents of register L in the memory location specified by register pair HL
pub fn ld_hl_l(cpu: &mut Cpu, memory: &mut Memory) {
    let address = lsb_msb_to_u16(cpu.h, cpu.l);
    memory.write_byte_at(address, cpu.l);

    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// After a HALT instruction is executed, the system clock is stopped and HALT mode is entered.
/// Although the system clock is stopped in this status, the oscillator circuit and LCD controller
/// continue to operate.
///
/// In addition, the status of the internal RAM register ports remains unchanged.
///
/// HALT mode is cancelled by an interrupt or reset signal.
///
/// The program counter is halted at the step after the HALT instruction. If both the interrupt
/// request flag and the corresponding interrupt enable flag are set, HALT mode is exited, even if
/// the interrupt master enable flag is not set.
///
/// Once HALT mode is cancelled, the program starts from the address indicated by the program
/// counter.
///
/// If the interrupt master enable flag is set, the contents of the program coounter are pushed to
/// the stack and control jumps to the starting address of the interrupt.
///
/// If the RESET terminal goes LOW in HALT mode, the mode becomes that of a normal reset.
pub fn halt(cpu: &mut Cpu) {
    cpu.state = Some(CpuState::Halt);
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Store the contents of register A in the memory location specified by register pair HL
pub fn ld_hl_a(cpu: &mut Cpu, memory: &mut Memory) {
    let address = lsb_msb_to_u16(cpu.h, cpu.l);
    memory.write_byte_at(address, cpu.a);

    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Load the contents of register B into register A
pub fn ld_a_b(cpu: &mut Cpu) {
    cpu.a = cpu.b;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the contents of register C into register A
pub fn ld_a_c(cpu: &mut Cpu) {
    cpu.a = cpu.c;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the contents of register D into register A
pub fn ld_a_d(cpu: &mut Cpu) {
    cpu.a = cpu.d;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the contents of register E into register A
pub fn ld_a_e(cpu: &mut Cpu) {
    cpu.a = cpu.e;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the contents of register H into register A
pub fn ld_a_h(cpu: &mut Cpu) {
    cpu.a = cpu.h;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the contents of register L into register A
pub fn ld_a_l(cpu: &mut Cpu) {
    cpu.a = cpu.l;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the 8-bit contents of memory specified by register pair HL into register A
pub fn ld_a_hl(cpu: &mut Cpu, memory: &Memory) {
    let address = lsb_msb_to_u16(cpu.h, cpu.l);
    cpu.a = memory.read_byte_at(address);
    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Load the contents of register A into register A
pub fn ld_a_a(cpu: &mut Cpu) {
    cpu.a = cpu.a;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Add the contents of register B to the contents of register A, and store the results in register
/// A
pub fn add_a_b(cpu: &mut Cpu) {
    cpu.half_carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.a = cpu.a.wrapping_add(cpu.b);
    cpu.zero = cpu.a == 0;
    cpu.negative = false;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Add the contents of register C to the contents of register A, and store the results in register
/// A
pub fn add_a_c(cpu: &mut Cpu) {
    cpu.half_carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.a = cpu.a.wrapping_add(cpu.c);
    cpu.zero = cpu.a == 0;
    cpu.negative = false;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Add the contents of register D to the contents of register A, and store the results in register
/// A
pub fn add_a_d(cpu: &mut Cpu) {
    cpu.half_carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.a = cpu.a.wrapping_add(cpu.d);
    cpu.zero = cpu.a == 0;
    cpu.negative = false;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Add the contents of register E to the contents of register A, and store the results in register
/// A
pub fn add_a_e(cpu: &mut Cpu) {
    cpu.half_carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.a = cpu.a.wrapping_add(cpu.e);
    cpu.zero = cpu.a == 0;
    cpu.negative = false;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Add the contents of register H to the contents of register A, and store the results in register
/// A
pub fn add_a_h(cpu: &mut Cpu) {
    cpu.half_carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.a = cpu.a.wrapping_add(cpu.h);
    cpu.zero = cpu.a == 0;
    cpu.negative = false;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Add the contents of register L to the contents of register A, and store the results in register
/// A
pub fn add_a_l(cpu: &mut Cpu) {
    cpu.half_carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.a = cpu.a.wrapping_add(cpu.l);
    cpu.zero = cpu.a == 0;
    cpu.negative = false;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Add the contents of memory specified by register pair HL to the contents of register A, and
/// store the results in register A
pub fn add_a_hl(cpu: &mut Cpu, memory: &Memory) {
    let address = lsb_msb_to_u16(cpu.h, cpu.l);

    cpu.half_carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.a = cpu.a.wrapping_add(memory.read_byte_at(address));
    cpu.zero = cpu.a == 0;
    cpu.negative = false;
    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Add the contents of register A to the contents of register A, and store the results in register
/// A
pub fn add_a_a(cpu: &mut Cpu) {
    cpu.half_carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.a = cpu.a.wrapping_add(cpu.a);
    cpu.zero = cpu.a == 0;
    cpu.negative = false;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Subtract the contents of register B from the contents of register A, and store the results in
/// register A.
pub fn sub_b(cpu: &mut Cpu) {
    cpu.half_carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.a = cpu.a.wrapping_sub(cpu.b);
    cpu.negative = true;
    cpu.zero = cpu.a == 0;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Subtract the contents of register C from the contents of register A, and store the results in
/// register A.
pub fn sub_c(cpu: &mut Cpu) {
    cpu.half_carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.a = cpu.a.wrapping_sub(cpu.c);
    cpu.negative = true;
    cpu.zero = cpu.a == 0;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Subtract the contents of register D from the contents of register A, and store the results in
/// register A.
pub fn sub_d(cpu: &mut Cpu) {
    cpu.half_carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.a = cpu.a.wrapping_sub(cpu.d);
    cpu.negative = true;
    cpu.zero = cpu.a == 0;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Subtract the contents of register E from the contents of register A, and store the results in
/// register A.
pub fn sub_e(cpu: &mut Cpu) {
    cpu.half_carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.a = cpu.a.wrapping_sub(cpu.e);
    cpu.negative = true;
    cpu.zero = cpu.a == 0;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Subtract the contents of register H from the contents of register A, and store the results in
/// register A.
pub fn sub_h(cpu: &mut Cpu) {
    cpu.half_carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.a = cpu.a.wrapping_sub(cpu.h);
    cpu.negative = true;
    cpu.zero = cpu.a == 0;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Subtract the contents of register L from the contents of register A, and store the results in
/// register A.
pub fn sub_l(cpu: &mut Cpu) {
    cpu.half_carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.a = cpu.a.wrapping_sub(cpu.l);
    cpu.negative = true;
    cpu.zero = cpu.a == 0;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Subtract the contents of memory specified by register pair HL from the contents of register A,
/// and store the results in register A.
pub fn sub_hl(cpu: &mut Cpu, memory: &Memory) {
    let address = lsb_msb_to_u16(cpu.h, cpu.l);
    let value = memory.read_byte_at(address);

    cpu.half_carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.a = cpu.a.wrapping_sub(value);
    cpu.negative = true;
    cpu.zero = cpu.a == 0;
    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Subtract the contents of register A from the contents of register A, and store the results in
/// register A.
pub fn sub_a(cpu: &mut Cpu) {
    cpu.half_carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.a = cpu.a.wrapping_sub(cpu.a);
    cpu.negative = true;
    cpu.zero = cpu.a == 0;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Take the logical AND for each bit of the contents of register B and the contents of register A,
/// and store the results in register A.
pub fn and_b(cpu: &mut Cpu) {
    cpu.a &= cpu.b;
    cpu.zero = cpu.a == 0;
    cpu.negative = false;
    cpu.half_carry = true;
    cpu.carry = false;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Take the logical AND for each bit of the contents of register C and the contents of register A,
/// and store the results in register A.
pub fn and_c(cpu: &mut Cpu) {
    cpu.a &= cpu.c;
    cpu.zero = cpu.a == 0;
    cpu.negative = false;
    cpu.half_carry = true;
    cpu.carry = false;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Take the logical AND for each bit of the contents of register D and the contents of register A,
/// and store the results in register A.
pub fn and_d(cpu: &mut Cpu) {
    cpu.a &= cpu.d;
    cpu.zero = cpu.a == 0;
    cpu.negative = false;
    cpu.half_carry = true;
    cpu.carry = false;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Take the logical AND for each bit of the contents of register E and the contents of register A,
/// and store the results in register A.
pub fn and_e(cpu: &mut Cpu) {
    cpu.a &= cpu.e;
    cpu.zero = cpu.a == 0;
    cpu.negative = false;
    cpu.half_carry = true;
    cpu.carry = false;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Take the logical AND for each bit of the contents of register H and the contents of register A,
/// and store the results in register A.
pub fn and_h(cpu: &mut Cpu) {
    cpu.a &= cpu.h;
    cpu.zero = cpu.a == 0;
    cpu.negative = false;
    cpu.half_carry = true;
    cpu.carry = false;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Take the logical AND for each bit of the contents of register L and the contents of register A,
/// and store the results in register A.
pub fn and_l(cpu: &mut Cpu) {
    cpu.a &= cpu.l;
    cpu.zero = cpu.a == 0;
    cpu.negative = false;
    cpu.half_carry = true;
    cpu.carry = false;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Take the logical AND for each bit of the contents of memory specified by register pair HL and
/// the contents of register A, and store the results in register A.
pub fn and_hl(cpu: &mut Cpu, memory: &Memory) {
    let address = lsb_msb_to_u16(cpu.h, cpu.l);
    let value = memory.read_byte_at(address);

    cpu.a &= value;
    cpu.zero = cpu.a == 0;
    cpu.negative = false;
    cpu.half_carry = true;
    cpu.carry = false;
    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Take the logical AND for each bit of the contents of register A and the contents of register A,
/// and store the results in register A.
pub fn and_a(cpu: &mut Cpu) {
    cpu.a &= cpu.a;
    cpu.zero = cpu.a == 0;
    cpu.negative = false;
    cpu.half_carry = true;
    cpu.carry = false;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Take the logical exclusive-OR for each bit of the contents of register B and the contents of
/// register A, and store the results in register A.
pub fn xor_b(cpu: &mut Cpu) {
    cpu.a ^= cpu.b;
    cpu.zero = cpu.a == 0;
    cpu.negative = false;
    cpu.half_carry = false;
    cpu.carry = false;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Take the logical exclusive-OR for each bit of the contents of register C and the contents of
/// register A, and store the results in register A.
pub fn xor_c(cpu: &mut Cpu) {
    cpu.a ^= cpu.c;
    cpu.zero = cpu.a == 0;
    cpu.negative = false;
    cpu.half_carry = false;
    cpu.carry = false;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Take the logical exclusive-OR for each bit of the contents of register D and the contents of
/// register A, and store the results in register A.
pub fn xor_d(cpu: &mut Cpu) {
    cpu.a ^= cpu.d;
    cpu.zero = cpu.a == 0;
    cpu.negative = false;
    cpu.half_carry = false;
    cpu.carry = false;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Take the logical exclusive-OR for each bit of the contents of register E and the contents of
/// register A, and store the results in register A.
pub fn xor_e(cpu: &mut Cpu) {
    cpu.a ^= cpu.e;
    cpu.zero = cpu.a == 0;
    cpu.negative = false;
    cpu.half_carry = false;
    cpu.carry = false;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Take the logical exclusive-OR for each bit of the contents of register H and the contents of
/// register A, and store the results in register A.
pub fn xor_h(cpu: &mut Cpu) {
    cpu.a ^= cpu.h;
    cpu.zero = cpu.a == 0;
    cpu.negative = false;
    cpu.half_carry = false;
    cpu.carry = false;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Take the logical exclusive-OR for each bit of the contents of register L and the contents of
/// register A, and store the results in register A.
pub fn xor_l(cpu: &mut Cpu) {
    cpu.a ^= cpu.l;
    cpu.zero = cpu.a == 0;
    cpu.negative = false;
    cpu.half_carry = false;
    cpu.carry = false;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Take the logical exclusive-OR for each bit of the contents of memory specified by register pair
/// HL and the contents of register A, and store the results in register A.
pub fn xor_hl(cpu: &mut Cpu, memory: &Memory) {
    let address = lsb_msb_to_u16(cpu.h, cpu.l);
    let value = memory.read_byte_at(address);

    cpu.a ^= value;
    cpu.zero = cpu.a == 0;
    cpu.negative = false;
    cpu.half_carry = false;
    cpu.carry = false;
    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Take the logical exclusive-OR for each bit of the contents of register A and the contents of
/// register A, and store the results in register A.
pub fn xor_a(cpu: &mut Cpu) {
    cpu.a ^= cpu.a;
    cpu.zero = cpu.a == 0;
    cpu.negative = false;
    cpu.half_carry = false;
    cpu.carry = false;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Take the logical OR for each bit of the contents of register B and the contents of
/// register A, and store the results in register A.
pub fn or_b(cpu: &mut Cpu) {
    cpu.a |= cpu.b;
    cpu.zero = cpu.a == 0;
    cpu.negative = false;
    cpu.half_carry = false;
    cpu.carry = false;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Take the logical OR for each bit of the contents of register C and the contents of
/// register A, and store the results in register A.
pub fn or_c(cpu: &mut Cpu) {
    cpu.a |= cpu.c;
    cpu.zero = cpu.a == 0;
    cpu.negative = false;
    cpu.half_carry = false;
    cpu.carry = false;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Take the logical OR for each bit of the contents of register D and the contents of
/// register A, and store the results in register A.
pub fn or_d(cpu: &mut Cpu) {
    cpu.a |= cpu.d;
    cpu.zero = cpu.a == 0;
    cpu.negative = false;
    cpu.half_carry = false;
    cpu.carry = false;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Take the logical OR for each bit of the contents of register E and the contents of
/// register A, and store the results in register A.
pub fn or_e(cpu: &mut Cpu) {
    cpu.a |= cpu.e;
    cpu.zero = cpu.a == 0;
    cpu.negative = false;
    cpu.half_carry = false;
    cpu.carry = false;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Take the logical OR for each bit of the contents of register H and the contents of
/// register A, and store the results in register A.
pub fn or_h(cpu: &mut Cpu) {
    cpu.a |= cpu.h;
    cpu.zero = cpu.a == 0;
    cpu.negative = false;
    cpu.half_carry = false;
    cpu.carry = false;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Take the logical OR for each bit of the contents of register L and the contents of
/// register A, and store the results in register A.
pub fn or_l(cpu: &mut Cpu) {
    cpu.a |= cpu.l;
    cpu.zero = cpu.a == 0;
    cpu.negative = false;
    cpu.half_carry = false;
    cpu.carry = false;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Take the logical OR for each bit of the contents of memory specified by register pair
/// HL and the contents of register A, and store the results in register A.
pub fn or_hl(cpu: &mut Cpu, memory: &Memory) {
    let address = lsb_msb_to_u16(cpu.h, cpu.l);
    let value = memory.read_byte_at(address);

    cpu.a |= value;
    cpu.zero = cpu.a == 0;
    cpu.negative = false;
    cpu.half_carry = false;
    cpu.carry = false;
    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Take the logical OR for each bit of the contents of register A and the contents of
/// register A, and store the results in register A.
pub fn or_a(cpu: &mut Cpu) {
    cpu.a |= cpu.a;
    cpu.zero = cpu.a == 0;
    cpu.negative = false;
    cpu.half_carry = false;
    cpu.carry = false;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Compare the contents of register B and the contents of register A by calculating A - B, and set
/// the Z flag if they are equal.
pub fn cp_b(cpu: &mut Cpu) {
    cpu.zero = cpu.a.wrapping_sub(cpu.b) == 0;
    cpu.negative = true;
    cpu.half_carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Compare the contents of register C and the contents of register A by calculating A - C, and set
/// the Z flag if they are equal.
pub fn cp_c(cpu: &mut Cpu) {
    cpu.zero = cpu.a.wrapping_sub(cpu.c) == 0;
    cpu.negative = true;
    cpu.half_carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Compare the contents of register D and the contents of register A by calculating A - D, and set
/// the Z flag if they are equal.
pub fn cp_d(cpu: &mut Cpu) {
    cpu.zero = cpu.a.wrapping_sub(cpu.d) == 0;
    cpu.negative = true;
    cpu.half_carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Compare the contents of register E and the contents of register A by calculating A - E, and set
/// the Z flag if they are equal.
pub fn cp_e(cpu: &mut Cpu) {
    cpu.zero = cpu.a.wrapping_sub(cpu.e) == 0;
    cpu.negative = true;
    cpu.half_carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Compare the contents of register H and the contents of register A by calculating A - H, and set
/// the Z flag if they are equal.
pub fn cp_h(cpu: &mut Cpu) {
    cpu.zero = cpu.a.wrapping_sub(cpu.h) == 0;
    cpu.negative = true;
    cpu.half_carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Compare the contents of register L and the contents of register A by calculating A - L, and set
/// the Z flag if they are equal.
pub fn cp_l(cpu: &mut Cpu) {
    cpu.zero = cpu.a.wrapping_sub(cpu.l) == 0;
    cpu.negative = true;
    cpu.half_carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Compare the contents of memory specified by register pair HL and the contents of register A by
/// calculating A - (HL), and set the Z flag if they are equal.
pub fn cp_hl(cpu: &mut Cpu, memory: &Memory) {
    let address = lsb_msb_to_u16(cpu.h, cpu.l);
    let value = memory.read_byte_at(address);

    cpu.zero = cpu.a.wrapping_sub(value) == 0;
    cpu.negative = true;
    cpu.half_carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Compare the contents of register A and the contents of register A by calculating A - A, and set
/// the Z flag if they are equal.
pub fn cp_a(cpu: &mut Cpu) {
    cpu.zero = cpu.a.wrapping_sub(cpu.a) == 0;
    cpu.negative = true;
    cpu.half_carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// If the Z flag is 0, control is returned to the source program by popping from the memory stack
/// the program counter PC value that was pushed to the stack when the subroutine was called.
pub fn ret_nz(cpu: &mut Cpu, memory: &mut Memory) {
    if cpu.zero {
        cpu.program_counter += 1;
        cpu.cycle += 2;
    } else {
        cpu.program_counter = memory.pop_stack_u16(cpu);
        cpu.cycle += 5;
    }
}

/// Pop the contents from the memory stack into register pair into register pair BC.
pub fn pop_bc(cpu: &mut Cpu, memory: &mut Memory) {
    cpu.b = memory.pop_stack_u8(cpu);
    cpu.c = memory.pop_stack_u8(cpu);
    cpu.program_counter += 1;
    cpu.cycle += 3;
}

/// Load the 16-bit immediate operand a16 into the program counter PC if the Z flag is 0. If the Z
/// flag is 0, then the subsequent instruction starts at address a16. If not, the contents of PC
/// are incremented, and the next instruction following the current JP instruction is executed
/// (as usual).
pub fn jp_nz_a16(cpu: &mut Cpu, memory: &Memory) {
    if cpu.zero {
        cpu.program_counter += 3;
        cpu.cycle += 3;
    } else {
        cpu.program_counter = memory.read_16_bit_value_at(cpu.program_counter + 1);
        cpu.cycle += 4;
    }
}

/// Load the 16-bit immediate operand a16 into the program counter (PC)
pub fn jp_a16(cpu: &mut Cpu, memory: &Memory) {
    // Jump to the specified address
    cpu.program_counter = memory.read_16_bit_value_at(cpu.program_counter + 1);
    cpu.cycle += 4;
}

/// If the Z flag is 0, the program counter PC value corresponding to the memory location of the
/// instruction following the CALL instruction is pushed to the 2 bytes following the memory byte
/// specified by the stack pointer SP. The 16-bit immediate operand a16 is then loaded into PC.
pub fn call_nz_a16(cpu: &mut Cpu, memory: &mut Memory) {
    if cpu.zero {
        cpu.program_counter += 3;
        cpu.cycle += 3;
    } else {
        memory.push_stack_u16(cpu.program_counter + 3, cpu);
        cpu.program_counter = memory.read_16_bit_value_at(cpu.program_counter + 1);
        cpu.cycle += 6;
    }
}

/// Push the contents of register pair BC onto the memory stack.
pub fn push_bc(cpu: &mut Cpu, memory: &mut Memory) {
    let value = lsb_msb_to_u16(cpu.b, cpu.c);
    memory.push_stack_u16(value, cpu);

    cpu.program_counter += 1;
    cpu.cycle += 4;
}

/// Add the contents of the 8-bit immediate operand d8 to the contents of register A, and store the
/// results in register A.
pub fn add_a_d8(cpu: &mut Cpu, memory: &Memory) {
    let value = memory.read_byte_at(cpu.program_counter + 1);

    cpu.half_carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.a = cpu.a.wrapping_add(value);
    cpu.negative = false;
    cpu.zero = cpu.a == 0;
    cpu.program_counter += 2;
    cpu.cycle += 2;
}

/// If the Z flag is 1, control is returned to the source program by popping from the memory stack
/// the program counter PC value that was pushed to the stack when the subroutine was called.
pub fn ret_z(cpu: &mut Cpu, memory: &mut Memory) {
    if cpu.zero {
        cpu.program_counter = memory.pop_stack_u16(cpu);
        cpu.cycle += 5;
    } else {
        cpu.program_counter += 1;
        cpu.cycle += 2;
    }
}

/// Return from a subroutine
pub fn ret(cpu: &mut Cpu, memory: &mut Memory) {
    cpu.program_counter = memory.pop_stack_u16(cpu);
    cpu.cycle += 4;
}

/// Load the 16-bit immediate operand a16 into the program counter PC if the Z flag is 1. If the Z
/// flag is 1, then the subsequent instruction starts at address a16. If not, the contents of PC
/// are incremented, and the next instruction following the current JP instruction is executed
/// (as usual).
pub fn jp_z_a16(cpu: &mut Cpu, memory: &Memory) {
    if cpu.zero {
        cpu.program_counter = memory.read_16_bit_value_at(cpu.program_counter + 1);
        cpu.cycle += 4;
    } else {
        cpu.program_counter += 3;
        cpu.cycle += 3;
    }
}

/// If the Z flag is 1, the program counter PC value corresponding to the memory location of the
/// instruction following the CALL instruction is pushed to the 2 bytes following the memory byte
/// specified by the stack pointer SP. The 16-bit immediate operand a16 is then loaded into PC.
pub fn call_z_a16(cpu: &mut Cpu, memory: &mut Memory) {
    if cpu.zero {
        memory.push_stack_u16(cpu.program_counter + 3, cpu);
        cpu.program_counter = memory.read_16_bit_value_at(cpu.program_counter + 1);
        cpu.cycle += 6;
    } else {
        cpu.program_counter += 3;
        cpu.cycle += 3;
    }
}

/// In memory, push the program counter PC value corresponding to the address following the CALL
/// instruction to the 2 bytes following the byte specified by the current stack pointer SP. Then
/// load the 16-bit immediate operand a16 into PC.
///
/// The subroutine is placed after the location specified by the new PC value. When the subroutine
/// finishes, control is returned to the source program using a return instruction and by popping
/// the starting address of the next instruction (which was just pushed) and moving it to the PC.
pub fn call_a16(cpu: &mut Cpu, memory: &mut Memory) {
    // Store return address on the stack
    let return_address_after_call = memory.read_16_bit_value_at(cpu.program_counter + 3);
    memory.push_stack_u16(return_address_after_call, cpu);

    // Go to the subroutine's address
    cpu.program_counter = memory.read_16_bit_value_at(cpu.program_counter + 1);
    cpu.cycle += 6;
}

/// If the CY flag is 0, control is returned to the source program by popping from the memory stack
/// the program counter PC value that was pushed to the stack when the subroutine was called.
pub fn ret_nc(cpu: &mut Cpu, memory: &mut Memory) {
    if cpu.carry {
        cpu.program_counter += 1;
        cpu.cycle += 2;
    } else {
        cpu.program_counter = memory.pop_stack_u16(cpu);
        cpu.cycle += 5;
    }
}

/// Pop the contents from the memory stack into register pair into register pair DE.
pub fn pop_de(cpu: &mut Cpu, memory: &mut Memory) {
    cpu.d = memory.pop_stack_u8(cpu);
    cpu.e = memory.pop_stack_u8(cpu);
    cpu.program_counter += 1;
    cpu.cycle += 3;
}

/// Load the 16-bit immediate operand a16 into the program counter PC if the CY flag is 0. If the
/// CY flag is 0, then the subsequent instruction starts at address a16. If not, the contents of PC
/// are incremented, and the next instruction following the current JP instruction is executed
/// (as usual).
pub fn jp_nc_a16(cpu: &mut Cpu, memory: &Memory) {
    if cpu.carry {
        cpu.program_counter += 3;
        cpu.cycle += 3;
    } else {
        cpu.program_counter = memory.read_16_bit_value_at(cpu.program_counter + 1);
        cpu.cycle += 4;
    }
}

/// If the CY flag is 0, the program counter PC value corresponding to the memory location of the
/// instruction following the CALL instruction is pushed to the 2 bytes following the memory byte
/// specified by the stack pointer SP. The 16-bit immediate operand a16 is then loaded into PC.
pub fn call_nc_a16(cpu: &mut Cpu, memory: &mut Memory) {
    if cpu.carry {
        cpu.program_counter += 3;
        cpu.cycle += 3;
    } else {
        memory.push_stack_u16(cpu.program_counter + 3, cpu);
        cpu.program_counter = memory.read_16_bit_value_at(cpu.program_counter + 1);
        cpu.cycle += 6;
    }
}

/// Push the contents of register pair DE onto the memory stack.
pub fn push_de(cpu: &mut Cpu, memory: &mut Memory) {
    let value = lsb_msb_to_u16(cpu.d, cpu.e);
    memory.push_stack_u16(value, cpu);

    cpu.program_counter += 1;
    cpu.cycle += 4;
}

/// Subtract the contents of the 8-bit immediate operand d8 from the contents of register A, and
/// store the results in register A.
pub fn sub_d8(cpu: &mut Cpu, memory: &Memory) {
    let value = memory.read_byte_at(cpu.program_counter + 1);

    cpu.half_carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.a = cpu.a.wrapping_sub(value);
    cpu.negative = true;
    cpu.zero = cpu.a == 0;
    cpu.program_counter += 2;
    cpu.cycle += 2;
}

/// If the CY flag is 1, control is returned to the source program by popping from the memory stack
/// the program counter PC value that was pushed to the stack when the subroutine was called.
pub fn ret_c(cpu: &mut Cpu, memory: &mut Memory) {
    if cpu.carry {
        cpu.program_counter = memory.pop_stack_u16(cpu);
        cpu.cycle += 5;
    } else {
        cpu.program_counter += 1;
        cpu.cycle += 2;
    }
}

/// Used when an interrupt-service routine finishes. The address for the return from the interrupt
/// is loaded in the program counter PC. The master interrupt enable flag is returned to its
/// pre-interrupt status.
pub fn reti(cpu: &mut Cpu, memory: &mut Memory) {
    cpu.program_counter = memory.pop_stack_u16(cpu);
    cpu.ime = false;

    cpu.set_interrupt_flag_state(memory, InterruptFlag::VBlank(Some(false)));
    cpu.set_interrupt_flag_state(memory, InterruptFlag::LcdStat(Some(false)));
    cpu.set_interrupt_flag_state(memory, InterruptFlag::Timer(Some(false)));
    cpu.set_interrupt_flag_state(memory, InterruptFlag::Serial(Some(false)));
    cpu.set_interrupt_flag_state(memory, InterruptFlag::Joypad(Some(false)));

    cpu.cycle += 4;
}

/// Load the 16-bit immediate operand a16 into the program counter PC if the CY flag is 1. If the
/// CY flag is 1, then the subsequent instruction starts at address a16. If not, the contents of PC
/// are incremented, and the next instruction following the current JP instruction is executed
/// (as usual).
pub fn jp_c_a16(cpu: &mut Cpu, memory: &Memory) {
    if cpu.carry {
        cpu.program_counter = memory.read_16_bit_value_at(cpu.program_counter + 1);
        cpu.cycle += 4;
    } else {
        cpu.program_counter += 3;
        cpu.cycle += 3;
    }
}

/// If the CY flag is 1, the program counter PC value corresponding to the memory location of the
/// instruction following the CALL instruction is pushed to the 2 bytes following the memory byte
/// specified by the stack pointer SP. The 16-bit immediate operand a16 is then loaded into PC.
pub fn call_c_a16(cpu: &mut Cpu, memory: &mut Memory) {
    if cpu.carry {
        memory.push_stack_u16(cpu.program_counter + 3, cpu);
        cpu.program_counter = memory.read_16_bit_value_at(cpu.program_counter + 1);
        cpu.cycle += 6;
    } else {
        cpu.program_counter += 3;
        cpu.cycle += 3;
    }
}

/// Store the contents of register A in the internal RAM, port register, or mode register at the
/// address in the range 0xFF00-0xFFFF specified by the 8-bit immediate operand a8
pub fn ld_a8_a(cpu: &mut Cpu, memory: &mut Memory) {
    let address = u16::from(memory.read_byte_at(cpu.program_counter + 1));
    memory.write_byte_at(address, cpu.a);

    cpu.program_counter += 2;
    cpu.cycle += 3;
}

/// Pop the contents from the memory stack into register pair into register pair HL.
pub fn pop_hl(cpu: &mut Cpu, memory: &mut Memory) {
    cpu.h = memory.pop_stack_u8(cpu);
    cpu.l = memory.pop_stack_u8(cpu);
    cpu.program_counter += 1;
    cpu.cycle += 3;
}

/// Store the contents of register A in the internal RAM, port register, or mode register at the
/// address in the range 0xFF00-0xFFFF specified by register C.
pub fn ld_port_c_a(cpu: &mut Cpu, memory: &mut Memory) {
    let address = lsb_msb_to_u16(cpu.c, 0xFF);
    memory.write_byte_at(address, cpu.a);

    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Push the contents of register pair HL onto the memory stack.
pub fn push_hl(cpu: &mut Cpu, memory: &mut Memory) {
    let value = lsb_msb_to_u16(cpu.h, cpu.l);
    memory.push_stack_u16(value, cpu);

    cpu.program_counter += 1;
    cpu.cycle += 4;
}

/// Take the logical AND for each bit of the contents of 8-bit immediate operand d8 and the
/// contents of register A, and store the results in register A.
pub fn and_d8(cpu: &mut Cpu, memory: &Memory) {
    let value = memory.read_byte_at(cpu.program_counter + 1);

    cpu.a &= value;
    cpu.zero = cpu.a == 0;
    cpu.negative = false;
    cpu.half_carry = true;
    cpu.carry = false;
    cpu.program_counter += 2;
    cpu.cycle += 2;
}

/// Add the contents of the 8-bit signed (2's complement) immediate operand s8 and the stack
/// pointer SP and store the results in SP.
pub fn add_sp_s8(cpu: &mut Cpu, memory: &Memory) {
    let value = memory.read_byte_at(cpu.program_counter + 1) as i16;

    cpu.zero = false;
    cpu.negative = false;
    cpu.half_carry = is_nth_bit_set_u16(cpu.stack_pointer, 15);
    cpu.carry = is_nth_bit_set_u16(cpu.stack_pointer, 15);
    cpu.stack_pointer = (cpu.stack_pointer as i16 + value) as u16;
    cpu.program_counter += 2;
    cpu.cycle += 4;
}

/// Load the contents of register pair HL into the program counter PC. The next instruction is
/// fetched from the location specified by the new value of PC.
pub fn jp_hl(cpu: &mut Cpu) {
    cpu.program_counter = lsb_msb_to_u16(cpu.h, cpu.l);
    cpu.cycle += 1;
}

/// Store the contents of register A in the internal RAM or register specified by the 16-bit
/// immediate operand a16
pub fn ld_a16_a(cpu: &mut Cpu, memory: &mut Memory) {
    let address = memory.read_16_bit_value_at(cpu.program_counter + 1);
    memory.write_byte_at(address, cpu.a);

    cpu.program_counter += 3;
    cpu.cycle += 4;
}

/// Take the logical exclusive-OR for each bit of the contents of the 8-bit immediate operand d8
/// and the contents of register A, and store the results in register A.
pub fn xor_d8(cpu: &mut Cpu, memory: &Memory) {
    let value = memory.read_byte_at(cpu.program_counter + 1);

    cpu.a ^= value;
    cpu.zero = cpu.a == 0;
    cpu.negative = false;
    cpu.half_carry = false;
    cpu.carry = false;
    cpu.program_counter += 2;
    cpu.cycle += 2;
}

/// Load into register A the contents of the internal RAM, port register, or mode register at the
/// address in the range 0xFF00-0xFFFF specified by the 8-bit immediate operand a8.
pub fn ld_a_a8(cpu: &mut Cpu, memory: &mut Memory) {
    let address_lsb = memory.read_byte_at(cpu.program_counter + 1);
    let address = lsb_msb_to_u16(address_lsb, 0xFF);
    memory.write_byte_at(address, cpu.a);

    cpu.program_counter += 2;
    cpu.cycle += 3;
}

/// Pop the contents from the memory stack into register pair into register pair AF.
pub fn pop_af(cpu: &mut Cpu, memory: &mut Memory) {
    // Lower portion contains flags
    let flags = memory.pop_stack_u8(cpu);

    // Convert bit flags into the Boolean flags used in the emulator
    cpu.carry = is_nth_bit_set_u8(flags, 0);
    cpu.half_carry = is_nth_bit_set_u8(flags, 1);
    cpu.negative = is_nth_bit_set_u8(flags, 2);
    cpu.zero = is_nth_bit_set_u8(flags, 3);

    // Upper portion contains data for the A register
    cpu.a = memory.pop_stack_u8(cpu);

    cpu.program_counter += 1;
    cpu.cycle += 3;
}

/// Load into register A the contents of the internal RAM, port register, or mode register at the
/// address in the range 0xFF00-0xFFFF specified by register C.
pub fn ld_a_port_c(cpu: &mut Cpu, memory: &mut Memory) {
    let address = lsb_msb_to_u16(cpu.c, 0xFF);
    cpu.a = memory.read_byte_at(address);

    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Reset the interrupt master enable (IME) flag and prohibit maskable interrupts
pub fn di(cpu: &mut Cpu, memory: &mut Memory) {
    cpu.set_interrupt_flag_state(memory, InterruptFlag::VBlank(Some(false)));
    cpu.set_interrupt_flag_state(memory, InterruptFlag::LcdStat(Some(false)));
    cpu.set_interrupt_flag_state(memory, InterruptFlag::Timer(Some(false)));
    cpu.set_interrupt_flag_state(memory, InterruptFlag::Serial(Some(false)));
    cpu.set_interrupt_flag_state(memory, InterruptFlag::Joypad(Some(false)));
    cpu.ime = false;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Push the contents of register pair AF onto the memory stack.
pub fn push_af(cpu: &mut Cpu, memory: &mut Memory) {
    // Higher portion should contain the data in the A register
    memory.push_stack_u8(cpu.a, cpu);

    // Lower portion should contain the flags represented as bits
    let mut flags = 0u8;
    flags = set_bit_n_state_u8(flags, 0, cpu.carry);
    flags = set_bit_n_state_u8(flags, 1, cpu.half_carry);
    flags = set_bit_n_state_u8(flags, 2, cpu.negative);
    flags = set_bit_n_state_u8(flags, 3, cpu.zero);

    memory.push_stack_u8(flags, cpu);

    cpu.program_counter += 1;
    cpu.cycle += 4;
}

/// Take the logical OR for each bit of the contents of the 8-bit immediate operand d8 and the
/// contents of register A, and store the results in register A.
pub fn or_d8(cpu: &mut Cpu, memory: &Memory) {
    let value = memory.read_byte_at(cpu.program_counter + 1);

    cpu.a |= value;
    cpu.zero = cpu.a == 0;
    cpu.negative = false;
    cpu.half_carry = false;
    cpu.carry = false;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Add the 8-bit signed operand s8 (values -128 to +127) to the stack pointer SP, and store the
/// result in register pair HL.
pub fn ld_hl_sp_plus_s8(cpu: &mut Cpu, memory: &mut Memory) {
    cpu.zero = false;
    cpu.negative = false;
    cpu.half_carry = is_nth_bit_set_u16(cpu.stack_pointer, 15);
    cpu.carry = is_nth_bit_set_u16(cpu.stack_pointer, 15);

    let value = memory.read_byte_at(cpu.program_counter + 1) as i16;
    let result = (cpu.stack_pointer as i16 + value) as u16;
    let (lsb, msb) = u16_to_lsb_msb(result);

    cpu.h = lsb;
    cpu.l = msb;
    cpu.program_counter += 2;
    cpu.cycle += 3;
}

/// Load the contents of register pair HL into the stack pointer SP.
pub fn ld_sp_hl(cpu: &mut Cpu) {
    cpu.stack_pointer = lsb_msb_to_u16(cpu.h, cpu.l);
    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Load into register A the contents of the internal RAM or register specified by the 16-bit
/// immediate operand a16.
pub fn ld_a_a16(cpu: &mut Cpu, memory: &Memory) {
    let address = memory.read_16_bit_value_at(cpu.program_counter + 1);
    cpu.a = memory.read_byte_at(address);
    cpu.program_counter += 3;
    cpu.cycle += 4;
}

/// Set the interrupt master enable (IME) flag and enable maskable interrupts. This instruction can
/// be used in an interrupt routine to enable higher-order interrupts.
///
/// The IME flag is reset immediately after an interrupt occurs. The IME flag reset remains in
/// effect if control is returned from the interrupt routine by a RET instruction. However, if an
/// EI instruction is executed in the interrupt routine, control is returned with IME = 1.
pub fn ei(cpu: &mut Cpu) {
    cpu.ime = true;
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Compare the contents of register A and the contents of the 8-bit immediate operand d8 by
/// calculating A - d8, and set the Z flag if they are equal.
///
/// The execution of this instruction does not affect the contents of register A.
pub fn cp_d8(cpu: &mut Cpu, memory: &Memory) {
    let value = memory.read_byte_at(cpu.program_counter + 1);

    cpu.zero = cpu.a.wrapping_sub(value) == 0;
    cpu.negative = true;
    cpu.half_carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.program_counter += 2;
    cpu.cycle += 2;
}
