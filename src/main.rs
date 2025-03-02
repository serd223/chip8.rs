use framebrush::{Canvas, RGBu32};
use minifb::{Window, WindowOptions};

const DEFAULT_WIDTH: usize = 800;
const DEFAULT_HEIGHT: usize = 600;
fn main() {
    let mut buf = vec![0; DEFAULT_WIDTH * DEFAULT_HEIGHT];

    let mut window = Window::new(
        "Hello, framebrush!",
        DEFAULT_WIDTH,
        DEFAULT_HEIGHT,
        WindowOptions {
            resize: true,
            ..Default::default()
        },
    )
    .unwrap();

    window.set_target_fps(144);
    while window.is_open() {
        let (width, height) = window.get_size();
        buf.resize(width * height, 0);

        // Begin drawing
        let mut canvas = Canvas::new(&mut buf, (width, height), (DEFAULT_WIDTH, DEFAULT_HEIGHT));
        canvas.fill(0);
        canvas.rect(10, 10, 30, 30, &RGBu32::Rgb(190, 96, 105));
        // End drawing
        window.update_with_buffer(&buf, width, height).unwrap();
    }
}
