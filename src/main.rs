mod ram;
mod widget;
const BAR_HEIGHT: u32 = 20;
use std::mem;

use surface_init::*;
use widget::*;

struct BarDrawerState<'a> {
    fonts: &'a [fontdue::Font],
    first_render: bool,
    widgets: Vec<(Box<dyn Widget>, Block)>,
}

impl<'a> BarDrawerState<'a> {
    fn new(fonts: &'a [fontdue::Font]) -> Self {
        Self {
            fonts,
            first_render: true,
            widgets: vec![],
        }
    }
    fn add_widget(&mut self, widget: Box<dyn Widget>, wl_data: &WlData) {
        let block = Block::for_widget(widget.as_ref(), wl_data.surface_size());
        self.widgets.push((widget, block));
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
        for (widget, block) in self.widgets.iter_mut() {
            let mut block = block.draw(&mut buffer);
            widget.redraw(&mut block);
            damage.push(mem::take(&mut block.damage));
        }
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
