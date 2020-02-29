#![deny(unused_imports)]
#![deny(dead_code)]
#![deny(unused_must_use)]

use crate::{
    cpu::{Cpu, KeyState},
    opts::Opts,
};
use log::{error, info, warn};
use sdl2::{event::Event as SdlEvent, keyboard::Keycode};
use sdl2_runner::Event;
use std::{
    error::Error,
    fs,
    io::{self, Read},
};
use structopt::StructOpt;

mod cpu;
mod opts;
mod sdl2_runner;

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

    sdl2_runner::run(Cpu::new(), |cpu, events, ui| {
        for _ in 0..opts.clock {
            cpu.step();
        }

        // for k in 0..=0xF {
        //     cpu.set_key(k, KeyState::Up);
        // }

        for event in events {
            if let Event::Sdl2(event) = event {
                handle_sdl_event(event, cpu);
            }
        }

        if let Some(k) = keypad_gui(ui) {
            warn!("gui keypad not implemented yet");
        }

        debug_gui(ui, cpu, &program);
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

fn handle_sdl_event(event: &SdlEvent, cpu: &mut Cpu) {
    match event {
        SdlEvent::KeyDown {
            keycode: Some(Keycode::Q),
            ..
        } => cpu.set_key(0x4, KeyState::Down),
        SdlEvent::KeyUp {
            keycode: Some(Keycode::Q),
            ..
        } => cpu.set_key(0x4, KeyState::Up),
        SdlEvent::KeyDown {
            keycode: Some(Keycode::Num1),
            ..
        } => cpu.set_key(0x1, KeyState::Down),
        SdlEvent::KeyUp {
            keycode: Some(Keycode::Num1),
            ..
        } => cpu.set_key(0x1, KeyState::Up),
        _ => {}
    }
}

fn debug_gui(ui: &imgui::Ui, cpu: &mut Cpu, program: &[u8]) {
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
            ui.label_text(im_str!("DT"), &im_str!("{}", cpu.delay_timer()));
            ui.label_text(im_str!("ST"), &im_str!("{}", cpu.sound_timer()));
            ui.label_text(im_str!("Stack"), &im_str!("{:?}", cpu.stack()));
            if ui.small_button(im_str!("Load")) {
                cpu.load(&program[..]);
            }
            if ui.small_button(im_str!("Reset")) {
                cpu.reset();
            }
            if ui.small_button(im_str!("Halt")) {
                cpu.halt();
            }
        });
}

fn keypad_gui(ui: &imgui::Ui) -> Option<usize> {
    use imgui::im_str;
    let mut pressed = None;
    imgui::Window::new(im_str!("Keypad"))
        .always_auto_resize(true)
        .resizable(false)
        .build(ui, || {
            let [x, y] = ui.cursor_pos();
            let labels = [
                [im_str!("1"), im_str!("2"), im_str!("3"), im_str!("C")],
                [im_str!("4"), im_str!("5"), im_str!("6"), im_str!("D")],
                [im_str!("7"), im_str!("8"), im_str!("9"), im_str!("E")],
                [im_str!("A"), im_str!("0"), im_str!("B"), im_str!("F")],
            ];
            let code = [
                [0x1, 0x2, 0x3, 0xC],
                [0x4, 0x5, 0x6, 0xD],
                [0x7, 0x8, 0x9, 0xE],
                [0xA, 0x0, 0xB, 0xF],
            ];
            let size = [24.0, 24.0];

            for i in 0..4 {
                for j in 0..4 {
                    ui.set_cursor_pos([x + 28.0 * (j as f32), y + 28.0 * (i as f32)]);
                    if ui.button(labels[i][j], size) {
                        pressed = Some(code[i][j]);
                    }
                }
            }
        });
    pressed
}
