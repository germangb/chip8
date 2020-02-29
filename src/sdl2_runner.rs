use crate::cpu::{Cpu, PixelState};
use imgui::Ui;
use std::{thread, time, time::Duration};

pub enum Event {
    Sdl2(sdl2::event::Event),
}

pub fn run<F>(mut cpu: Cpu, mut closure: F)
where
    F: FnMut(&mut Cpu, &[Event], &Ui),
{
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let window = video
        .window("CHIP-8", 640, 480)
        .opengl()
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let opengl_context = window.gl_create_context().unwrap();
    window.gl_make_current(&opengl_context).unwrap();

    gl::load_with(|s| video.gl_get_proc_address(s) as _);

    let mut imgui = imgui::Context::create();
    let mut imgui_sdl2 = imgui_sdl2::ImguiSdl2::new(&mut imgui, &window);
    let mut imgui_opengl =
        imgui_opengl_renderer::Renderer::new(&mut imgui, |s| video.gl_get_proc_address(s) as _);

    let mut event_pump = sdl.event_pump().unwrap();
    let mut events = Vec::new();

    let mut scale = 8.0;
    let mut pixels: Box<[u8]> = vec![0x0; 64 * 32 * 3].into_boxed_slice();
    let mut texture: gl::types::GLuint = 0;
    unsafe {
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as _);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as _);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as _);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as _);
        #[rustfmt::skip]
        gl::TexImage2D(
            gl::TEXTURE_2D, 0, gl::RGB8 as _, 64, 32, 0, gl::RGB, gl::UNSIGNED_BYTE, pixels.as_ptr() as _);
        gl::BindTexture(gl::TEXTURE_2D, 0);
    }

    'mainLoop: loop {
        for event in event_pump.poll_iter() {
            use sdl2::event::{Event as Sdl2Event, WindowEvent};
            if let Sdl2Event::Window {
                win_event: WindowEvent::Close,
                ..
            } = &event
            {
                break 'mainLoop;
            }
            if imgui_sdl2.ignore_event(&event) {
                imgui_sdl2.handle_event(&mut imgui, &event);
            } else {
                events.push(Event::Sdl2(event));
            }
        }

        imgui_sdl2.prepare_frame(imgui.io_mut(), &window, &event_pump.mouse_state());

        let ui = imgui.frame();

        closure(&mut cpu, &events[..], &ui);
        events.clear();

        // update texture
        cpu.display().iter().enumerate().for_each(|(i, p)| match p {
            PixelState::On => {
                pixels[3 * i + 0] = 0xFF;
                pixels[3 * i + 1] = 0xFF;
                pixels[3 * i + 2] = 0xFF;
            }
            PixelState::Off => {
                pixels[3 * i + 0] = 0x00;
                pixels[3 * i + 1] = 0x00;
                pixels[3 * i + 2] = 0x00;
            }
        });

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, texture);
            #[rustfmt::skip]
            gl::TexSubImage2D(
                gl::TEXTURE_2D, 0, 0, 0, 64, 32, gl::RGB, gl::UNSIGNED_BYTE, pixels.as_ptr() as _);
            gl::BindTexture(gl::TEXTURE_2D, 0);
            gl::ClearColor(0.5, 0.5, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // texture window
        #[rustfmt::skip]
        imgui::Window::new(imgui::im_str!("Display"))
            .always_auto_resize(true)
            .resizable(false)
            .build(&ui, || {
            imgui::Image::new(
                imgui::TextureId::from(texture as usize),
                [64.0 * scale, 32.0 * scale],
            )
            .border_col([1.0; 4])
            .build(&ui);
            if ui.small_button(imgui::im_str!("x1")) { scale = 1.0 }
            if ui.small_button(imgui::im_str!("x2")) { scale = 2.0 }
            if ui.small_button(imgui::im_str!("x4")) { scale = 4.0 }
            if ui.small_button(imgui::im_str!("x8")) { scale = 8.0 }
        });

        imgui_sdl2.prepare_render(&ui, &window);
        imgui_opengl.render(ui);

        window.gl_swap_window();
        thread::sleep(Duration::new(0, 1_000_000_000 / 60));
    }

    unsafe {
        gl::DeleteTextures(1, &mut texture);
    }
}
