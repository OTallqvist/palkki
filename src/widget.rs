use crate::{Damage, Vec2};
#[derive(Clone)]
#[repr(C)]
pub(crate) struct Pixel {
    pub b: u8,
    pub g: u8,
    pub r: u8,
    pub a: u8,
}

impl Pixel {
    pub(crate) fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { b, g, r, a: 255 }
    }
}

pub(crate) struct PixBuf<'a> {
    pub width: usize,
    pub height: usize,
    buf: &'a mut [Pixel],
}

impl<'a> PixBuf<'a> {
    pub(crate) fn new(buf: &'a mut [u8], width: usize, height: usize) -> Self {
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
    pub(crate) fn index_from_position(&self, position: Vec2) -> usize {
        self.width * position.y as usize + position.x as usize
    }
    pub(crate) fn mutate_line(&mut self, position: Vec2, len: usize) -> &mut [Pixel] {
        let idx = self.index_from_position(position);
        assert!(position.x as usize + len <= self.width);
        &mut self.buf[idx..idx + len]
    }
}

pub(crate) enum Positioning {
    LeftAlign { width: u32 },
    RightAlign { width: u32 },
    Center { width: u32 },
}

pub(crate) trait Widget {
    fn redraw(&mut self, block: &mut DrawableBlock);
    ///Returns how the widget wants to be placed
    fn postioning(&self) -> Positioning;
}

pub(crate) struct Block {
    pub(crate) offset: Vec2,
    pub(crate) size: Vec2,
}

impl<'b, 'buffer> Block {
    pub(super) fn new(offset: Vec2, size: Vec2) -> Self {
        Self { offset, size }
    }
    pub(crate) fn draw(&'b self, buf: &'b mut PixBuf<'buffer>) -> DrawableBlock<'b, 'buffer> {
        DrawableBlock {
            block: self,
            buf,
            damage: Damage::default(),
        }
    }
    pub(crate) fn width(&self) -> u32 {
        self.size.x
    }
    pub(crate) fn height(&self) -> u32 {
        self.size.y
    }
}

pub(crate) struct DrawableBlock<'b, 'buffer> {
    pub(crate) block: &'b Block,
    buf: &'b mut PixBuf<'buffer>,
    pub(crate) damage: Damage,
}

impl DrawableBlock<'_, '_> {
    pub(crate) fn mutate_line(&mut self, mut position: Vec2, len: usize) -> &mut [Pixel] {
        if !(position.x + len as u32 <= self.block.width()) {
            panic!("{} + {len} > {}", position.x, self.block.width())
        }
        position += self.block.offset;
        let idx = self.buf.index_from_position(position);
        &mut self.buf.buf[idx..idx + len]
    }
    pub(crate) fn vert_center(&self) -> u32 {
        self.block.size.y / 2
    }
    pub(crate) fn draw_str(&mut self, font: &fontdue::Font, size: f32, str: &str, position: Vec2) {
        let mut char_offset = 0;
        for char in str.chars() {
            let (metrics, bitmap) = font.rasterize(char, size);
            let stride = metrics.width;
            let char_pos = Vec2 {
                x: position.x + char_offset,
                y: position.y,
            };
            if char_pos.x + metrics.width as u32 > self.block.width() {
                break;
            }
            for y in 0..metrics.height {
                if y as u32 + char_pos.y > self.block.height() {
                    println!(
                        "broke at iteration {y} with coordinate {}",
                        char_pos.y + y as u32
                    );
                    break;
                }
                let buf_line = self.mutate_line(char_pos + Vec2::from_y(y as u32), metrics.width);
                buf_line.iter_mut().enumerate().for_each(|(x, px)| {
                    let bitmap_val = bitmap[stride * y + x];
                    *px = Pixel {
                        b: bitmap_val,
                        g: bitmap_val,
                        r: bitmap_val,
                        a: 255,
                    }
                });
            }
            char_offset += metrics.width as u32;
        }
    }
    pub(crate) fn set_bg_color(&mut self, color: Pixel) {
        for line in 0..self.block.size.y {
            let line = self.mutate_line(Vec2::new(0, line), self.block.size.x as usize);
            line.iter_mut().for_each(|px| *px = color.clone());
        }
    }
}
