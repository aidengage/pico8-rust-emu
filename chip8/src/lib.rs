use rand::rngs::SmallRng;
use rand::{RngCore, SeedableRng};

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
const MEM_SIZE: usize = 0x1000;
const ROM_ADDR: usize = 0x200;

#[derive(Clone, Copy)]
struct Reg(u8);

struct Regs([u8; 0x10]);

impl Regs {
    fn new() -> Self {
        Self([0; 0x10])
    }
}

impl Index<Reg> for Regs {
    type Output = u8;

    fn index(&self, reg: Reg) -> Self::Output {
        self.0[reg.0 as usize]
    }
}

impl IndexMut<Reg> for Regs {
    fn index_mut(&mut self, reg: Reg) -> &mut Self::Output {
        &mut self.0[reg.0 as usize]
    }
}

pub struct Chip8<R: RngCore> {
    mem: [u8; MEM_SIZE],
    v: Regs,
    i: u16,
    pc: u16,
    stack: [u16; 0x10],
    sp: u8,
    dt: u8,
    st: u8,
    keypad: u16,
    fb: [u8; SCREEN_WIDTH * SCREEN_HEIGHT / 8],
    tone: bool,
    time: isize,
    rng: R,
}

/// Op: Set Vx = Vx + b.
fn op_add(&mut self, x: Reg, b: u8) -> usize {
    let (res, overflow) = self.v[x].overflowing_add(b);
    self.v[x] = res;
    self.v[Reg(0xf)] = if overflow { 1 } else { 0 };
    self.pc += 2;
    45
}

fn op_sub(&mut self, x: Reg, )

fn main() {
    println!("Hello, world!");
}
