use chip8::cpu::PixelState;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Cpu(chip8::cpu::Cpu);

#[wasm_bindgen]
impl Cpu {
    pub fn new() -> Self {
        Self(chip8::cpu::Cpu::new())
    }

    pub fn reset(&mut self) {
        self.0.reset();
    }

    pub fn display(&self) -> *const PixelState {
        self.0.display().as_ptr()
    }
}
