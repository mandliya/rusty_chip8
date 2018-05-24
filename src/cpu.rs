use rand;
use rand::Rng;
use display::Display;
use keypad::KeyPad;
use std::thread;
use std::time::Duration;
use font::FONT_SET;
use consts::EXECUTION_DELAY;
use sdl2;
extern crate hex_slice;
//use hex_slice::AsHex;

pub struct Cpu {
    opcode: u16,
    v_ram: [u8; 4096],
    v: [u8; 16],
    i: usize,
    pc: usize,
    stack: [u16; 16],
    sp: usize,
    delay_timer: u8,
    sound_timer: u8,
    pub screen : Display,
    pub key_pad: KeyPad,
}

impl Cpu {
    pub fn new(sdl_context: &sdl2::Sdl) -> Cpu {
        let mut cpu = Cpu {
            opcode: 0,
            v_ram: [0; 4096],
            v: [0; 16],
            i: 0x200,
            pc: 0x200,
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            screen: Display::new(sdl_context),
            key_pad: KeyPad::new(sdl_context),
        };
        for i in 0..80 {
            cpu.v_ram[i] = FONT_SET[i];
        }
        cpu
    }

    fn fetch_opcode(&mut self) {
        self.opcode = (self.v_ram[self.pc] as u16) << 8 |
            (self.v_ram[self.pc+1] as u16);
    }

    fn not_implemented(&mut self) {
        println!("Not implemented opcode: 0x{:X}, pc: 0x{:X}", self.opcode, self.pc);
    }

    fn exec_opcode(&mut self) {
        // println!("--------------------------");
        // println!("Opcode: {:x}", self.opcode);
        // println!("Match: {:x}", self.opcode & 0xf000);
        // println!("v: {:02X}", self.v.as_hex());
        // println!("---------------------------");
        match self.opcode & 0xf000 {
            0x0000 => self.op_0xxx(),
            0x1000 => self.op_1xxx(),
            0x2000 => self.op_2xxx(),
            0x3000 => self.op_3xxx(),
            0x4000 => self.op_4xxx(),
            0x5000 => self.op_5xxx(),
            0x6000 => self.op_6xxx(),
            0x7000 => self.op_7xxx(),
            0x8000 => self.op_8xxx(),
            0x9000 => self.op_9xxx(),
            0xA000 => self.op_axxx(),
            0xB000 => self.op_bxxx(),
            0xC000 => self.op_cxxx(),
            0xD000 => self.op_dxxx(),
            0xE000 => self.op_exxx(),
            0xF000 => self.op_fxxx(),
            _ => self.not_implemented()
        }
    }

    fn op_x(&self) -> usize {
        ((self.opcode & 0x0F00) >> 8) as usize
    }

    fn op_y(&self) -> usize {
        ((self.opcode & 0x00F0) >> 4) as usize
    }

    fn op_n(&self) -> u8 {
        ((self.opcode & 0x000F)) as u8
    }

    fn op_nn(&self) -> u8 {
        ((self.opcode & 0x00FF)) as u8
    }

    fn op_nnn(&self) -> u16 {
        self.opcode & 0x0FFF
    }

    fn op_0xxx(&mut self) {
        match self.opcode & 0x000F {
            0x0000 => {
                self.screen.clear()
            }
            0x000E => {
                self.sp -= 1;
                self.pc = self.stack[self.sp] as usize;
            }
            _ => { 
                self.not_implemented()
            }
        }
        self.pc += 2;
    }

    // jump to location nnn
    fn op_1xxx(&mut self) {
        self.pc = self.op_nnn() as usize;
    }

    // calls subroutine
    fn  op_2xxx(&mut self) {
        self.stack[self.sp] = self.pc as u16;
        self.sp += 1;
        self.pc = self.op_nnn() as usize;
    }

    // skips the next instruction if vx == NN
    fn op_3xxx(&mut self) {
        self.pc += if self.v[self.op_x()] == self.op_nn() {
            4
        } else {
            2
        }
    }

