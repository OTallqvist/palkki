use surface_init::{Damage, Vec2};

use crate::widget::{Pixel, Positioning, TextPosition, Widget};

pub(super) struct Clock {
    last_time: time::Time,
}

impl Clock {
    pub(super) fn new() -> Box<Self> {
        Box::new(Self {
            last_time: time::OffsetDateTime::now_local()
                .unwrap_or(time::OffsetDateTime::now_utc())
                .time(),
        })
    }
}

impl Widget for Clock {
    fn postioning(&self, bar_size: Vec2) -> crate::widget::Positioning {
        const WIDTH: u32 = 100;
        Positioning::Absolute {
            offset: bar_size.x / 2 - WIDTH / 2,
            width: WIDTH,
            foreground: true,
        }
    }
    fn redraw(&mut self, block: &mut crate::widget::DrawableBlock) {
        let time_now = time::OffsetDateTime::now_local().unwrap_or(time::OffsetDateTime::now_utc());
        if time_now.time() == self.last_time {
            return;
        }
        self.last_time = time_now.time();
        block.set_bg_color(Pixel::rgb(0x30, 0x30, 0x30));
        let mut time_str = time_now.time().truncate_to_second().to_string();
        //for some reason the string contains a ".0" at the end
        time_str.truncate(time_str.len() - 2);
        let _ = block.draw_text(&time_str, 12., TextPosition::Center);
        block.damage = Damage::from_0_0(block.block.size)
    }
}
