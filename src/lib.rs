use rand::prelude::*;

const SCREEN_WIDTH: u8 = 64;
const SCREEN_HEIGHT: u8 = 64;
const SPRITE_WIDTH: u8 = 8;

#[derive(Debug)]
pub struct CPU {
    program_counter: usize,
    stack_pointer: usize,
    I: usize,
    pub memory: [u8; 4096],
    stack: [u16; 16],
    pub registers: [u8; 16],
    pub display: [u8; 4096]
}

// #[derive(Debug, Clone, Copy)]
// struct Pixel {
//   x: u8,
//   y: u8
// }

impl CPU {
    const FONTSET: [u8; 80] = [
        0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
        0x20, 0x60, 0x20, 0x20, 0x70, // 1
        0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
        0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
        0x90, 0x90, 0xF0, 0x10, 0x10, // 4
        0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
        0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
        0xF0, 0x10, 0x20, 0x40, 0x40, // 7
        0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
        0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
        0xF0, 0x90, 0xF0, 0x90, 0x90, // A
        0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
        0xF0, 0x80, 0x80, 0x80, 0xF0, // C
        0xE0, 0x90, 0x90, 0x90, 0xE0, // D
        0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
        0xF0, 0x80, 0xF0, 0x80, 0x80, // F
    ];

    pub fn new() -> Self {
        CPU {
            memory: [0; 4096],
            program_counter: 512,
            I: 0,
            stack: [0; 16],
            stack_pointer: 0,
            registers: [0; 16],
            display: [0; 4096]
        }
    }

    pub fn load_font_into_memory(&mut self) {
        for (i, f) in Self::FONTSET.iter().enumerate() {
            self.memory[i] = *f;
        }
    }

    pub fn draw(&mut self, opcode: u16, x: u8, y: u8) {
        let height = opcode & 0x000F;
        let x_coordinate = self.registers[x as usize];
        let y_coordinate = self.registers[y as usize];

        // println!("height: {} --- x_co: {} --- y_co: {}", height, x_coordinate, y_coordinate);

        // let ands: [u8; 8] = [128, 64, 32, 16, 8, 4, 2, 1];

        self.registers[0xF] = 0;

        for i in 0..height {
          let pixel = self.memory[self.I + i as usize];

          // println!("pixel: {}", pixel);
          for j in 0..SPRITE_WIDTH {
            // if x_coordinate + j as i8 == SCREEN_WIDTH as i8 {
            //   x_coordinate = -(j as i8);
            // }
            // if y_coordinate + i as i8 == SCREEN_HEIGHT as i8 {
            //   y_coordinate = -(i as i8);
            // }
            // println!("j: {} --- 0x80_j: {} --- &: {}", j, 0x80 >> j, pixel & (0x80 >> j));
            // println!("lol: {}", self.display[((x_coordinate + j) as usize + ((y_coordinate + i as u8) as usize) * 64) as usize]);
            if pixel & (0x80 >> j) != 0 {
              self.display[((x_coordinate + j) as usize + ((y_coordinate + i as u8) as usize) * 64) as usize] ^= 1;
              print!("*");
              // if self.display[((x_coordinate + j) as usize + ((y_coordinate + i as u8) as usize) * 64) as usize] == 1 {
              //   self.registers[0xF] = 1;
              //   self.display[((x_coordinate + j) as usize + ((y_coordinate + i as u8) as usize) * 64) as usize] ^= 1;
              // }
            } else {
                print!(" ");
            }
          }
          print!("\n");            
        }
    }

