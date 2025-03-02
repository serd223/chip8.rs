use chip8::Chip8;
use framebrush::{Canvas, RGBu32, WHITE};
use minifb::{Key, Window, WindowOptions};
use std::time::{Instant, UNIX_EPOCH};

const DEFAULT_WIDTH: usize = 800;
const DEFAULT_HEIGHT: usize = 600;

fn main() {
    let mut buf = vec![0; DEFAULT_WIDTH * DEFAULT_HEIGHT];
    let mut chip8 = Chip8::new(Default::default());
    let file_path = {
        let mut args = std::env::args();
        let program = args.next().unwrap();
        if let Some(p) = args.next() {
            p
        } else {
            eprintln!("Usage: {program} <rom path>");
            return;
        }
    };
    let program = std::fs::read(&file_path).expect("file not found");
    chip8.set_program(&program);
    let mut window = Window::new(
        "chip8",
        DEFAULT_WIDTH,
        DEFAULT_HEIGHT,
        WindowOptions {
            resize: true,
            ..Default::default()
        },
    )
    .unwrap();

    let keys = [
        (Key::Key1, 0x1u8),
        (Key::Key2, 0x2),
        (Key::Key3, 0x3),
        (Key::Key4, 0xC),
        (Key::Q, 0x4),
        (Key::W, 0x5),
        (Key::E, 0x6),
        (Key::R, 0xD),
        (Key::A, 0x7),
        (Key::S, 0x8),
        (Key::D, 0x9),
        (Key::F, 0xE),
        (Key::Z, 0xA),
        (Key::X, 0x0),
        (Key::C, 0xB),
        (Key::V, 0xF),
    ];

    let mut last_frame = Instant::now();
    window.set_target_fps(144);
    while window.is_open() {
        let delta = {
            let now = Instant::now();
            let res = now.duration_since(last_frame).as_secs_f32();
            last_frame = now;
            res
        };
        let (width, height) = window.get_size();
        buf.resize(width * height, 0);
        let keypress = {
            let mut res = None;
            for (key, code) in keys {
                if window.is_key_down(key) {
                    res = Some(code);
                    chip8.keys[code as usize] = true;
                } else {
                    chip8.keys[code as usize] = false;
                }
            }
            res
        };

        if chip8.should_play_sound() {
            // TODO: Play sound here
        }

        chip8.frame(delta, keypress, || {
            (UNIX_EPOCH.elapsed().unwrap().as_micros() % 255) as u8
        });

        // Begin drawing
        let mut canvas = Canvas::new(&mut buf, (width, height), (Chip8::WIDTH, Chip8::HEIGHT));
        canvas.fill(0);
        for y in 0..Chip8::HEIGHT {
            for x in 0..Chip8::WIDTH {
                let color = if chip8.framebuffer[y * Chip8::WIDTH + x] {
                    WHITE
                } else {
                    RGBu32::Rgb(0, 0, 0)
                };
                canvas.rect(x as i32, y as i32, 1, 1, &color);
            }
        }
        // End drawing
        window.update_with_buffer(&buf, width, height).unwrap();
    }
}
