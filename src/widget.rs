use crate::{Damage, Vec2};
#[repr(C)]
pub(crate) struct Pixel {
    pub b: u8,
    pub g: u8,
    pub r: u8,
    pub a: u8,
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
    Absolute { pos: Vec2, size: Vec2 },
    Relative { pos: (f32, f32), size: (f32, f32) },
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
    pub(crate) fn for_widget<W: Widget + ?Sized>(widget: &W, dimensions: Vec2) -> Block {
        use crate::Positioning as P;
        //TODO: Actual layout stuff
        match widget.postioning() {
            P::LeftAlign { width } => Block {
                offset: Vec2 { x: 0, y: 0 },
                size: Vec2 {
                    x: width,
                    y: dimensions.y,
                },
            },
            _ => todo!(),
        }
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
        position += self.block.offset;
        assert!(position.x + len as u32 <= self.block.size.x);
        let idx = self.buf.index_from_position(position);
        &mut self.buf.buf[idx..idx + len]
    }
}