    // skips the next instruction if vx != NN
    fn op_4xxx(&mut self) {
        self.pc += if self.v[self.op_x()] != self.op_nn() {
            4
        } else {
            2
        }
    }

    // skips the next instruction if vx == vy
    fn op_5xxx(&mut self) {
        self.pc += if self.v[self.op_x()] == self.v[self.op_y()] {
            4
        } else {
            2
        }
    }

    // puts the value NN into register Vx.
    fn op_6xxx(&mut self) {
        self.v[self.op_x()] = self.op_nn();
        self.pc += 2;
    }

    // Adds the value NN to the value of register Vx, then stores the result in Vx.
    fn op_7xxx(&mut self) {
        let vx = self.v[self.op_x()] as u16;
        let val = self.op_nn() as u16;
        let result = vx + val;
        self.v[self.op_x()] = result as u8;
        self.pc += 2;
    }

    
    fn op_8xxx(&mut self) {
        match self.opcode & 0x00F {
            0x0 => {
                // 0x8xy0 Stores the value of register Vy in register Vx.
                 self.v[self.op_x()] = self.v[self.op_y()];
            },

            0x1 => {
                // 0x8xy1 Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx.
                self.v[self.op_x()] |= self.v[self.op_y()];
            },

            0x2 => {
                // 0x8xy2 Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx
                self.v[self.op_x()] &= self.v[self.op_y()];
            },

            0x3 => {
                // 0x8xy3 Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx
                self.v[self.op_x()] ^= self.v[self.op_y()];
            },

            0x4 => {
                // 0x8xy4 Vx = Vx + Vy, set VF = carry.
                // println!("op_x:{:X}, op_y:{:X}", self.op_x(), self.op_y());
                // println!("v[self.op_x()]:{:X}, v[self.op_y()]:{:X}", self.v[self.op_x()], self.v[self.op_y()]);
                let vx = self.v[self.op_x()] as u16;
                let vy = self.v[self.op_y()] as u16;
                let result = vx + vy;
                self.v[self.op_x()] = result as u8;
                self.v[0x0F]= if self.v[self.op_x()] < self.v[self.op_y()] {
                    1
                } else {
                    0
                }
            },

            0x005 => {
                // 0x8xy5 Set Vx = Vx - Vy, set VF = NOT borrow.
                self.v[0x0F] = if self.v[self.op_x()] > self.v[self.op_y()] {
                    1
                } else {
                    0
                };
                self.v[self.op_x()] = self.v[self.op_x()].wrapping_sub(self.v[self.op_y()]);
            },

            0x006 => {
                // 0x8xy6 Set Vx = Vx SHR 1.
                self.v[0x0F] = self.v[self.op_x()] & 1;
                self.v[self.op_x()] >>= 1;
            },

            0x007 => {
                // 0x8xy7 Set Vx = Vy - Vx, set VF = NOT borrow.
                self.v[0x0F] = if self.v[self.op_y()] > self.v[self.op_x()] {
                    1
                } else {
                    0
                };
                self.v[self.op_x()] = self.v[self.op_y()].wrapping_sub(self.v[self.op_x()]);
            },

            0x00E => {
                // 0x8xyE Set Vx = Vx SHL 1.
                self.v[0x0F] = self.v[self.op_x()] >> 7;
                self.v[self.op_x()] <<= 1;
            }

            _ => {
                self.not_implemented();
            }
        }
        self.pc += 2;
    }

    // Skip next instruction if Vx != Vy.
    fn op_9xxx(&mut self) {
        self.pc += if self.v[self.op_x()] != self.v[self.op_y()] {
            4
        } else {
            2
        };
    }

    //Set I = nnn.
    fn op_axxx(&mut self) {
        self.i = self.op_nnn() as usize;
        self.pc += 2;
    }

    // Jump to location nnn + V0.
    fn op_bxxx(&mut self) {
        self.pc = (self.op_nnn() + (self.v[0] as u16)) as usize;
    }

