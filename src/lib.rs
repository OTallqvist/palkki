pub mod widget;
pub use common::{Rect, Vec2};
use std::{
    sync::{LazyLock, Mutex},
    thread,
    time::Duration,
};

use fontdue::layout::{GlyphPosition, TextStyle};
use surface_init::*;
use widget::*;

pub static FONTS: LazyLock<Box<[fontdue::Font]>> = LazyLock::new(|| {
    Box::new([fontdue::Font::from_bytes(
        include_bytes!("../fonts/0xproto-ascii.ttf") as &[u8],
        fontdue::FontSettings::default(),
    )
    .unwrap()])
});

pub struct TextLayout {
    layout: Mutex<fontdue::layout::Layout>,
}

pub enum FontType {
    Default,
    Bold,
    Italic,
}

impl TextLayout {
    pub fn shape_text(&self, text: &str, px: f32, _font: FontType) -> Vec<GlyphPosition> {
        let mut layout = self.layout.lock().unwrap();
        let font_index = 0;
        layout.append(FONTS.as_ref(), &TextStyle::new(text, px, font_index));
        let glyphs = layout.glyphs().clone();
        layout.clear();
        glyphs
    }
    fn new() -> Self {
        Self {
            layout: Mutex::new(fontdue::layout::Layout::new(
                fontdue::layout::CoordinateSystem::PositiveYDown,
            )),
        }
    }
}

pub static TEXT_LAYOUT: LazyLock<TextLayout> = LazyLock::new(TextLayout::new);

pub struct Bar {
    widgets: Box<WidgetTable>,
    wl_data: WlData,
    first_render: bool,
    update_time: Duration,
}

impl Bar {
    pub fn with_height(height: u32) -> Self {
        Self {
            first_render: true,
            wl_data: init_bar_surface(height),
            widgets: Box::new(WidgetTable::default()),
            update_time: Duration::from_millis(1000),
        }
    }
    pub fn add_widgets(&mut self, widgets: &[&dyn Fn() -> Box<dyn Widget>]) {
        widgets.iter().for_each(|widget| self.add_widget(widget()));
    }
    pub fn add_widget(&mut self, widget: Box<dyn Widget>) {
        use crate::Positioning as P;
        let update_time = widget.update_time();
        if update_time > Duration::from_millis(100) && update_time < self.update_time {
            self.update_time = update_time;
        }
        let positioning = widget.postioning(self.wl_data.surface_size());
        let widget = match positioning {
            P::RightAlign { width } => {
                let offset = Vec2::from_x(
                    self.widgets
                        .right
                        .last()
                        .map(|widget| widget.position().x - width)
                        .unwrap_or(self.wl_data.surface_size().x - width),
                );
                WidgetData::new(
                    widget,
                    Block::new(offset, Vec2::new(width, self.height())),
                    update_time,
                )
            }
            P::LeftAlign { width } => {
                let offset =
                    Vec2::from_x(self.widgets.left.iter().map(|widget| widget.size().x).sum());
                WidgetData::new(
                    widget,
                    Block::new(offset, Vec2::new(width, self.height())),
                    update_time,
                )
            }
            P::Absolute {
                offset,
                width,
                foreground: _,
            } => WidgetData::new(
                widget,
                Block::new(
                    Vec2::from_x(offset),
                    Vec2 {
                        x: width,
                        y: self.wl_data.surface_size().y,
                    },
                ),
                update_time,
            ),
        };
        self.widgets.insert_widget(&positioning, widget);
    }
    pub fn run(mut self) {
        let width = self.width() as usize;
        let height = self.height() as usize;
        let buffer = self.wl_data.frame_buffer();
        let mut buffer = PixBuf::new(buffer, width, height);
        draw_bg(&mut buffer);
        loop {
            self.wl_data.dispatch_pending().unwrap();
            let damage = self.draw(width, height);
            self.wl_data.update_surface(damage);
            thread::sleep(self.update_time);
        }
    }
    pub fn width(&self) -> u32 {
        self.wl_data.surface_size().x
    }
    pub fn height(&self) -> u32 {
        self.wl_data.surface_size().y
    }
    fn draw(&mut self, width: usize, height: usize) -> Vec<Rect> {
        let buffer = self.wl_data.frame_buffer();
        let mut buffer = PixBuf::new(buffer, width, height);
        let mut damage = vec![];
        damage.append(&mut self.widgets.draw_widgets(&mut buffer));
        if self.first_render {
            self.first_render = false;
            vec![Rect::from_0_0(Vec2::new(width as u32, height as u32))]
        } else {
            damage
        }
    }
}

fn draw_bg(buffer: &mut PixBuf) {
    for y in 0..buffer.height {
        let line = buffer.mutate_line(Vec2::new(0, y as u32), buffer.width);
        for px in line.iter_mut() {
            *px = Pixel {
                b: 0x00,
                g: 0x00,
                r: 0x00,
                a: 0xFF,
            }
        }
    }
}