    pub fn run(&mut self) {
        loop {
            let op_byte1 = self.memory[self.program_counter] as u16;
            let op_byte2 = self.memory[self.program_counter + 1] as u16;

            let opcode = op_byte1 << 8 | op_byte2;

            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let kk = (opcode & 0x00FF) as u8;
            let op_minor = (opcode & 0x000F) as u8;
            let nnn = opcode & 0x0FFF;

            self.program_counter += 2;

            match opcode {
                0x0000 => {
                    return;
                }
                0x00E0 => self.cls(),
                0x00EE => self.ret(),
                0x1000..=0x1FFF => self.jp(nnn),
                0x2000..=0x2FFF => self.call(nnn),
                0x3000..=0x3FFF => self.se(x, kk),
                0x4000..=0x4FFF => self.sne(x, kk),
                0x5000..=0x5FFF => self.se_xy(x, y),
                0x6000..=0x6FFF => self.ld(x, kk),
                0x7000..=0x7FFF => self.add(x, kk),
                0x8000..=0x8FFF => match op_minor {
                    0 => self.ld_xy(x, y),
                    1 => self.or_xy(x, y),
                    2 => self.and_xy(x, y),
                    3 => self.xor_xy(x, y),
                    4 => self.add_xy(x, y),
                    5 => self.sub_xy(x, y),
                    _ => {
                        todo!("opcode: {:04x}", opcode);
                    }
                },
                0x9000..=0x9FFF => self.sne_xy(x, y),
                0xA000..=0xAFFF => self.ld_i(nnn),
                0xB000..=0xBFFF => self.jp_0(nnn),
                0xC000..=0xCFFF => self.rnd(x, kk),
                0xD000..=0xDFFF => self.draw(opcode, x, y),
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }

    fn cls(&self) {}

    fn jp(&mut self, nnn: u16) {
        self.program_counter = nnn as usize;
    }

    fn call(&mut self, nnn: u16) {
        if self.stack_pointer >= self.stack.len() {
            panic!("Stack Overflow!");
        }

        self.stack[self.stack_pointer] = self.program_counter as u16;
        self.stack_pointer += 1;
        self.program_counter = nnn as usize;
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack Underflow!");
        }

        self.stack_pointer -= 1;
        self.program_counter = self.stack[self.stack_pointer] as usize;
    }

    fn se(&mut self, x: u8, kk: u8) {
        if x == kk {
            self.program_counter += 2;
        }
    }

    fn sne(&mut self, x: u8, kk: u8) {
        if x != kk {
            self.program_counter += 2;
        }
    }

    fn se_xy(&mut self, x: u8, y: u8) {
        if x == y {
            self.program_counter += 2;
        }
    }

    fn sne_xy(&mut self, x: u8, y: u8) {
        if x != y {
            self.program_counter += 2;
        }
    }

    fn ld(&mut self, x: u8, kk: u8) {
        self.registers[x as usize] = kk;
    }

    fn add(&mut self, x: u8, kk: u8) {
        self.registers[x as usize] = self.registers[x as usize] + kk;
    }

    fn ld_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] = self.registers[y as usize];
    }

    fn and_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] = self.registers[x as usize] & self.registers[y as usize];
    }

    fn or_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] = self.registers[x as usize] | self.registers[y as usize];
    }

    fn xor_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] = self.registers[x as usize] ^ self.registers[y as usize];
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let _x = self.registers[x as usize];
        let _y = self.registers[y as usize];

        let (value, overflow) = _x.overflowing_add(_y);

        if overflow {
            self.registers[0xF] = 1;
        }

        self.registers[x as usize] = value;
    }

    fn sub_xy(&mut self, x: u8, y: u8) {
        let _x = self.registers[x as usize];
        let _y = self.registers[y as usize];

        if _x > _y {
            self.registers[0xF] = 1;
            self.registers[x as usize] = _x - _y;
        } else {
            self.registers[0xF] = 0;
            self.registers[x as usize] = _y - _x;
        }
    }

    // fn shr(&mut self, x: u8, y: u8) {

    // }

    fn ld_i(&mut self, nnn: u16) {
        self.I = nnn as usize;
    }

    fn jp_0(&mut self, nnn: u16) {
        self.program_counter = nnn as usize + self.registers[0 as usize] as usize;
    }

    fn rnd(&mut self, x: u8, kk: u8) {
        let rand_num: u8 = random();
        self.registers[x as usize] = rand_num & kk;
    }
}