    // Set Vx = random byte AND NN.
    fn op_cxxx(&mut self) {
        let mut rng = rand::thread_rng();
        self.v[self.op_x()] = self.op_nn() & rng.gen::<u8>();
        self.pc += 2;
    }

    // Display n-byte sprite starting at memory location I at (Vx, Vy),
    // set VF = collision.
    fn op_dxxx(&mut self) {
        let start = self.i;
        let end = start + (self.op_n() as usize);
        let x = self.v[self.op_x()] as usize;
        let y = self.v[self.op_y()] as usize;
        // set collision and draw
        self.v[0x0F] = self.screen.draw(x, y, &self.v_ram[start..end]);
        self.pc += 2;
    }

    // Ex9E : Skip next instruction if key with the value of Vx is pressed.
    // ExA1 : Skip next instruction if key with the value of Vx is not pressed.
    fn op_exxx(&mut self) {
        let vxkey = self.v[self.op_x()] as usize;
        self.pc += match self.opcode & 0x00FF {
            0x9E => if self.key_pad.keys[vxkey] { 4 } else { 2 },
            0xA1 => if !self.key_pad.keys[vxkey] { 4 } else { 2 },
            _ => 2,
        }
    }

    fn wait_keypress(&mut self) {
        for i in 0..self.key_pad.keys.len() {
            if self.key_pad.keys[i] {
                self.v[self.op_x()] = i as u8;
                self.pc += 2;
                break;
            }
        }
        //self.pc -= 2;
    }

    fn op_fxxx(&mut self) {
        match self.opcode & 0x00FF {
            0x07 => {
                // Fx07 Set Vx = delay timer value.
                self.v[self.op_x()] = self.delay_timer;
            },
            0x0A => {
                // Fx0A Wait for a key press, store the value of the key in Vx.
                self.wait_keypress();
            },
            0x15 => {
                // Fx15 Set delay timer = Vx.
                self.delay_timer = self.v[self.op_x()];
            },
            0x18 => {
                // Fx18 Set sound timer = Vx.
                self.sound_timer = self.v[self.op_x()];
            }
            0x1E => {
                // Fx1E Set I = I + Vx.
                self.i += self.v[self.op_x()] as usize;
            }
            0x29 => {
                // Fx29 Set I = location of sprite for digit Vx.
                self.i = (self.v[self.op_x()] as usize) * 5;
            }
            0x33 => {
                // Fx33 Store BCD representation of Vx in memory locations I, I+1, and I+2.
                self.v_ram[self.i] = self.v[self.op_x()] / 100;
                self.v_ram[self.i + 1] = (self.v[self.op_x()] / 10) % 10;
                self.v_ram[self.i + 2] = (self.v[self.op_x()] / 100) % 10;
            }
            0x55 => {
                // Fx55 Store registers V0 through Vx in memory starting at location I.
                for j in 0..self.op_x()+1 {
                    self.v_ram[self.i + j] = self.v[j];
                }
                self.i = self.op_x() + 1;
            }
            0x65 => {
                // Fx65 Read registers V0 through Vx from memory starting at location I.
                for j in 0..self.op_x()+1 {
                    self.v[j] = self.v_ram[self.i + j];
                }
                self.i += self.op_x() + 1;
            }
            _ => self.not_implemented(),
        }
        self.pc += 2;
    }

    pub fn load(&mut self, data: &[u8]) {
        for (i, &byte) in data.iter().enumerate() {
            let addr = 0x200 + i;
            if addr < 4096 {
                self.v_ram[addr] = byte;
            } else {
                break;
            }
        }
    }

    pub fn tick(&mut self) -> bool {
        while self.key_pad.poll().is_ok() {
            self.fetch_opcode();
            self.exec_opcode();
            self.screen.draw_screen();
            if self.delay_timer > 0 {
                self.delay_timer -= 1;
            }
            // Around 500Hz clock speed.
            thread::sleep(Duration::from_millis(EXECUTION_DELAY));
        }
        return false;
    }
}