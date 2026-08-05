const BAR_HEIGHT: u32 = 20;

use surface_init::*;

struct BarDrawerState {
    offset: usize,
}

impl BarDrawerState {
    fn new() -> Self {
        Self { offset: 0 }
    }
}

impl Drawer for BarDrawerState {
    fn draw(&mut self, buffer: &mut [u8], width: usize, height: usize) -> Vec<Damage> {
        for i in 0..(width * height) {
            let idx = i * 4;
            buffer[idx] = 0x30; // B
            buffer[idx + 1] = 0x30; // G
            buffer[idx + 2] = 0x30; // R
            buffer[idx + 3] = 0xFF; // A
        }
        let line_width = 10;
        for j in 0..line_width {
            for i in 0..height {
                let idx = (i * width + (self.offset + j) % width) * 4;
                buffer[idx] = 0xFF;
                buffer[idx + 1] = 0x00;
                buffer[idx + 2] = 0x00;
                buffer[idx + 3] = 0xFF;
            }
        }
        let step = 17;
        let line_damage = Damage::new(
            ((self.offset - step).min(width - step) as u32, 0),
            (line_width as u32 + step as u32, height as u32),
        );
        self.offset = (self.offset + step) % width;
        vec![line_damage]
    }
}

fn main() {
    init_bar_surface(BAR_HEIGHT, Box::new(BarDrawerState::new()));
}
