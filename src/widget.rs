use fontdue::layout::GlyphPosition;
use std::{
    mem,
    ops::{Mul, MulAssign},
    time::{Duration, Instant},
};
use surface_init::Vec2;

use crate::{Damage, TEXT_LAYOUT};

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Pixel {
    pub b: u8,
    pub g: u8,
    pub r: u8,
    pub a: u8,
}

///Does not multiply alpha values
impl<N> Mul<N> for Pixel
where
    Self: MulAssign<N>,
{
    type Output = Self;
    fn mul(mut self, rhs: N) -> Self::Output {
        self *= rhs;
        self
    }
}

impl MulAssign<f32> for Pixel {
    fn mul_assign(&mut self, rhs: f32) {
        self.r = (self.r as f32 * rhs).round() as u8;
        self.g = (self.g as f32 * rhs).round() as u8;
        self.b = (self.b as f32 * rhs).round() as u8;
    }
}

impl Pixel {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { b, g, r, a: 255 }
    }
    pub const WHITE: Self = Self::rgb(255, 255, 255);
}

pub(crate) struct WidgetData {
    widget: Box<dyn Widget>,
    block: Block,
    last_update: Instant,
    update_time: Duration,
}

impl WidgetData {
    ///Updates the widget if its update time has been exeeded
    fn update_widget(
        &mut self,
        damage_list: &mut Vec<Damage>,
        current_time: &Instant,
        buffer: &mut PixBuf,
    ) {
        if *current_time - self.last_update < self.update_time {
            return;
        }
        self.last_update = *current_time;
        let mut block = self.block.draw(buffer);
        self.widget.redraw(&mut block);
        damage_list.push(mem::take(&mut block.damage.offset(block.block.offset)));
    }
    pub(crate) fn position(&self) -> Vec2 {
        self.block.offset
    }
    pub(crate) fn size(&self) -> Vec2 {
        self.block.size
    }
    pub(crate) fn new(widget: Box<dyn Widget>, block: Block, update_time: Duration) -> Self {
        Self {
            widget,
            block,
            last_update: Instant::now() - Duration::from_secs(1000),
            update_time,
        }
    }
}

#[derive(Default)]
pub(crate) struct WidgetTable {
    pub left: Vec<WidgetData>,
    pub right: Vec<WidgetData>,
    pub absolute_back: Vec<WidgetData>,
    pub absolute_front: Vec<WidgetData>,
}

impl WidgetTable {
    pub fn draw_widgets(&mut self, buffer: &mut PixBuf) -> Vec<Damage> {
        let mut damage = vec![];
        let current_time = Instant::now();
        self.absolute_back
            .iter_mut()
            .chain(self.right.iter_mut())
            .chain(self.left.iter_mut())
            .chain(self.absolute_front.iter_mut())
            .for_each(|widget| {
                widget.update_widget(&mut damage, &current_time, buffer);
            });
        damage
    }
}

pub struct PixBuf<'a> {
    pub width: usize,
    pub height: usize,
    buf: &'a mut [Pixel],
}

