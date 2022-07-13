use log::debug;

use crate::{cpu::Cpu, memory::Memory};

/// No operation
pub fn nop(cpu: &mut Cpu) {
    cpu.move_program_counter_by(1);
    cpu.add_cycles(1);
}

/// Jump to address
pub fn jp_a16(cpu: &mut Cpu, memory: &Memory) {
    // Read value next to the current instruction
    let address = memory.read_16_bit_value_at(cpu.get_program_counter() + 1);

    debug!("JP to {:#06x}", address);

    // Jump to the specified address
    cpu.set_program_counter(address);
    cpu.add_cycles(4);
}
