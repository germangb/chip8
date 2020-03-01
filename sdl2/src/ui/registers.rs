use chip8::cpu::Cpu;
use imgui::{im_str, Ui, Window};

pub fn draw(ui: &Ui, cpu: &mut Cpu) {
    Window::new(im_str!("Registers"))
        .always_auto_resize(true)
        .resizable(false)
        .build(ui, || {
            let i = cpu.i();
            let pc = cpu.program_counter();
            let sp = cpu.stack_pointer();
            let dt = cpu.delay_timer();
            let st = cpu.sound_timer();
            ui.label_text(im_str!("State"), &im_str!("{:?}", cpu.state()));
            ui.label_text(im_str!("Registers"), &im_str!("{:?}", cpu.registers()));
            ui.label_text(im_str!("I"), &im_str!("{:X?} ({})", i, i));
            ui.label_text(im_str!("PC"), &im_str!("{:X?} ({})", pc, pc));
            ui.label_text(im_str!("SP"), &im_str!("{:X?} ({})", sp, sp));
            ui.label_text(im_str!("DT"), &im_str!("{:X?} ({})", dt, dt));
            ui.label_text(im_str!("ST"), &im_str!("{:X?} ({})", st, dt));
            ui.label_text(im_str!("Stack"), &im_str!("{:?}", cpu.stack()));
        });
}
