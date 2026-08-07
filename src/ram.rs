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
    last_ram_usage: usize,
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
            last_ram_usage: 0,
            total_ram,
            layout: Layout::new(CoordinateSystem::PositiveYUp),
            fonts,
            meminfo,
        }
    }
    ///Ram usage in GB
    ///Returns None if the value didn't change since last update. And therefor the widget does not
    ///need to update anything
    fn get_ram_usage(&mut self) -> Option<f32> {
        let mut buf = [0; 150];
        self.meminfo.read_exact(&mut buf).unwrap();
        self.meminfo.rewind().unwrap();
        let buf = str::from_utf8(buf.as_ref()).unwrap();
        let mut lines = buf.lines().skip(2);
        let avail_ram = parse_meminfo_line(lines.next().unwrap());
        let usage = (self.total_ram - avail_ram) * 1000 / 1_048_576 / 8; //conversion to 8MiB
        //The conversion is done like this because ram usage is displayed with precision of 10MiB so
        //any smaller change won't matter. 8MiB is used specifically because the math is easier
        if self.last_ram_usage == usage {
            None
        } else {
            Some(usage as f32 / 128.) //conversion to GiB
        }
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

fn ram_usage_to_str(usage: f32) -> String {
    let str = format!("{:.2}Gi", usage);
    str
}

impl Widget for Ram<'_> {
    fn postioning(&self) -> Positioning {
        Positioning::LeftAlign { width: 60 }
    }
    fn redraw(&mut self, block: &mut DrawableBlock) {
        let Some(usage) = self.get_ram_usage() else {
            return;
        };
        let usage = ram_usage_to_str(usage);
        block.draw_str(&self.fonts[0], 14., &usage, Vec2 { x: 5, y: 5 });
        block.damage = Damage::from_0_0(block.block.size)
    }
}
