use chip8::cpu::PixelState;
use wasm_bindgen::prelude::*;

const ROM: &[u8] = include_bytes!("../../roms/Trip8 Demo (2008) [Revival Studios].ch8");

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

    pub fn load(&mut self) {
        self.0.load(ROM)
    }

    pub fn halt(&mut self) {
        self.0.halt()
    }

    pub fn step(&mut self) {
        self.0.step()
    }
}
