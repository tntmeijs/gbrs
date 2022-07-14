use log::debug;

use crate::{
    bits::{is_nth_bit_set_u16, is_nth_bit_set_u8, lsb_msb_to_u16, u16_to_lsb_msb},
    cpu::Cpu,
    memory::Memory,
};

/// Only advances the program counter by 1
pub fn nop(cpu: &mut Cpu) {
    debug!("NOP");
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the 2 bytes of immediate data into register pair BC
pub fn ld_bc_d16(cpu: &mut Cpu, memory: &Memory) {
    let value = memory.read_16_bit_value_at(cpu.program_counter + 1);
    let (lsb, msb) = u16_to_lsb_msb(value);

    cpu.b = lsb;
    cpu.c = msb;

    debug!("LD {:#06X} into register pair BC", value);
    cpu.program_counter += 3;
    cpu.cycle += 3;
}

/// Store the contents of register A in the memory location specified by register pair BC
pub fn ld_bc_a(cpu: &mut Cpu, memory: &mut Memory) {
    memory.write_byte_at(lsb_msb_to_u16(cpu.b, cpu.c), cpu.a);

    debug!("LD A into address {:#06X}", lsb_msb_to_u16(cpu.b, cpu.c));
    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Increment the contents of register pair BC by 1
pub fn inc_bc(cpu: &mut Cpu) {
    let value = lsb_msb_to_u16(cpu.b, cpu.c) + 1;
    let (lsb, msb) = u16_to_lsb_msb(value);

    cpu.b = lsb;
    cpu.c = msb;

    debug!("INC register pair BC");
    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Increment the contents of register B by 1
pub fn inc_b(cpu: &mut Cpu) {
    cpu.half_carry = is_nth_bit_set_u8(cpu.b, 7);
    cpu.b += 1;
    cpu.zero = cpu.b == 0;
    cpu.negative = false;

    debug!("INC register B");
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Decrement the contents of register B by 1
pub fn dec_b(cpu: &mut Cpu) {
    cpu.half_carry = is_nth_bit_set_u8(cpu.b, 7);
    cpu.b -= 1;
    cpu.zero = cpu.b == 0;
    cpu.negative = true;

    debug!("DEC register B");
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the 8-bit immediate operand d8 into register B
pub fn ld_b_d8(cpu: &mut Cpu, memory: &Memory) {
    cpu.b = memory.read_byte_at(cpu.program_counter + 1);

    debug!("LD {:#06X} into register B", cpu.b);
    cpu.program_counter += 2;
    cpu.cycle += 2;
}

/// Store the lower byte of stack pointer SP at the address specified by the 16-bit immediate
/// operand a16, and store the upper byte of SP at address a16 + 1
pub fn ld_a16_sp(cpu: &mut Cpu, memory: &mut Memory) {
    let (lsb, msb) = u16_to_lsb_msb(cpu.stack_pointer);
    let address = memory.read_16_bit_value_at(cpu.program_counter + 1);
    memory.write_byte_at(address, lsb);
    memory.write_byte_at(address + 1, msb);

    debug!("LD SP into {:#06X}", address);
    cpu.program_counter += 3;
    cpu.cycle += 5;
}

/// Add the contents of register pair BC to the contents of register pair HL, and store the results
/// in register pair HL
pub fn add_hl_bc(cpu: &mut Cpu) {
    let bc = lsb_msb_to_u16(cpu.b, cpu.c);
    let hl = lsb_msb_to_u16(cpu.h, cpu.l);
    let result = bc + hl;
    let (lsb, msb) = u16_to_lsb_msb(result);

    cpu.half_carry = is_nth_bit_set_u16(hl, 15);
    cpu.carry = is_nth_bit_set_u16(hl, 15);
    cpu.h = lsb;
    cpu.l = msb;
    cpu.negative = false;

    debug!("ADD HL BC");
    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Load the 8-bit contents of memory specified by register pair BC into register A
pub fn ld_a_bc(cpu: &mut Cpu, memory: &Memory) {
    let address = lsb_msb_to_u16(cpu.b, cpu.c);
    let value = memory.read_byte_at(address);

    cpu.a = value;

    debug!("LD {:#06X} into A", value);
    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Decrement the contents of register pair BC by 1
pub fn dec_bc(cpu: &mut Cpu) {
    let value = lsb_msb_to_u16(cpu.b, cpu.c) - 1;
    let (lsb, msb) = u16_to_lsb_msb(value);

    cpu.b = lsb;
    cpu.c = msb;

    debug!("DEC register BC");
    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Increment the contents of register C by 1
pub fn inc_c(cpu: &mut Cpu) {
    cpu.half_carry = is_nth_bit_set_u8(cpu.c, 7);
    cpu.c += 1;
    cpu.zero = cpu.c == 0;
    cpu.negative = false;

    debug!("INC register C");
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Decrement the contents of register C by 1
pub fn dec_c(cpu: &mut Cpu) {
    cpu.half_carry = is_nth_bit_set_u8(cpu.c, 7);
    cpu.c -= 1;
    cpu.zero = cpu.c == 0;
    cpu.negative = true;

    debug!("DEC register C");
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the 8-bit immediate operand d8 into register C
pub fn ld_c_d8(cpu: &mut Cpu, memory: &Memory) {
    cpu.c = memory.read_byte_at(cpu.program_counter + 1);

    debug!("LD {:#06X} into register C", cpu.c);
    cpu.program_counter += 2;
    cpu.cycle += 2;
}

/// Load the 2 bytes of immediate data into register pair DE
pub fn ld_de_d16(cpu: &mut Cpu, memory: &Memory) {
    let value = memory.read_16_bit_value_at(cpu.program_counter + 1);
    let (lsb, msb) = u16_to_lsb_msb(value);

    cpu.d = lsb;
    cpu.e = msb;

    debug!("LD {:#06X} into register pair DE", value);
    cpu.program_counter += 3;
    cpu.cycle += 3;
}

/// Store the contents of register A in the memory location specified by register pair DE
pub fn ld_de_a(cpu: &mut Cpu, memory: &mut Memory) {
    let address = lsb_msb_to_u16(cpu.d, cpu.e);
    memory.write_byte_at(address, cpu.a);

    debug!("LD A into {:#06X}", address);
    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Increment the contents of register pair DE by 1
pub fn inc_de(cpu: &mut Cpu) {
    let value = lsb_msb_to_u16(cpu.d, cpu.e) + 1;
    let (lsb, msb) = u16_to_lsb_msb(value);

    cpu.d = lsb;
    cpu.e = msb;

    debug!("INC register pair DE");
    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Increment the contents of register D by 1
pub fn inc_d(cpu: &mut Cpu) {
    cpu.half_carry = is_nth_bit_set_u8(cpu.d, 7);
    cpu.d += 1;
    cpu.zero = cpu.d == 0;
    cpu.negative = false;

    debug!("INC register D");
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Decrement the contents of register D by 1
pub fn dec_d(cpu: &mut Cpu) {
    cpu.half_carry = is_nth_bit_set_u8(cpu.d, 7);
    cpu.d -= 1;
    cpu.zero = cpu.d == 0;
    cpu.negative = true;

    debug!("DEC register D");
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the 8-bit immediate operand d8 into register D
pub fn ld_d_d8(cpu: &mut Cpu, memory: &Memory) {
    cpu.d = memory.read_byte_at(cpu.program_counter + 1);

    debug!("LD {:#04X} into register D", cpu.d);
    cpu.program_counter += 2;
    cpu.cycle += 2;
}

/// Jump s8 steps from the current address in the program counter
pub fn jr_s8(cpu: &mut Cpu, memory: &Memory) {
    let relative_jump = u16::from(memory.read_byte_at(cpu.program_counter + 1));

    // Jump relative to the current program counter's address
    cpu.program_counter += relative_jump;

    debug!("JR to {:#06X}", cpu.program_counter);
    cpu.cycle += 3;
}

/// Add the contents of register pair DE to the contents of register pair HL, and store the results
/// in register pair HL
pub fn add_hl_de(cpu: &mut Cpu) {
    let de = lsb_msb_to_u16(cpu.d, cpu.e);
    let hl = lsb_msb_to_u16(cpu.h, cpu.l);
    let result = de + hl;
    let (lsb, msb) = u16_to_lsb_msb(result);

    cpu.half_carry = is_nth_bit_set_u16(hl, 15);
    cpu.carry = is_nth_bit_set_u16(hl, 15);
    cpu.h = lsb;
    cpu.l = msb;
    cpu.zero = result == 0;
    cpu.negative = false;

    debug!("ADD HL DE");
    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Load the 8-bit contents of memory specified by register pair DE into register A
pub fn ld_a_de(cpu: &mut Cpu, memory: &Memory) {
    let address = lsb_msb_to_u16(cpu.d, cpu.e);
    cpu.a = memory.read_byte_at(address);

    debug!("LD {:#06X} into A", cpu.a);
    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Decrement the contents of register pair DE by 1
pub fn dec_de(cpu: &mut Cpu) {
    let value = lsb_msb_to_u16(cpu.d, cpu.e) - 1;
    let (lsb, msb) = u16_to_lsb_msb(value);

    cpu.d = lsb;
    cpu.e = msb;

    debug!("DEC register pair DE");
    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Increment the contents of register E by 1
pub fn inc_e(cpu: &mut Cpu) {
    cpu.half_carry = is_nth_bit_set_u8(cpu.e, 7);
    cpu.e += 1;
    cpu.zero = cpu.e == 0;
    cpu.negative = false;

    debug!("INC register E");
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Decrement the contents of register E by 1
pub fn dec_e(cpu: &mut Cpu) {
    cpu.half_carry = is_nth_bit_set_u8(cpu.e, 7);
    cpu.e -= 1;
    cpu.zero = cpu.e == 0;
    cpu.negative = true;

    debug!("DEC register E");
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the 8-bit immediate operand d8 into register E
pub fn ld_e_d8(cpu: &mut Cpu, memory: &Memory) {
    cpu.e = memory.read_byte_at(cpu.program_counter + 1);

    dbg!("LD {:#04X} into register E", cpu.e);
    cpu.program_counter += 2;
    cpu.cycle += 2;
}

/// If the Z flag is 0, jump s8 steps from the current address stored in the program counter (PC).
/// If not, the instruction following the current JP instruction is executed (as usual).
pub fn jr_nz_s8(cpu: &mut Cpu, memory: &Memory) {
    if !cpu.zero {
        cpu.program_counter += u16::from(memory.read_byte_at(cpu.program_counter + 1));
        cpu.cycle += 3;

        debug!("JR NZ - jump to {:#06X}", cpu.program_counter);
    } else {
        debug!("JR NZ - no jump");
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

    debug!("LD {:#06X} into register pair HL", value);
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
    let (lsb, msb) = u16_to_lsb_msb(address_in_hl + 1);
    cpu.h = lsb;
    cpu.l = msb;

    debug!("LD HL+ A");

    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Increment the contents of register pair HL by 1
pub fn inc_hl(cpu: &mut Cpu) {
    let value = lsb_msb_to_u16(cpu.h, cpu.l) + 1;
    let (lsb, msb) = u16_to_lsb_msb(value);

    cpu.h = lsb;
    cpu.l = msb;

    debug!("INC register pair HL");
    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Increment the contents of register H by 1
pub fn inc_h(cpu: &mut Cpu) {
    cpu.half_carry = is_nth_bit_set_u8(cpu.h, 7);
    cpu.h += 1;
    cpu.zero = cpu.h == 0;
    cpu.negative = false;

    debug!("INC register H");
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Decrement the contents of register H by 1
pub fn dec_h(cpu: &mut Cpu) {
    cpu.half_carry = is_nth_bit_set_u8(cpu.h, 7);
    cpu.h -= 1;
    cpu.zero = cpu.h == 0;
    cpu.negative = true;

    debug!("DEC register H");
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the 8-bit immediate operand d8 into register H
pub fn ld_h_d8(cpu: &mut Cpu, memory: &Memory) {
    cpu.h = memory.read_byte_at(cpu.program_counter + 1);

    debug!("LD {:#04X} into register H", cpu.h);
    cpu.program_counter += 2;
    cpu.cycle += 2;
}

/// If the Z flag is 1, jump s8 steps from the current address stored in the program counter (PC).
/// If not, the instruction following the current JP instruction is executed (as usual).
pub fn jr_z_s8(cpu: &mut Cpu, memory: &Memory) {
    if cpu.zero {
        cpu.program_counter += u16::from(memory.read_byte_at(cpu.program_counter + 1));
        cpu.cycle += 3;

        debug!("JR Z - jump to {:#06X}", cpu.program_counter);
    } else {
        debug!("JR Z - no jump");
        cpu.program_counter += 2;
        cpu.cycle += 2;
    }
}

/// Add the contents of register pair HL to the contents of register pair HL, and store the results
/// in register pair HL
pub fn add_hl_hl(cpu: &mut Cpu) {
    let hl = lsb_msb_to_u16(cpu.h, cpu.l);
    let result = hl + hl;
    let (lsb, msb) = u16_to_lsb_msb(result);

    cpu.half_carry = is_nth_bit_set_u16(hl, 15);
    cpu.carry = is_nth_bit_set_u16(hl, 15);
    cpu.h = lsb;
    cpu.l = msb;
    cpu.zero = result == 0;
    cpu.negative = false;

    debug!("ADD HL HL");
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
    let (lsb, msb) = u16_to_lsb_msb(address_in_hl + 1);
    cpu.h = lsb;
    cpu.l = msb;

    debug!("LD A HL+");
    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Decrement the contents of register pair HL by 1
pub fn dec_hl(cpu: &mut Cpu) {
    let value = lsb_msb_to_u16(cpu.h, cpu.l) - 1;
    let (lsb, msb) = u16_to_lsb_msb(value);

    cpu.h = lsb;
    cpu.l = msb;

    debug!("DEC register pair HL");
    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Increment the contents of register L by 1
pub fn inc_l(cpu: &mut Cpu) {
    cpu.half_carry = is_nth_bit_set_u8(cpu.l, 7);
    cpu.l += 1;
    cpu.zero = cpu.l == 0;
    cpu.negative = false;

    debug!("INC register L");
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Decrement the contents of register L by 1
pub fn dec_l(cpu: &mut Cpu) {
    cpu.half_carry = is_nth_bit_set_u8(cpu.l, 7);
    cpu.l -= 1;
    cpu.zero = cpu.l == 0;
    cpu.negative = true;

    debug!("DEC register L");
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the 8-bit immediate operand d8 into register L
pub fn ld_l_d8(cpu: &mut Cpu, memory: &Memory) {
    cpu.l = memory.read_byte_at(cpu.program_counter + 1);

    dbg!("LD {:#04X} into register L", cpu.l);
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

        debug!("JR NC - jump to {:#06X}", cpu.program_counter);
    } else {
        debug!("JR NC - no jump");
        cpu.program_counter += 2;
        cpu.cycle += 2;
    }
}

/// Load the 2 bytes of immediate data into register pair SP
pub fn ld_sp_d16(cpu: &mut Cpu, memory: &Memory) {
    cpu.stack_pointer = memory.read_16_bit_value_at(cpu.program_counter + 1);

    debug!("LD {:#06X} into stack pointer", cpu.stack_pointer);
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
    let (lsb, msb) = u16_to_lsb_msb(address_in_hl - 1);
    cpu.h = lsb;
    cpu.l = msb;

    debug!("LD HL- A");

    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Increment the contents of register pair SP by 1
pub fn inc_sp(cpu: &mut Cpu) {
    cpu.stack_pointer += 1;

    debug!("INC stack pointer");
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

    debug!("INC value pointed to by HL");
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

    debug!("DEC value pointed to by HL");
    cpu.program_counter += 1;
    cpu.cycle += 3;
}

/// Store the contents of 8-bit immediate operand d8 in the memory location specified by register
/// pair HL
pub fn ld_hl_d8(cpu: &mut Cpu, memory: &mut Memory) {
    let value = memory.read_byte_at(cpu.program_counter + 1);
    let address = lsb_msb_to_u16(cpu.h, cpu.l);
    memory.write_byte_at(address, value);

    debug!("LD HL {:#06X}", value);
    cpu.program_counter += 2;
    cpu.cycle += 3;
}

/// Set the carry flag CY
pub fn scf(cpu: &mut Cpu) {
    cpu.carry = true;
    cpu.negative = false;
    cpu.half_carry = false;
    cpu.carry = true;

    debug!("SCF");
}

/// If the CY flag is 1, jump s8 steps from the current address stored in the program counter (PC).
/// If not, the instruction following the current JP instruction is executed (as usual).
pub fn jr_c_s8(cpu: &mut Cpu, memory: &Memory) {
    if cpu.carry {
        cpu.program_counter += u16::from(memory.read_byte_at(cpu.program_counter + 1));
        cpu.cycle += 3;

        debug!("JR C - jump to {:#06X}", cpu.program_counter);
    } else {
        debug!("JR C - no jump");
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

    let (lsb, msb) = u16_to_lsb_msb(value + cpu.stack_pointer);
    cpu.h = lsb;
    cpu.l = msb;

    debug!("ADD HL SP");
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

    debug!("LD A HL-");
    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Decrement the contents of register pair SP by 1
pub fn dec_sp(cpu: &mut Cpu) {
    cpu.stack_pointer -= 1;

    debug!("DEC SP");
    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Increment the contents of register A by 1
pub fn inc_a(cpu: &mut Cpu) {
    cpu.half_carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.a += 1;
    cpu.zero = cpu.a == 0;
    cpu.negative = false;

    debug!("INC A");
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Decrement the contents of register A by 1
pub fn dec_a(cpu: &mut Cpu) {
    cpu.half_carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.a -= 1;
    cpu.zero = cpu.a == 0;
    cpu.negative = false;

    debug!("DEC A");
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the 8-bit immediate operand d8 into register A
pub fn ld_a_d8(cpu: &mut Cpu, memory: &Memory) {
    cpu.a = memory.read_byte_at(cpu.program_counter + 1);

    debug!("LD {:#06X} into register A", cpu.a);
    cpu.program_counter += 2;
    cpu.cycle += 2;
}

/// Flip the carry flag CY
pub fn ccf(cpu: &mut Cpu) {
    cpu.negative = false;
    cpu.half_carry = false;
    cpu.carry = !cpu.carry;

    debug!("CCF");
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the contents of register B into register B
pub fn ld_b_b(cpu: &mut Cpu) {
    cpu.b = cpu.b;
    cpu.program_counter += 1;
    cpu.cycle += 1;

    debug!("LD B B");
}

/// Load the contents of register C into register B
pub fn ld_b_c(cpu: &mut Cpu) {
    cpu.b = cpu.c;
    cpu.program_counter += 1;
    cpu.cycle += 1;

    debug!("LD B C");
}

/// Load the contents of register D into register B
pub fn ld_b_d(cpu: &mut Cpu) {
    cpu.b = cpu.d;
    cpu.program_counter += 1;
    cpu.cycle += 1;

    debug!("LD B D");
}

/// Load the contents of register E into register B
pub fn ld_b_e(cpu: &mut Cpu) {
    cpu.b = cpu.e;
    cpu.program_counter += 1;
    cpu.cycle += 1;

    debug!("LD B E");
}

/// Load the contents of register H into register B
pub fn ld_b_h(cpu: &mut Cpu) {
    cpu.b = cpu.h;
    cpu.program_counter += 1;
    cpu.cycle += 1;

    debug!("LD B H");
}

/// Load the contents of register L into register B
pub fn ld_b_l(cpu: &mut Cpu) {
    cpu.b = cpu.l;
    cpu.program_counter += 1;
    cpu.cycle += 1;

    debug!("LD B L");
}

/// Load the 8-bit contents of memory specified by register pair HL into register B
pub fn ld_b_hl(cpu: &mut Cpu, memory: &Memory) {
    let address = lsb_msb_to_u16(cpu.h, cpu.l);
    cpu.b = memory.read_byte_at(address);
    cpu.program_counter += 1;
    cpu.cycle += 2;

    debug!("LD B {:04X}", cpu.b);
}

/// Load the contents of register A into register B
pub fn ld_b_a(cpu: &mut Cpu) {
    cpu.b = cpu.a;
    cpu.program_counter += 1;
    cpu.cycle += 1;

    debug!("LD B A");
}

/// Load the contents of register B into register C
pub fn ld_c_b(cpu: &mut Cpu) {
    cpu.c = cpu.b;
    cpu.program_counter += 1;
    cpu.cycle += 1;

    debug!("LD C B");
}

/// Load the contents of register C into register C
pub fn ld_c_c(cpu: &mut Cpu) {
    cpu.c = cpu.c;
    cpu.program_counter += 1;
    cpu.cycle += 1;

    debug!("LD C C");
}

/// Load the contents of register D into register C
pub fn ld_c_d(cpu: &mut Cpu) {
    cpu.c = cpu.d;
    cpu.program_counter += 1;
    cpu.cycle += 1;

    debug!("LD C D");
}

/// Load the contents of register E into register C
pub fn ld_c_e(cpu: &mut Cpu) {
    cpu.c = cpu.b;
    cpu.program_counter += 1;
    cpu.cycle += 1;

    debug!("LD C E");
}

/// Load the contents of register H into register C
pub fn ld_c_h(cpu: &mut Cpu) {
    cpu.c = cpu.b;
    cpu.program_counter += 1;
    cpu.cycle += 1;

    debug!("LD C H");
}

/// Load the contents of register L into register C
pub fn ld_c_l(cpu: &mut Cpu) {
    cpu.c = cpu.l;
    cpu.program_counter += 1;
    cpu.cycle += 1;

    debug!("LD C L");
}

/// Load the 8-bit contents of memory specified by register pair HL into register C
pub fn ld_c_hl(cpu: &mut Cpu, memory: &Memory) {
    let address = lsb_msb_to_u16(cpu.h, cpu.l);
    cpu.c = memory.read_byte_at(address);
    cpu.program_counter += 1;
    cpu.cycle += 2;

    debug!("LD C {:04X}", cpu.c);
}

/// Load the contents of register A into register C
pub fn ld_c_a(cpu: &mut Cpu) {
    cpu.c = cpu.a;
    cpu.program_counter += 1;
    cpu.cycle += 1;

    debug!("LD C A");
}

/// Load the contents of register B into register D
pub fn ld_d_b(cpu: &mut Cpu) {
    cpu.d = cpu.b;
    cpu.program_counter += 1;
    cpu.cycle += 1;

    debug!("LD D B");
}

/// Load the contents of register C into register D
pub fn ld_d_c(cpu: &mut Cpu) {
    cpu.d = cpu.c;
    cpu.program_counter += 1;
    cpu.cycle += 1;

    debug!("LD D C");
}

/// Load the contents of register D into register D
pub fn ld_d_d(cpu: &mut Cpu) {
    cpu.d = cpu.d;
    cpu.program_counter += 1;
    cpu.cycle += 1;

    debug!("LD D D");
}

/// Load the contents of register E into register D
pub fn ld_d_e(cpu: &mut Cpu) {
    cpu.d = cpu.e;
    cpu.program_counter += 1;
    cpu.cycle += 1;

    debug!("LD D E");
}

/// Load the contents of register H into register D
pub fn ld_d_h(cpu: &mut Cpu) {
    cpu.d = cpu.h;
    cpu.program_counter += 1;
    cpu.cycle += 1;

    debug!("LD D H");
}

/// Load the contents of register L into register D
pub fn ld_d_l(cpu: &mut Cpu) {
    cpu.d = cpu.l;
    cpu.program_counter += 1;
    cpu.cycle += 1;

    debug!("LD D L");
}

/// Load the 8-bit contents of memory specified by register pair HL into register D
pub fn ld_d_hl(cpu: &mut Cpu, memory: &Memory) {
    let address = lsb_msb_to_u16(cpu.h, cpu.l);
    cpu.d = memory.read_byte_at(address);
    cpu.program_counter += 1;
    cpu.cycle += 2;

    debug!("LD D {:04X}", cpu.b);
}

/// Load the contents of register A into register D
pub fn ld_d_a(cpu: &mut Cpu) {
    cpu.d = cpu.a;
    cpu.program_counter += 1;
    cpu.cycle += 1;

    debug!("LD D A");
}

/// Load the contents of register B into register E
pub fn ld_e_b(cpu: &mut Cpu) {
    cpu.e = cpu.b;
    cpu.program_counter += 1;
    cpu.cycle += 1;

    debug!("LD E B");
}

/// Load the contents of register C into register E
pub fn ld_e_c(cpu: &mut Cpu) {
    cpu.e = cpu.c;
    cpu.program_counter += 1;
    cpu.cycle += 1;

    debug!("LD E C");
}

/// Load the contents of register D into register E
pub fn ld_e_d(cpu: &mut Cpu) {
    cpu.e = cpu.d;
    cpu.program_counter += 1;
    cpu.cycle += 1;

    debug!("LD E D");
}

/// Load the contents of register E into register E
pub fn ld_e_e(cpu: &mut Cpu) {
    cpu.e = cpu.e;
    cpu.program_counter += 1;
    cpu.cycle += 1;

    debug!("LD E E");
}

/// Load the contents of register H into register E
pub fn ld_e_h(cpu: &mut Cpu) {
    cpu.e = cpu.h;
    cpu.program_counter += 1;
    cpu.cycle += 1;

    debug!("LD E H");
}

/// Load the contents of register L into register E
pub fn ld_e_l(cpu: &mut Cpu) {
    cpu.e = cpu.l;
    cpu.program_counter += 1;
    cpu.cycle += 1;

    debug!("LD E L");
}

/// Load the 8-bit contents of memory specified by register pair HL into register E
pub fn ld_e_hl(cpu: &mut Cpu, memory: &Memory) {
    let address = lsb_msb_to_u16(cpu.h, cpu.l);
    cpu.e = memory.read_byte_at(address);
    cpu.program_counter += 1;
    cpu.cycle += 2;

    debug!("LD E {:04X}", cpu.c);
}

/// Load the contents of register A into register E
pub fn ld_e_a(cpu: &mut Cpu) {
    cpu.e = cpu.a;
    cpu.program_counter += 1;
    cpu.cycle += 1;

    debug!("LD E A");
}

/// Load the contents of register B into register H
pub fn ld_h_b(cpu: &mut Cpu) {
    cpu.h = cpu.b;
    cpu.program_counter += 1;
    cpu.cycle += 1;

    debug!("LD H B");
}

/// Load the contents of register C into register H
pub fn ld_h_c(cpu: &mut Cpu) {
    cpu.h = cpu.c;
    cpu.program_counter += 1;
    cpu.cycle += 1;

    debug!("LD H C");
}

/// Load the contents of register D into register H
pub fn ld_h_d(cpu: &mut Cpu) {
    cpu.h = cpu.d;
    cpu.program_counter += 1;
    cpu.cycle += 1;

    debug!("LD H D");
}

/// Load the contents of register E into register H
pub fn ld_h_e(cpu: &mut Cpu) {
    cpu.h = cpu.e;
    cpu.program_counter += 1;
    cpu.cycle += 1;

    debug!("LD H E");
}

/// Load the contents of register H into register H
pub fn ld_h_h(cpu: &mut Cpu) {
    cpu.h = cpu.h;
    cpu.program_counter += 1;
    cpu.cycle += 1;

    debug!("LD H H");
}

/// Load the contents of register L into register H
pub fn ld_h_l(cpu: &mut Cpu) {
    cpu.h = cpu.l;
    cpu.program_counter += 1;
    cpu.cycle += 1;

    debug!("LD H L");
}

/// Load the 8-bit contents of memory specified by register pair HL into register H
pub fn ld_h_hl(cpu: &mut Cpu, memory: &Memory) {
    let address = lsb_msb_to_u16(cpu.h, cpu.l);
    cpu.h = memory.read_byte_at(address);
    cpu.program_counter += 1;
    cpu.cycle += 2;

    debug!("LD H {:04X}", cpu.b);
}

/// Load the contents of register A into register H
pub fn ld_h_a(cpu: &mut Cpu) {
    cpu.h = cpu.a;
    cpu.program_counter += 1;
    cpu.cycle += 1;

    debug!("LD H A");
}

/// Load the contents of register B into register L
pub fn ld_l_b(cpu: &mut Cpu) {
    cpu.l = cpu.b;
    cpu.program_counter += 1;
    cpu.cycle += 1;

    debug!("LD L B");
}

/// Load the contents of register C into register L
pub fn ld_l_c(cpu: &mut Cpu) {
    cpu.l = cpu.c;
    cpu.program_counter += 1;
    cpu.cycle += 1;

    debug!("LD L C");
}

/// Load the contents of register D into register L
pub fn ld_l_d(cpu: &mut Cpu) {
    cpu.l = cpu.d;
    cpu.program_counter += 1;
    cpu.cycle += 1;

    debug!("LD L D");
}

/// Load the contents of register E into register L
pub fn ld_l_e(cpu: &mut Cpu) {
    cpu.l = cpu.e;
    cpu.program_counter += 1;
    cpu.cycle += 1;

    debug!("LD L E");
}

/// Load the contents of register H into register L
pub fn ld_l_h(cpu: &mut Cpu) {
    cpu.l = cpu.h;
    cpu.program_counter += 1;
    cpu.cycle += 1;

    debug!("LD L H");
}

/// Load the contents of register L into register L
pub fn ld_l_l(cpu: &mut Cpu) {
    cpu.l = cpu.l;
    cpu.program_counter += 1;
    cpu.cycle += 1;

    debug!("LD L L");
}

/// Load the 8-bit contents of memory specified by register pair HL into register L
pub fn ld_l_hl(cpu: &mut Cpu, memory: &Memory) {
    let address = lsb_msb_to_u16(cpu.h, cpu.l);
    cpu.l = memory.read_byte_at(address);
    cpu.program_counter += 1;
    cpu.cycle += 2;

    debug!("LD L {:04X}", cpu.c);
}

/// Load the contents of register A into register L
pub fn ld_l_a(cpu: &mut Cpu) {
    cpu.l = cpu.a;
    cpu.program_counter += 1;
    cpu.cycle += 1;

    debug!("LD L A");
}

/// Store the contents of register B in the memory location specified by register pair HL
pub fn ld_hl_b(cpu: &mut Cpu, memory: &mut Memory) {
    let address = lsb_msb_to_u16(cpu.h, cpu.l);
    memory.write_byte_at(address, cpu.b);

    cpu.program_counter += 1;
    cpu.cycle += 2;

    debug!("LD HL {:#04X}", cpu.b);
}

/// Store the contents of register C in the memory location specified by register pair HL
pub fn ld_hl_c(cpu: &mut Cpu, memory: &mut Memory) {
    let address = lsb_msb_to_u16(cpu.h, cpu.l);
    memory.write_byte_at(address, cpu.c);

    cpu.program_counter += 1;
    cpu.cycle += 2;

    debug!("LD HL {:#04X}", cpu.c);
}

/// Store the contents of register D in the memory location specified by register pair HL
pub fn ld_hl_d(cpu: &mut Cpu, memory: &mut Memory) {
    let address = lsb_msb_to_u16(cpu.h, cpu.l);
    memory.write_byte_at(address, cpu.d);

    cpu.program_counter += 1;
    cpu.cycle += 2;

    debug!("LD HL {:#04X}", cpu.d);
}

/// Store the contents of register E in the memory location specified by register pair HL
pub fn ld_hl_e(cpu: &mut Cpu, memory: &mut Memory) {
    let address = lsb_msb_to_u16(cpu.h, cpu.l);
    memory.write_byte_at(address, cpu.e);

    cpu.program_counter += 1;
    cpu.cycle += 2;

    debug!("LD HL {:#04X}", cpu.e);
}

/// Store the contents of register H in the memory location specified by register pair HL
pub fn ld_hl_h(cpu: &mut Cpu, memory: &mut Memory) {
    let address = lsb_msb_to_u16(cpu.h, cpu.l);
    memory.write_byte_at(address, cpu.h);

    cpu.program_counter += 1;
    cpu.cycle += 2;

    debug!("LD HL {:#04X}", cpu.h);
}

/// Store the contents of register L in the memory location specified by register pair HL
pub fn ld_hl_l(cpu: &mut Cpu, memory: &mut Memory) {
    let address = lsb_msb_to_u16(cpu.h, cpu.l);
    memory.write_byte_at(address, cpu.l);

    cpu.program_counter += 1;
    cpu.cycle += 2;

    debug!("LD HL {:#04X}", cpu.l);
}

/// Store the contents of register A in the memory location specified by register pair HL
pub fn ld_hl_a(cpu: &mut Cpu, memory: &mut Memory) {
    let address = lsb_msb_to_u16(cpu.h, cpu.l);
    memory.write_byte_at(address, cpu.a);

    cpu.program_counter += 1;
    cpu.cycle += 2;

    debug!("LD HL {:#04X}", cpu.a);
}

/// Load the contents of register B into register A
pub fn ld_a_b(cpu: &mut Cpu) {
    cpu.a = cpu.b;

    debug!("LD register B into register A");
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the contents of register C into register A
pub fn ld_a_c(cpu: &mut Cpu) {
    cpu.a = cpu.c;

    debug!("LD register C into register A");
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the contents of register D into register A
pub fn ld_a_d(cpu: &mut Cpu) {
    cpu.a = cpu.d;

    debug!("LD register D into register A");
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the contents of register E into register A
pub fn ld_a_e(cpu: &mut Cpu) {
    cpu.a = cpu.e;

    debug!("LD register E into register A");
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the contents of register H into register A
pub fn ld_a_h(cpu: &mut Cpu) {
    cpu.a = cpu.h;

    debug!("LD register H into register A");
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the contents of register L into register A
pub fn ld_a_l(cpu: &mut Cpu) {
    cpu.a = cpu.l;

    debug!("LD register L into register A");
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the 8-bit contents of memory specified by register pair HL into register A
pub fn ld_a_hl(cpu: &mut Cpu, memory: &Memory) {
    let address = lsb_msb_to_u16(cpu.h, cpu.l);
    cpu.a = memory.read_byte_at(address);

    debug!("LD {:#04X} into register A", cpu.a);
    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Load the contents of register A into register A
pub fn ld_a_a(cpu: &mut Cpu) {
    cpu.a = cpu.a;

    debug!("LD register A into register A");
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Add the contents of register B to the contents of register A, and store the results in register
/// A
pub fn add_a_b(cpu: &mut Cpu) {
    cpu.half_carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.a += cpu.b;
    cpu.zero = cpu.a == 0;
    cpu.negative = false;

    debug!("ADD A B");
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Add the contents of register C to the contents of register A, and store the results in register
/// A
pub fn add_a_c(cpu: &mut Cpu) {
    cpu.half_carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.a += cpu.c;
    cpu.zero = cpu.a == 0;
    cpu.negative = false;

    debug!("ADD A C");
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Add the contents of register D to the contents of register A, and store the results in register
/// A
pub fn add_a_d(cpu: &mut Cpu) {
    cpu.half_carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.a += cpu.d;
    cpu.zero = cpu.a == 0;
    cpu.negative = false;

    debug!("ADD A D");
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Add the contents of register E to the contents of register A, and store the results in register
/// A
pub fn add_a_e(cpu: &mut Cpu) {
    cpu.half_carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.a += cpu.e;
    cpu.zero = cpu.a == 0;
    cpu.negative = false;

    debug!("ADD A E");
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Add the contents of register H to the contents of register A, and store the results in register
/// A
pub fn add_a_h(cpu: &mut Cpu) {
    cpu.half_carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.a += cpu.h;
    cpu.zero = cpu.a == 0;
    cpu.negative = false;

    debug!("ADD A H");
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Add the contents of register L to the contents of register A, and store the results in register
/// A
pub fn add_a_l(cpu: &mut Cpu) {
    cpu.half_carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.a += cpu.l;
    cpu.zero = cpu.a == 0;
    cpu.negative = false;

    debug!("ADD A L");
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Add the contents of memory specified by register pair HL to the contents of register A, and
/// store the results in register A
pub fn add_a_hl(cpu: &mut Cpu, memory: &Memory) {
    let address = lsb_msb_to_u16(cpu.h, cpu.l);

    cpu.half_carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.a += memory.read_byte_at(address);
    cpu.zero = cpu.a == 0;
    cpu.negative = false;

    debug!("ADD A HL");
    cpu.program_counter += 1;
    cpu.cycle += 2;
}

/// Add the contents of register A to the contents of register A, and store the results in register
/// A
pub fn add_a_a(cpu: &mut Cpu) {
    cpu.half_carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.carry = is_nth_bit_set_u8(cpu.a, 7);
    cpu.a += cpu.a;
    cpu.zero = cpu.a == 0;
    cpu.negative = false;

    debug!("ADD A A");
    cpu.program_counter += 1;
    cpu.cycle += 1;
}

/// Load the 16-bit immediate operand a16 into the program counter (PC)
pub fn jp_a16(cpu: &mut Cpu, memory: &Memory) {
    // Jump to the specified address
    cpu.program_counter = memory.read_16_bit_value_at(cpu.program_counter + 1);

    debug!("JP to {:#06X}", cpu.program_counter);
    cpu.cycle += 4;
}

/// Return from a subroutine
pub fn ret(cpu: &mut Cpu, memory: &mut Memory) {
    cpu.program_counter = memory.pop_stack_u16(cpu);

    debug!("RET to {:#06X}", cpu.program_counter);
    cpu.cycle += 4;
}

/// Call a subroutine
pub fn call_a16(cpu: &mut Cpu, memory: &mut Memory) {
    // Store return address on the stack
    let return_address_after_call = memory.read_16_bit_value_at(cpu.program_counter + 3);
    memory.push_stack_u16(return_address_after_call, cpu);

    // Go to the subroutine's address
    cpu.program_counter = memory.read_16_bit_value_at(cpu.program_counter + 1);

    debug!(
        "CALL {:#06X} (resume execution at: {:#06X})",
        cpu.program_counter, return_address_after_call
    );
    cpu.cycle += 6;
}

/// Store the contents of register A in the internal RAM, port register, or mode register at the
/// address in the range 0xFF00-0xFFFF specified by the 8-bit immediate operand a8
pub fn ld_a8_a(cpu: &mut Cpu, memory: &mut Memory) {
    let address = u16::from(memory.read_byte_at(cpu.program_counter + 1));
    memory.write_byte_at(address, cpu.a);

    debug!("LD {:#04X} into location {:#06X}", cpu.a, address);
    cpu.program_counter += 2;
    cpu.cycle += 3;
}

/// Store the contents of register A in the internal RAM or register specified by the 16-bit
/// immediate operand a16
pub fn ld_a16_a(cpu: &mut Cpu, memory: &mut Memory) {
    let address = memory.read_16_bit_value_at(cpu.program_counter + 1);
    memory.write_byte_at(address, cpu.a);

    debug!("LD register A into {:#06X}", address);
    cpu.program_counter += 3;
    cpu.cycle += 4;
}

/// Reset the interrupt master enable (IME) flag and prohibit maskable interrupts
pub fn di(cpu: &mut Cpu) {
    debug!("Disable interrupts");

    cpu.disable_interrupts = true;
    cpu.program_counter += 1;
    cpu.cycle += 4;
}
