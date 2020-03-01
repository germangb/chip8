use chip8::cpu::PixelState;
use wasm_bindgen::prelude::*;

const DEMO: &[u8] = include_bytes!("../../roms/Trip8 Demo (2008) [Revival Studios].ch8");
const PONG: &[u8] = include_bytes!("../../roms/Pong (1 player).ch8");

#[wasm_bindgen]
pub struct Cpu(chip8::cpu::Cpu);

#[wasm_bindgen]
impl Cpu {
    pub fn new() -> Self {
        Self(chip8::cpu::Cpu::new())
    }

    pub fn update_timers(&mut self) {
        self.0.update_timers()
    }

    pub fn reset(&mut self) {
        self.0.reset();
    }

    pub fn display(&self) -> *const PixelState {
        self.0.display().as_ptr()
    }

    pub fn load_demo(&mut self) {
        self.0.load(DEMO)
    }

    pub fn load_pong(&mut self) {
        self.0.load(PONG)
    }

    pub fn halt(&mut self) {
        self.0.halt()
    }

    pub fn step(&mut self) {
        self.0.step()
    }

    pub fn program_counter(&self) -> usize {
        self.0.program_counter()
    }

    pub fn stack_pointer(&self) -> usize {
        self.0.stack_pointer()
    }

    pub fn delay_timer(&self) -> usize {
        self.0.delay_timer()
    }

    pub fn sound_timer(&self) -> usize {
        self.0.sound_timer()
    }

    pub fn i(&self) -> u16 {
        self.0.i()
    }
}
