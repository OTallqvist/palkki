use crate::DrawableBlock;
use crate::Positioning;
use crate::Widget;
use fontdue::layout::{CoordinateSystem, Layout};
use std::{
    fs::File,
    io::{Read, Seek},
};
use surface_init::*;
pub(super) struct Ram<'a> {
    layout: Layout,
    fonts: &'a [fontdue::Font],
    meminfo: File,
    total_ram: usize,
}

fn get_total_ram(meminfo: &mut File) -> usize {
    let mut buf = [0; 150];
    meminfo.read_exact(&mut buf).unwrap();
    meminfo.rewind().unwrap();
    let buf = str::from_utf8(buf.as_ref()).unwrap();
    let mut lines = buf.lines();
    parse_meminfo_line(lines.next().unwrap())
}

impl<'a> Ram<'a> {
    pub(super) fn new(fonts: &'a [fontdue::Font]) -> Self {
        let mut meminfo = File::open("/proc/meminfo").unwrap();
        let total_ram = get_total_ram(&mut meminfo);
        Self {
            total_ram,
            layout: Layout::new(CoordinateSystem::PositiveYUp),
            fonts,
            meminfo,
        }
    }
    ///Ram usage in GB
    fn get_ram_usage(&mut self) -> f32 {
        let mut buf = [0; 150];
        self.meminfo.read_exact(&mut buf).unwrap();
        self.meminfo.rewind().unwrap();
        let buf = str::from_utf8(buf.as_ref()).unwrap();
        let mut lines = buf.lines().skip(2);
        let avail_ram = parse_meminfo_line(lines.next().unwrap());
        (self.total_ram - avail_ram) as f32 / 1_000_000.0
    }
}

fn parse_meminfo_line(line: &str) -> usize {
    let mut number: String = line
        .chars()
        .skip_while(|c| *c != ' ')
        .skip_while(|c| *c == ' ')
        .collect();
    //number = "{actual_number} kB"
    number.truncate(number.len() - 3);
    number.parse::<usize>().unwrap()
}

impl Widget for Ram<'_> {
    fn postioning(&self) -> Positioning {
        Positioning::LeftAlign { width: 100 }
    }
    fn redraw(&mut self, block: &mut DrawableBlock) {
        for line in 0..block.block.size.y {
            let line = block.mutate_line(Vec2::new(0, line), block.block.size.x as usize);
            line.iter_mut().for_each(|px| px.g = 255);
        }
        block.damage = Damage::from_0_0(block.block.size)
    }
}
