mod ram;
mod widget;
const BAR_HEIGHT: u32 = 20;
use std::mem;

use surface_init::*;
use widget::*;

struct BarDrawerState<'a> {
    fonts: &'a [fontdue::Font],
    widgets: Box<WidgetTable>,
    first_render: bool,
}

#[derive(Default)]
struct WidgetTable {
    left: Vec<(Box<dyn Widget>, Block)>,
    right: Vec<(Box<dyn Widget>, Block)>,
    center: Vec<(Box<dyn Widget>, Block)>,
}

impl WidgetTable {
    fn draw_widgets(&mut self, buffer: &mut PixBuf) -> Vec<Damage> {
        let mut damage = vec![];
        for (widget, block) in self.right.iter_mut() {
            let mut block = block.draw(buffer);
            widget.redraw(&mut block);
            damage.push(mem::take(&mut block.damage.offset(block.block.offset)));
        }
        for (widget, block) in self.center.iter_mut() {
            let mut block = block.draw(buffer);
            widget.redraw(&mut block);
            damage.push(mem::take(&mut block.damage.offset(block.block.offset)));
        }
        for (widget, block) in self.left.iter_mut() {
            let mut block = block.draw(buffer);
            widget.redraw(&mut block);
            damage.push(mem::take(&mut block.damage.offset(block.block.offset)));
        }
        damage
    }
}

impl<'a> BarDrawerState<'a> {
    fn new(fonts: &'a [fontdue::Font]) -> Self {
        Self {
            fonts,
            first_render: true,
            widgets: Box::new(WidgetTable::default()),
        }
    }
    fn add_widget(&mut self, widget: Box<dyn Widget>, wl_data: &WlData) {
        use crate::Positioning as P;
        match widget.postioning() {
            P::RightAlign { width } => {
                let offset = Vec2::from_x(
                    self.widgets
                        .right
                        .last()
                        .map(|(_, block)| block.offset.x - width)
                        .unwrap_or(wl_data.surface_size().x - width),
                );
                self.widgets.right.push((
                    widget,
                    Block::new(offset, Vec2::new(width, wl_data.surface_size().y)),
                ));
            }
            P::LeftAlign { width } => {
                let offset = Vec2::from_x(
                    self.widgets
                        .left
                        .iter()
                        .map(|(_, block)| block.width())
                        .sum(),
                );
                self.widgets.left.push((
                    widget,
                    Block::new(offset, Vec2::new(width, wl_data.surface_size().y)),
                ));
            }
            _ => todo!(),
        };
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

impl Drawer for BarDrawerState<'_> {
    fn draw(&mut self, buffer: &mut [u8], width: usize, height: usize) -> Vec<Damage> {
        let mut buffer = PixBuf::new(buffer, width, height);
        let mut damage = vec![];
        draw_bg(&mut buffer);
        damage.append(&mut self.widgets.draw_widgets(&mut buffer));
        // let line_damage = draw_moving_line(&mut buffer, self);
        if self.first_render {
            self.first_render = false;
            vec![Damage::from_0_0(Vec2::new(width as u32, height as u32))]
        } else {
            damage
        }
    }
}

fn main() {
    let fonts = Box::leak(Box::new([fontdue::Font::from_bytes(
        include_bytes!("../fonts/0xproto-ascii.ttf") as &[u8],
        fontdue::FontSettings::default(),
    )
    .unwrap()]));
    let mut state = Box::new(BarDrawerState::new(fonts));
    let mut wl_data = init_bar_surface(BAR_HEIGHT);
    state.add_widget(Box::new(ram::Ram::new(state.fonts)), &wl_data);
    wl_data.render_loop(state);
}
