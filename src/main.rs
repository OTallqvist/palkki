const BAR_HEIGHT: u32 = 20;
use surface_init::*;

struct BarDrawerState {
    counter: usize,
}

impl BarDrawerState {
    fn new() -> Self {
        Self { counter: 0 }
    }
}

impl Drawer for BarDrawerState {
    fn draw(&mut self, buffer: &mut [u8], width: usize, height: usize) -> Vec<Damage> {
        for i in 0..(width * height) {
            let idx = i * 4;
            if self.counter.is_multiple_of(3) {
                buffer[idx] = 0xFF; // B
                buffer[idx + 1] = 0x30; // G
                buffer[idx + 2] = 0x30; // R
                buffer[idx + 3] = 0xFF; // A
            } else {
                buffer[idx] = 0x30; // B
                buffer[idx + 1] = 0x30; // G
                buffer[idx + 2] = 0x30; // R
                buffer[idx + 3] = 0xFF; // A
            }
        }
        self.counter += 1;
        println!("updated {} times \r", self.counter);
        vec![Damage::full()]
    }
}

fn main() {
    init_bar_surface(BAR_HEIGHT, Box::new(BarDrawerState::new()));
}
