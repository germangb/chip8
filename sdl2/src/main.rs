#![deny(unused_imports)]
#![deny(dead_code)]
#![deny(unused_must_use)]
#![deny(unused_variables)]
#![deny(unused_mut)]
use crate::opts::Opts;
use chip8::cpu::Cpu;
use imgui::MenuItem;
use log::{error, info, warn};
use std::{
    error::Error,
    fs,
    io::{self, Read},
};

mod opts;
mod sdl2_runner;
mod ui;

struct App {
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

    let program = load_program(&opts.rom)?;
    let mut app = App {
        display: true,
        keypad: true,
        debug: true,
        memory: true,
        registers: true,
    };

    sdl2_runner::run(Cpu::new(), |cpu, ui| {
        ui.main_menu_bar(|| {
            ui.menu(imgui::im_str!("App"), true, || {
                ui.checkbox(imgui::im_str!("Display"), &mut app.display);
                ui.checkbox(imgui::im_str!("Keypad"), &mut app.keypad);
                ui.checkbox(imgui::im_str!("Memory"), &mut app.memory);
                ui.checkbox(imgui::im_str!("Debug"), &mut app.debug);
                ui.checkbox(imgui::im_str!("Registers"), &mut app.registers);
            });
            ui.menu(imgui::im_str!("Rom"), true, || {
                if MenuItem::new(imgui::im_str!("Load")).build(ui) {
                    cpu.load(&program);
                }
                if MenuItem::new(imgui::im_str!("Reset")).build(ui) {
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

fn load_program(path: &Option<String>) -> io::Result<Box<[u8]>> {
    let mut rom: Box<dyn Read> = match path {
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
