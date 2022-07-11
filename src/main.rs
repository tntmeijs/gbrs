mod cartridge;

use std::env;

use cartridge::Cartridge;
use log::error;
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new()
        .init()
        .expect("Unable to initialize logger");

    let game_data = load_cartridge_from_cmdline_args();
}

/// Load a ROM file using the first command-line argument as a path to the ROM file
fn load_cartridge_from_cmdline_args() -> Cartridge {
    let args = env::args().collect::<Vec<String>>();

    // First command-line argumnet is always the executable itself, hence the length has to equal 2
    if args.len() != 2 {
        error!("Expected exactly 1 argument with a path to a rom file");
        return Cartridge::new_empty();
    }

    Cartridge::new_from_file(&args[1])
}
