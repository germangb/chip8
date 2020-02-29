use crate::cpu::{Cpu, KeyState};
use std::io::{self, Read};

mod cpu;
mod sdl2_runner;

fn load_program() -> Box<[u8]> {
    let mut program = Vec::new();
    let read = io::stdin()
        .read_to_end(&mut program)
        .expect("Program please");
    program.into_boxed_slice()
}

fn main() {
    let mut cpu = Cpu::new();
    let program = load_program();

    sdl2_runner::run(cpu, |cpu, events, ui| {
        for _ in 0..4 {
            cpu.step();
        }

        use imgui::im_str;

        imgui::Window::new(im_str!("Debug"))
            .always_auto_resize(true)
            .resizable(false)
            .build(ui, || {
                ui.label_text(im_str!("State"), &im_str!("{:?}", cpu.state()));
                ui.label_text(im_str!("Registers"), &im_str!("{:?}", cpu.registers()));
                ui.label_text(im_str!("I"), &im_str!("{}", cpu.i()));
                ui.label_text(im_str!("PC"), &im_str!("{}", cpu.program_counter()));
                ui.label_text(im_str!("SP"), &im_str!("{}", cpu.stack_pointer()));
                ui.label_text(im_str!("Stack"), &im_str!("{:?}", cpu.stack()));
                if ui.small_button(im_str!("Load")) {
                    cpu.load(&program[..]);
                    eprintln!("Loaded program = {}B", program.len());
                }
                if ui.small_button(im_str!("Reset")) {
                    cpu.reset();
                }
                if ui.small_button(im_str!("Halt")) {
                    cpu.halt();
                }
            });
    });
}
