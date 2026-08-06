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

#[repr(C)]
struct Pixel {
    pub b: u8,
    pub g: u8,
    pub r: u8,
    pub a: u8,
}

struct PixBuf<'a> {
    width: usize,
    height: usize,
    buf: &'a mut [Pixel],
}

impl<'a> PixBuf<'a> {
    fn new(buf: &'a mut [u8], width: usize, height: usize) -> Self {
        assert!(buf.len().is_multiple_of(size_of::<Pixel>()));
        let ptr = buf.as_mut_ptr();
        assert!((ptr as usize).is_multiple_of(align_of::<Pixel>()));
        Self {
            width,
            height,
            buf: unsafe {
                std::slice::from_raw_parts_mut(ptr as *mut Pixel, buf.len() / size_of::<Pixel>())
            },
        }
    }
    fn index_from_position(&self, position: Vec2) -> usize {
        self.width * position.y as usize + position.x as usize
    }
    fn mutate_pixel(&mut self, position: Vec2) -> &mut Pixel {
        self.buf
            .get_mut(self.index_from_position(position))
            .unwrap_or_else(|| {
                panic!(
                    "Attempt at out of bounds write to pixel ({}, {})",
                    position.x, position.y
                )
            })
    }
}

fn draw_bg(buffer: &mut PixBuf) {
    for y in 0..buffer.height {
        for x in 0..buffer.width {
            *buffer.mutate_pixel(Vec2::new(x as u32, y as u32)) = Pixel {
                b: 0x00,
                g: 0x00,
                r: 0x00,
                a: 0xFF,
            }
        }
    }
}

fn draw_moving_line(buffer: &mut PixBuf, state: &mut BarDrawerState) -> Damage {
    let line_width = 10;
    for x in state.offset..state.offset + line_width {
        for y in 0..buffer.height {
            *buffer.mutate_pixel(Vec2::new(x as u32, y as u32)) = Pixel {
                b: 0xFF,
                g: 0x00,
                r: 0x00,
                a: 0xFF,
            };
        }
    }
    let step = 17;
    let line_damage = Damage::new(
        ((state.offset - step).min(buffer.width - step) as u32, 0),
        (line_width as u32 + step as u32, buffer.height as u32),
    );
    state.offset = (state.offset + step) % buffer.width;
    line_damage
}

impl Drawer for BarDrawerState {
    fn draw(&mut self, buffer: &mut [u8], width: usize, height: usize) -> Vec<Damage> {
        let mut buffer = PixBuf::new(buffer, width, height);
        draw_bg(&mut buffer);
        let line_damage = draw_moving_line(&mut buffer, self);
        if self.first_render {
            self.first_render = false;
            vec![Damage::from_0_0((width as u32, height as u32))]
        } else {
            vec![line_damage]
        }
    }
}

fn main() {
    init_bar_surface(BAR_HEIGHT, Box::new(BarDrawerState::new()));
}