impl<'a> PixBuf<'a> {
    pub fn new(buf: &'a mut [u8], width: usize, height: usize) -> Self {
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
    pub fn index_from_position(&self, position: Vec2) -> usize {
        self.width * position.y as usize + position.x as usize
    }
    pub fn mutate_line(&mut self, position: Vec2, len: usize) -> &mut [Pixel] {
        let idx = self.index_from_position(position);
        assert!(position.x as usize + len <= self.width);
        &mut self.buf[idx..idx + len]
    }
}

pub enum Positioning {
    LeftAlign {
        width: u32,
    },
    RightAlign {
        width: u32,
    },
    Absolute {
        offset: u32,
        width: u32,
        foreground: bool,
    },
}

pub trait Widget {
    fn redraw(&mut self, block: &mut DrawableBlock);
    ///How the widget wants to be placed
    fn postioning(&self, bar_size: Vec2) -> Positioning;
    ///Time between updates of this widget.
    ///The actual time between widget updates may be longer.
    ///Values below 100ms are not recomended.
    fn update_time(&self) -> Duration {
        Duration::from_millis(1000)
    }
}

#[derive(Debug)]
pub struct Block {
    pub offset: Vec2,
    pub size: Vec2,
}

impl<'b, 'buffer> Block {
    pub(super) fn new(offset: Vec2, size: Vec2) -> Self {
        Self { offset, size }
    }
    pub fn draw(&'b self, buf: &'b mut PixBuf<'buffer>) -> DrawableBlock<'b, 'buffer> {
        //Bounds checking assertions
        assert!(self.size.x + self.offset.x <= buf.width as u32);
        assert!(self.size.y + self.offset.y <= buf.height as u32);
        DrawableBlock {
            block: self,
            buf,
            damage: Damage::default(),
        }
    }
}

pub struct DrawableBlock<'b, 'buffer> {
    pub block: &'b Block,
    buf: &'b mut PixBuf<'buffer>,
    pub damage: Damage,
}

pub enum TextPosition {
    Absolute(Vec2),
    Center,
    Left { y: u32 },
    Right { y: u32 },
}

#[derive(Debug)]
pub enum BoundsError {
    YOverflow,
    YUnderflow,
    XOverflow,
    XUnderflow,
}

impl DrawableBlock<'_, '_> {
    ///Applies the blocks offset to the position. Returns Err if position is out of bounds.
    fn apply_offset_to(&self, position: &mut Vec2) -> Result<(), BoundsError> {
        if position.x >= self.width() {
            return Err(BoundsError::XOverflow);
        }
        if position.y >= self.height() {
            return Err(BoundsError::YOverflow);
        }
        *position += self.block.offset;
        Ok(())
    }
    pub fn width(&self) -> u32 {
        self.block.size.x
    }
    pub fn height(&self) -> u32 {
        self.block.size.y
    }
    pub fn mutate_line(
        &mut self,
        mut position: Vec2,
        len: usize,
    ) -> Result<&mut [Pixel], BoundsError> {
        if position.x + len as u32 > self.width() {
            return Err(BoundsError::XOverflow);
        }
        self.apply_offset_to(&mut position)?;
        let idx = self.buf.index_from_position(position);
        Ok(&mut self.buf.buf[idx..idx + len])
    }
    pub fn mutate_pixel(&mut self, mut position: Vec2) -> Result<&mut Pixel, BoundsError> {
        self.apply_offset_to(&mut position)?;
        let idx = self.buf.index_from_position(position);
        Ok(self.buf.buf.get_mut(idx).unwrap())
    }
    pub fn set_pixel(&mut self, position: Vec2, color: Pixel) -> Result<(), BoundsError> {
        *self.mutate_pixel(position)? = color;
        Ok(())
    }

    ///Supports monospace left to right text
    pub fn draw_text(
        &mut self,
        str: &str,
        size: f32,
        position: TextPosition,
        color: Pixel,
        //TODO:make the error useful
    ) -> Result<(), BoundsError> {
        let glyphs = TEXT_LAYOUT.shape_text(str, size, crate::FontType::Default);
        let position = match position {
            TextPosition::Center => {
                let text_width: u32 = glyphs
                    .last()
                    .map(|g| g.x as u32 + g.width as u32)
                    .unwrap_or(0);
                let text_height: u32 = glyphs.iter().map(|g| g.height as u32).max().unwrap_or(0);
                //The text layout system gives some vertical offset to the text. We should subtract it from our
                //position.y
                let text_y_offset: u32 = glyphs.iter().map(|g| g.y as u32).min().unwrap_or(0);
                let margin = (
                    self.width() as i32 - text_width as i32,
                    self.height() as i32 - text_height as i32,
                );
                Vec2 {
                    x: margin.0 as u32 / 2,
                    y: margin.1 as u32 / 2 - text_y_offset,
                }
            }
            TextPosition::Right { y } => {
                let text_width: u32 = glyphs.iter().map(|g| g.width as u32).sum();
                Vec2 {
                    x: self.width() - text_width,
                    y,
                }
            }
            TextPosition::Left { y } => Vec2::from_y(y),
            TextPosition::Absolute(position) => position,
        };
        self.draw_text_at(position, &glyphs, size, color)?;
        Ok(())
    }

    fn draw_text_at(
        &mut self,
        position: Vec2,
        glyphs: &[GlyphPosition],
        size: f32,
        color: Pixel,
        //TODO:make the error useful
    ) -> Result<(), BoundsError> {
        let mut res = Ok(());
        for glyph in glyphs {
            let (metrics, bitmap) = crate::FONTS[0].rasterize(glyph.parent, size);
            let stride = metrics.width;
            let char_pos = Vec2 {
                x: glyph.x as u32,
                y: glyph.y as u32,
            } + position;
            if char_pos.x + metrics.width as u32 > self.width() {
                res = Err(BoundsError::XOverflow);
            }
            for y in 0..metrics.height {
                if y as u32 + char_pos.y > self.height() {
                    res = Err(BoundsError::YOverflow);
                }
                let buf_line =
                    self.mutate_line(char_pos + Vec2::from_y(y as u32), metrics.width)?;
                buf_line.iter_mut().enumerate().for_each(|(x, px)| {
                    let bitmap_val = bitmap[stride * y + x];
                    if bitmap_val > 0 {
                        *px = color * (bitmap_val as f32 / 255.)
                    }
                });
            }
        }
        res
    }
    pub fn set_bg_color(&mut self, color: Pixel) {
        for line in 0..self.block.size.y {
            let line = self
                .mutate_line(Vec2::new(0, line), self.block.size.x as usize)
                .unwrap();
            line.iter_mut().for_each(|px| *px = color);
        }
    }
}
