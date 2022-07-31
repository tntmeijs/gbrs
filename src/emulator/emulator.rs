use super::{debugger::debugger::Debugger, gameboy::GameBoy};

/// GameBoy emulator and debugger
pub struct Emulator {
    gameboy: GameBoy,
    debugger: Debugger,
}

impl Emulator {
    /// Create a new Emulator
    pub fn new() -> Self {
        Self {
            gameboy: GameBoy::new(),
            debugger: Debugger::new(),
        }
    }

    /// Update the emulator and render the debugger
    fn update(&mut self, context: &egui::Context) {
        self.debugger.update(&mut self.gameboy);

        if self.debugger.gameboy_should_tick() {
            self.gameboy.tick();
        }

        self.debugger.draw(context, &mut self.gameboy);
    }
}

impl eframe::App for Emulator {
    /// Called whenever the UI should be repainted
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.update(ctx);
        ctx.request_repaint();
    }
}
