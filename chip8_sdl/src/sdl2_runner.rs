use crate::opts::Opts;
use chip8::cpu::Cpu;
use imgui::{im_str, Ui, Window};
use log::{error, info};
use sdl2::audio::{AudioCallback, AudioStatus};
use std::{error::Error, ffi::CStr, ptr};

const SAMPLE_RATE: i32 = 44100;
const MAX_FREQ: i32 = 2000;

struct Wave {
    rate: i32,
    freq: i32,
    phase: i32,
}

impl AudioCallback for Wave {
    type Channel = f32;
    fn callback(&mut self, samples: &mut [Self::Channel]) {
        let volume = 0.5;
        for (i, sample) in samples.iter_mut().enumerate() {
            #[allow(non_snake_case)]
            let F = (self.freq as f32) / (self.rate as f32);
            let phase = 2.0 * 3.14159265 * F * (self.phase as usize + i) as f32;
            *sample = phase.cos() * volume;
        }
        self.phase += samples.len() as i32;
    }
}

pub fn run<F>(mut cpu: Cpu, mut closure: F) -> Result<(), Box<dyn Error>>
where
    F: FnMut(&mut Cpu, &Ui),
{
    let opts = Opts::from_args();
    let app = imgui_very_quick::builder()
        .position_centered()
        .background(0.5, 0.5, 0.5, 1.0)
        .build()?;

    let audio_spec = sdl2::audio::AudioSpecDesired {
        freq: Some(SAMPLE_RATE),
        channels: Some(1),
        samples: None,
    };
    let device = if !opts.no_sound {
        info!("initializing SDL audio device");
        let audio = app.sdl().audio()?;
        let device = audio.open_playback(None, &audio_spec, |spec| {
            info!("sampling rate = {}", spec.freq);
            info!("channels = {}", spec.channels);
            info!("format = {:?}", spec.format);
            info!("buffer size (samples) = {}", spec.samples);
            info!("size = {}", spec.size);
            let freq = (opts.beep_freq as i32).min(MAX_FREQ);
            info!("wave frequency = {}", freq);
            Wave {
                rate: spec.freq,
                freq,
                phase: 0,
            }
        });
        match device {
            Ok(device) => Some(device),
            Err(err) => {
                error!("failed to initialize audio device = {}", err);
                None
            }
        }
    } else {
        None
    };

    gl::load_with(|s| app.gl_get_proc_addr(s) as _);
    log_gl();

    let mut scale = 4.0;
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
            gl::TEXTURE_2D, 0, gl::R8 as _, 64, 32, 0, gl::RED, gl::UNSIGNED_BYTE, ptr::null());
        gl::BindTexture(gl::TEXTURE_2D, 0);
    }

    app.run(|ui| {
        // update audio
        if let Some(device) = &device {
            let st = cpu.sound_timer();
            match (st, device.status()) {
                (0, AudioStatus::Playing) => device.pause(),
                (_, AudioStatus::Paused) | (_, AudioStatus::Stopped) => {
                    if st > 0 {
                        device.resume()
                    }
                }
                _ => {}
            }
        }

        // update texture
        unsafe {
            let pixels = cpu.display().as_ptr();
            gl::BindTexture(gl::TEXTURE_2D, texture);
            #[rustfmt::skip]
                gl::TexSubImage2D(
                gl::TEXTURE_2D, 0, 0, 0, 64, 32, gl::RED, gl::UNSIGNED_BYTE, pixels as _);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        closure(&mut cpu, &ui);

        // texture window
        Window::new(im_str!("Display"))
            .always_auto_resize(true)
            .resizable(false)
            .build(&ui, || {
                imgui::Image::new(
                    imgui::TextureId::from(texture as usize),
                    [64.0 * scale, 32.0 * scale],
                )
                .border_col([1.0; 4])
                .build(&ui);

                let scales = [1.0, 2.0, 4.0, 8.0, 16.0];
                let label = [
                    im_str!("x1"),
                    im_str!("x2"),
                    im_str!("x4"),
                    im_str!("x8"),
                    im_str!("x16"),
                ];
                let [x, y] = ui.cursor_pos();
                for i in 0..4 {
                    ui.set_cursor_pos([x + 28.0 * (i as f32), y]);
                    if ui.button(label[i], [24.0, 24.0]) {
                        scale = scales[i];
                    }
                }
            });

        Ok(())
    })?;

    unsafe {
        gl::DeleteTextures(1, &mut texture);
    }

    Ok(())
}

fn log_gl() {
    unsafe {
        #[rustfmt::skip]
        info!("GL_VENDOR = {:?}", CStr::from_ptr(gl::GetString(gl::VENDOR) as *const i8));
        #[rustfmt::skip]
        info!("GL_RENDERER = {:?}", CStr::from_ptr(gl::GetString(gl::RENDERER) as *const i8));
        #[rustfmt::skip]
        info!("GL_VERSION = {:?}", CStr::from_ptr(gl::GetString(gl::VERSION) as *const i8));
        #[rustfmt::skip]
        info!("GL_SHADING_LANGUAGE_VERSION = {:?}", CStr::from_ptr(gl::GetString(gl::SHADING_LANGUAGE_VERSION) as *const i8));
    }
}
