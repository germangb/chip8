#![deny(unused_imports)]
#![deny(dead_code)]
#![deny(unused_must_use)]
#![deny(unused_variables)]
#![deny(unused_mut)]
use crate::{cpu::Cpu, opts::Opts};
use log::{error, info, warn};
use std::{
    error::Error,
    fs,
    io::{self, Read},
};

mod cpu;
mod opts;
mod sdl2_runner;
mod ui;

struct App {
    pub running: bool,
    pub speed: usize,
    pub display: bool,
    pub keypad: bool,
    pub debug: bool,
    pub memory: bool,
    pub registers: bool,
}

fn main() {
    env_logger::init();

    match run() {
        Ok(_) => {}
        Err(err) => {
            error!("error = {}", err);
            std::process::exit(1);
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let opts = Opts::from_args();
    let program = load_program()?;
    let mut app = App {
        running: true,
        speed: opts.clock,
        display: true,
        keypad: true,
        debug: true,
        memory: true,
        registers: true,
    };

    sdl2_runner::run(Cpu::new(), |cpu, ui| {
        if app.running {
            for _ in 0..app.speed {
                cpu.step();
            }
        }

        ui.main_menu_bar(|| {
            ui.menu(imgui::im_str!("App"), true, || {
                ui.checkbox(imgui::im_str!("Display"), &mut app.display);
                ui.checkbox(imgui::im_str!("Keypad"), &mut app.keypad);
                ui.checkbox(imgui::im_str!("Memory"), &mut app.memory);
                ui.checkbox(imgui::im_str!("Debug"), &mut app.debug);
                ui.checkbox(imgui::im_str!("Registers"), &mut app.registers);
            });
            ui.menu(imgui::im_str!("Chip-8"), true, || {
                let mut speed = app.speed as _;
                ui.checkbox(imgui::im_str!("Running"), &mut app.running);
                ui.input_int(imgui::im_str!("Speed"), &mut speed)
                    .step(1)
                    .build();
                app.speed = speed as _;
            });
            ui.menu(imgui::im_str!("Rom"), true, || {
                if ui.small_button(imgui::im_str!("Load")) {
                    cpu.load(&program);
                }
                if ui.small_button(imgui::im_str!("Halt")) {
                    cpu.halt();
                }
                if ui.small_button(imgui::im_str!("Reset")) {
                    cpu.reset();
                }
            });
        });

        if app.keypad {
            if let Some(_key) = ui::keypad::draw(ui) {
                warn!("gui keypad not implemented yet");
            }
        }
        if app.debug {
            ui::debug::draw(ui, cpu);
        }
        if app.memory {
            ui::memory::draw(ui, cpu);
        }
        if app.registers {
            ui::registers::draw(ui, cpu);
        }
    })?;
    Ok(())
}

fn load_program() -> io::Result<Box<[u8]>> {
    let opts = Opts::from_args();

    let mut rom: Box<dyn Read> = match opts.rom {
        None => {
            info!("read ROM from STDIN");
            Box::new(io::stdin())
        }
        Some(path) => {
            info!("opening ROM from file = {}", path);
            Box::new(fs::File::open(path)?)
        }
    };
    let mut program = Vec::new();
    match rom.read_to_end(&mut program) {
        Ok(bytes) => {
            info!("read {} bytes", bytes);
            Ok(program.into_boxed_slice())
        }
        Err(err) => Err(err),
    }
}
