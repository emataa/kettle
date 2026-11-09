use std::fs::File;
use std::io::Read;
use std::env;
use kettle_core::*;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::keyboard::Keycode;
use sdl2::audio::{AudioCallback, AudioSpecDesired};

const SCALE: u32 = 15;
const WINDOW_WIDTH: u32 = (DISP_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (DISP_LEN as u32) * SCALE;
const TICKS_PER_FRAME: usize = 8;

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for sample in out.iter_mut() {
            *sample = if self.phase <= 0.5 { self.volume } else { -self.volume };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

fn draw_screen(emu: &Emu, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    let screen_buf = emu.get_display();
                // Now set draw color to white, iterate through each point and see if it should be drawn
    canvas.set_draw_color(Color::RGB(255, 255, 255));
     for (i, pixel) in screen_buf.iter().enumerate() {
         if *pixel {
            let x = (i % DISP_WIDTH) as u32;
            let y = (i / DISP_WIDTH) as u32;
            let rect = Rect::new((x * SCALE) as i32, (y  *SCALE) as i32, SCALE, SCALE);
            canvas.fill_rect(rect).unwrap();
            }
        }
    canvas.present();
}

fn key2button(key: Keycode) -> Option<usize>{
    match key{
        Keycode::Num1 =>    Some(0x1),
        Keycode::Num2 =>    Some(0x2),
        Keycode::Num3 =>    Some(0x3),
        Keycode::Num4 =>    Some(0xC),
        Keycode::Q =>       Some(0x4),
        Keycode::W =>       Some(0x5),
        Keycode::E =>       Some(0x6),
        Keycode::R =>       Some(0xD),
        Keycode::A =>       Some(0x7),
        Keycode::S =>       Some(0x8),
        Keycode::D =>       Some(0x9),
        Keycode::F =>       Some(0xE),
        Keycode::Z =>       Some(0xA),
        Keycode::X =>       Some(0x0),
        Keycode::C =>       Some(0xB),
        Keycode::V =>       Some(0xF),
        _ =>                None,        

    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2{
        println!("Usage: cargo run <game path>");
        return;
    }

    let sdl_context = sdl2::init().unwrap();
    let audio_subsystem = sdl_context.audio().unwrap();
    let spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1),
        samples: None,
    };
    let audio_device = audio_subsystem
        .open_playback(None, &spec, |spec| SquareWave {
            phase_inc: 440.0 / spec.freq as f32,
            phase: 0.0,
            volume: 0.15,
        })
        .unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("kettle: A Chip-8 Emulator", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut chip8 = Emu::new();
    let mut rom = File::open(&args[1]).expect("Unable to open file");
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer).unwrap();
    chip8.load(&buffer);
    'gameloop: loop {
        for evt in event_pump.poll_iter() {
            match evt {
                Event::Quit{..} | Event:: KeyDown{keycode: Some(Keycode::Escape), ..}=> {
                    break 'gameloop;
                },

                Event::KeyDown{keycode: Some(key), ..} => {
                    if let Some(k) = key2button(key) {
                        chip8.key_press(k, true);
                    }
                },
                Event::KeyUp{keycode: Some(key), ..} => {
                    if let Some(k) = key2button(key) {
                        chip8.key_press(k, false);
                    }
                },
                 _ => ()
            }
       }
       for _ in 0..TICKS_PER_FRAME{
        chip8.tick();
       }
        chip8.tick_timers();
        if chip8.sound_active() {
            audio_device.resume();
        } else {
            audio_device.pause();
        }
        draw_screen(&chip8, &mut canvas);
    }
}
            