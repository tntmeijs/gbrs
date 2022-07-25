mod cpu;
mod memory;
mod utility;

use std::env;

use cpu::cpu::Cpu;
use log::error;
use memory::{cartridge::Cartridge, memory::Memory};
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new()
        .init()
        .expect("Unable to initialize logger");

    // Initialize the system
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    let mut max_ticks = None;

    // Respect command-line argument flags
    let args = env::args().collect::<Vec<String>>();
    if args.len() > 1 {
        // Load the specified ROM into memory
        let game_data = Cartridge::new_from_file(&args[1]);
        memory.copy_into_memory_at_address(0u16, &game_data.data);

        // Optional argument: stop running after N ticks
        if args.len() > 2 {
            max_ticks = Some(
                args[2]
                    .parse::<u64>()
                    .expect("Maximum number of ticks is not a valid u64"),
            );
        }
    } else {
        error!("Expected the follow flags: <ROM PATH> <MAX NUM TICKS>");
    }

    // Start emulating
    let mut ticks = 0u64;
    loop {
        // Debug feature: stop emulating once the specified number of ticks have been hit
        if let Some(max_ticks) = max_ticks {
            if ticks >= max_ticks {
                break;
            }
        }

        cpu.tick(&mut memory);
        ticks += 1;
    }
}
