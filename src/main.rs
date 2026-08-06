const BAR_HEIGHT: u32 = 20;

use surface_init::*;

struct BarDrawerState {
    font: fontdue::Font,
    offset: usize,
    first_render: bool,
}

impl BarDrawerState {
    fn new() -> Self {
        Self {
            font: fontdue::Font::from_bytes(
                include_bytes!("/usr/share/fonts/0xProto/0xProtoNerdFont-Regular.ttf") as &[u8],
                fontdue::FontSettings::default(),
            )
            .unwrap(),
            offset: 0,
            first_render: true,
        }
    }
}

const PIXEL_SIZE: usize = 4;

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
        let text_start_x = 500;
        let (metrics, bitmap) = self.font.rasterize('1', 16.0);
        for line in 0..metrics.height {
            let line_start = (line * width + text_start_x) * PIXEL_SIZE;
            for i in 0..metrics.width {
                let idx = line_start + i * 4;
                buffer[idx] = bitmap[line * metrics.width + i];
                buffer[idx + 1] = bitmap[line * metrics.width + i];
                buffer[idx + 2] = bitmap[line * metrics.width + i];
                buffer[idx + 3] = 0xFF;
            }
        }
        self.offset = (self.offset + step) % width;
        if self.first_render {
            vec![Damage::from_0_0((width as u32, height as u32))]
        } else {
            vec![line_damage]
        }
    }
}

fn main() {
    init_bar_surface(BAR_HEIGHT, Box::new(BarDrawerState::new()));
}
