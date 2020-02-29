use crate::cpu::Cpu;
use imgui::{im_str, Ui, Window};

pub fn draw(ui: &Ui, cpu: &mut Cpu) {
    Window::new(im_str!("Registers"))
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
        });
}
