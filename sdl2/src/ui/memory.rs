use chip8::cpu::Cpu;
use imgui::{im_str, StyleColor, Ui, Window};

fn byte_color(cpu: &Cpu, addr: u16, byte: u8) -> [f32; 4] {
    let pc = cpu.program_counter() as u16;
    match (addr, byte) {
        // pc & instruction
        (b, _) if b == pc || b == pc + 1 => [1.0, 0.0, 0.0, 1.0],
        (_, 0) => [0.5, 0.5, 0.5, 1.0],
        // interpreter
        (b, _) if b < 0x200 => [0.5, 0.5, 1.0, 1.0],
        _ => [1.0, 1.0, 1.0, 1.0],
    }
}

/// Draw the memory map gui
// TODO don't allocate so many dynamic strings
pub fn draw(ui: &Ui, cpu: &Cpu) {
    Window::new(im_str!("Memory"))
        .horizontal_scrollbar(true)
        .build(ui, || {
            let mem = cpu.memory();
            let [x, y] = ui.cursor_pos();
            let [cw, ch] = ui.calc_text_size(im_str!("0"), false, 0.0);
            let rows = 4098 / 16;
            for row in 0..rows {
                let v_offset = ch * row as f32;
                let token = ui.push_style_color(StyleColor::Text, [0.25, 0.25, 0.25, 1.0]);
                ui.set_cursor_pos([x, y + v_offset]);
                ui.text(format!("{:03X}", 0x10 * row));
                token.pop(ui);
                for i in 0..8 {
                    let h_offset = (cw * 5.0) + (i as f32 * cw * 3.0);
                    let addr = 16 * row + i;
                    let byte = mem[addr];
                    let token =
                        ui.push_style_color(StyleColor::Text, byte_color(cpu, addr as u16, byte));
                    ui.set_cursor_pos([x + h_offset, y + v_offset]);
                    ui.text(format!("{:02X}", byte));
                    token.pop(ui);
                }
                for i in 0..8 {
                    let h_offset = (cw * 5.0) + (cw * 3.0 * 8.0 + cw) + (i as f32 * cw * 3.0);
                    let addr = 16 * row + i + 8;
                    let byte = mem[addr];
                    let token =
                        ui.push_style_color(StyleColor::Text, byte_color(cpu, addr as u16, byte));
                    ui.set_cursor_pos([x + h_offset, y + v_offset]);
                    ui.text(format!("{:02X}", byte));
                    token.pop(ui);
                }
            }
        });
}
