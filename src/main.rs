use log::info;
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new()
        .init()
        .expect("Unable to initialize logger");

    info!("Hello, world!");
}
