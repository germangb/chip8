use imgui::{im_str, Ui, Window};

/// Draw keypad gui
pub fn draw(ui: &Ui) -> Option<usize> {
    let mut pressed = None;
    Window::new(im_str!("Keypad"))
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
