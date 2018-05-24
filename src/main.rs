extern crate sdl2;
extern crate rand;
//extern crate hex_slice;

mod cpu;
mod cartridge;
mod display;
mod font;
mod consts;
mod keypad;

use cpu::Cpu;
use cartridge::Cartridge;
use std::env;


fn main() {
    let sdl_context = sdl2::init().expect("Fatal error: Failed to initialize SDL");
    let args : Vec<String> = env::args().collect();
    let c_filename = &args[1];
    let cart = Cartridge::new(c_filename);
    let mut cpu = Cpu::new(&sdl_context);
    cpu.load(&cart.rom);
    loop {
        if !cpu.tick() {
            break;
        }
    }
}
