use crate::cpu::{Cpu, Opcode};
use imgui::{im_str, Ui, Window};
use lazy_static::lazy_static;
use std::sync::Mutex;

/// Draw debuger gui
pub fn draw(ui: &Ui, cpu: &mut Cpu) {
    lazy_static! {
        static ref OPCODE: Mutex<Option<Opcode>> = Mutex::new(None);
    }

    Window::new(im_str!("Debugger")).build(ui, || {
        if ui.small_button(im_str!("Step")) {
            cpu.step();
        }
        if ui.small_button(im_str!("Fetch instruction")) {
            let op = cpu.fetch();
            OPCODE.lock().unwrap().replace(op);
        }

        OPCODE
            .lock()
            .unwrap()
            .iter()
            .for_each(|op| ui.text(format!("{:?}", op)));
    });
}
