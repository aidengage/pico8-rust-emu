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

fn main() {
    println!("Hello, world!");
}
