#![deny(unused_imports)]
#![deny(dead_code)]
#![deny(unused_must_use)]
#![deny(unused_variables)]

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
    /// CPU running
    pub running: bool,
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
    let mut app = App { running: true };

    sdl2_runner::run(Cpu::new(), |cpu, ui| {
        if app.running {
            for _ in 0..opts.clock {
                cpu.step();
            }
        }

        if let Some(_key) = ui::keypad::draw(ui) {
            warn!("gui keypad not implemented yet");
        }

        ui.main_menu_bar(|| {
            ui.menu(imgui::im_str!("App"), true, || {
                ui.checkbox(imgui::im_str!("Running"), &mut app.running);
            });
        });

        ui::debug::draw(ui, cpu);
        ui::memory::draw(ui, cpu);

        ui::debug_gui(ui, cpu, &program);
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
