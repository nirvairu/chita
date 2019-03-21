// impletements the CHIP-8 Instruction set
use rand::Rng;

pub const C8_WIDTH: usize = 64;
pub const C8_HEIGHT: usize = 32;

const PROG_START: usize = 0x200;
const MEMORY_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;

const FONTSET_SIZE: usize = 80;
const FONTSET: [u8; FONTSET_SIZE] = [
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
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];
pub struct Chip8 {
    display_buf: [[bool; C8_WIDTH]; C8_HEIGHT],
    memory: [u8; MEMORY_SIZE],
    stack: [usize; STACK_SIZE],
    stack_pointer: usize,
    addr_register: usize,
    register: [u8; 16], // Registers
    prog_counter: usize,
    delay_timer: u8,
    audio_timer: u8,
    key_state: [bool; 16],
}

impl Chip8 {
    pub fn new() -> Self {
        let mut sys = Self {
            display_buf: [[false; C8_WIDTH]; C8_HEIGHT],
            memory: [0; MEMORY_SIZE],
            stack: [0; 16],
            stack_pointer: 0,
            addr_register: 0,
            register: [0; 16],
            prog_counter: PROG_START,
            delay_timer: 0,
            audio_timer: 0,
            key_state: [false; 16],
        };

        sys.memory[0..FONTSET_SIZE].copy_from_slice(&FONTSET);
        sys
    }
    
    pub fn get_display(&self) -> &[[bool; C8_WIDTH]; C8_HEIGHT] {
        &self.display_buf
    }

    pub fn get_keys(&mut self) -> &mut [bool; 16] {
        &mut self.key_state
    }

    /*
    pub fn print_keys(&self) {
        println!("Array: {:?}", self.key_state);
    }
    */

    pub fn timer_advance(&mut self) -> bool {
        let mut beep = false;
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.audio_timer > 0 {
            if self.audio_timer == 1 {
                beep = true;
            }
            self.audio_timer -= 1;
        }
        beep
    }

    pub fn do_iteration(&mut self) {
        let opcode = self.fetch_opcode();
        self.run_opcode(opcode);
    }

    pub fn load_program(&mut self, data: &[u8]) {
        let start = PROG_START;
        let end = PROG_START + data.len();
        self.memory[start..end].copy_from_slice(data);
    }

    fn fetch_opcode(&mut self) -> u16 {
        let opcode: u16 = (self.memory[self.prog_counter] as u16) << 8 |
        (self.memory[self.prog_counter + 1] as u16);
        // Increment Prog Counter
        self.prog_counter += 2;
        opcode
    }

    fn run_opcode(&mut self, opcode: u16) {
        let op = (
            ((opcode & 0xF000) >> 12) as u8,
            ((opcode & 0x0F00) >> 8) as u8,
            ((opcode & 0x00F0) >> 4) as u8,
            (opcode & 0x000F) as u8,
        );

        let (_, x, y, n) = op;

        let nnn = opcode & 0xFFF;
        let nn = (opcode & 0xFF) as u8;

        match op {
            // No OP
            (0, 0, 0, 0) => return,
            // CLS
            (0, 0, 0xE, 0) => {
                for h in 0..C8_HEIGHT {
                    for  w in 0..C8_WIDTH {
                        self.display_buf[h][w] = false;
                    }
                }
            }
            // Return from subroutine
            (0, 0, 0xE, 0xE) => {
                self.prog_counter= self.pop();
            }
            // Jump to NNN
            (1, _, _, _) => {
                self.prog_counter = nnn as usize;
            }
            // Call subroutine at NNN
            (2, _, _, _) => {
                self.push(self.prog_counter);
                self.prog_counter = nnn as usize;
            }
            // Skip if Vx == NN
            (3, _, _, _) => {
                if self.register[x as usize] == nn {
                    self.prog_counter +=2;
                }
            }
            // Skip if Vx != NN
            (4, _, _, _) => {
                if self.register[x as usize] != nn {
                    self.prog_counter += 2;
                }
            }
            // Skip if Vx == Vy
            (5, _, _, _) => {
                if self.register[x as usize] == self.register[y as usize] {
                    self.prog_counter += 2;
                }
            }
            // Set VX = NN
            (6, _, _, _) => {
                self.register[x as usize] = nn;
            }
            // Vx += NN
            (7, _, _, _) => {
                self.register[x as usize] = self.register[x as usize].wrapping_add(nn);
            }
            // Vx = Vy
            (8, _, _, 0) => {
                self.register[x as usize] = self.register[y as usize];
            }
            // Vx |= Vy
            (8, _, _, 1) => {
                self.register[x as usize] |= self.register[y as usize];
            }
            // Vx &= Vy
            (8, _, _, 2) => {
                self.register[x as usize] &= self.register[y as usize];
            }
            // VX ^= VY
            (8, _, _, 3) => {
                self.register[x as usize] ^= self.register[y as usize];
            },
            // VX += VY
            // Check Correct overflow behvarious
            (8, _, _, 4) => {
                let (vx_new, overflow) = self.register[x as usize]
                    .overflowing_add(self.register[y as usize]);

                let carry = overflow as u8;
                self.register[x as usize] = vx_new;
                self.register[0xF] = carry;
            },
            // VX -= VY
            (8, _, _, 5) => {
                let (vx_new, underflow) = self.register[x as usize]
                    .overflowing_sub(self.register[y as usize]);

                let borrow = (!underflow) as u8;

                self.register[x as usize] = vx_new;
                self.register[0xF] = borrow;
            },
            // VX >>= 1
            (8, _, _, 6) => {
                let least_significant_bit = self.register[x as usize] & 0x1;

                self.register[x as usize] >>= 1;
                self.register[0xF] = least_significant_bit;
            },
            // VX = VY - VX
            (8, _, _, 7) => {

                let (vx_new, underflow) = self.register[y as usize]
                    .overflowing_sub(self.register[x as usize]);
                let borrow = (!underflow) as u8;

                self.register[x as usize] = vx_new;
                self.register[0xF] = borrow;
            },
            // VX <<= 1
            (8, _, _, 0xE) => {
                let most_significant_bit = (self.register[x as usize] >> 7 ) & 0x1;
                self.register[x as usize] <<= 1;
                self.register[0xF] = most_significant_bit;
            }
            // SKIP VX != VY
            (9, _, _, 0) => {
                if self.register[x as usize] != self.register[y as usize] {
                    self.prog_counter += 2;
                }
            }
            // I = NNN
            (0xA, _, _, _) => {
                self.addr_register = nnn as usize;
            },
            // JMP V0 + NNN
            (0xB, _, _, _) => {
                self.prog_counter = ((self.register[0] as u16) + nnn ) as usize;
            }
            // VX = rand() & NN
            (0xC, _, _, _) => {
                let rng: u8 = rand::thread_rng().gen();
                self.register[x as usize] = rng & nn;
            }
            // DRAW X, Y = coords, N = Height
            (0xD, _, _, _) => {

                let x_cord = self.register[x as usize];
                let y_cord = self.register[y as usize];
                let rows = n;

                let mut collision = false;

                for del_y in 0..rows {
                    let line_addr = self.addr_register + (del_y as usize);
                    let pixel_data = self.memory[line_addr];

                    // Note Each line is a Bit of the Byte at the Address
                    // We iterate over each bit in pixel_data
                    for del_x in 0..8 {
                        if (pixel_data & (0b1000_0000 >> del_x)) != 0 {
                            //  sprites should wrap
                            let x = (x_cord + del_x ) as usize % C8_WIDTH;
                            let y = (y_cord + del_y) as usize % C8_HEIGHT;

                            collision |= self.display_buf[y][x];
                            // Xor at the coordinate
                            self.display_buf[y][x] ^= true;
                        }
                    }
                }

                self.register[0xF] = match collision {
                    true => 1,
                    false => 0,
                };
            }
            // SKIP KEY PRESS
            (0xE, _, 9, 0xE) => {
                let vx = self.register[x as usize];
                let key = self.key_state[vx as usize];
                if key {
                    self.prog_counter += 2;
                }
            },
            // SKIP KEY RELEASE
            (0xE, _, 0xA, 1) => {
                let vx = self.register[x as usize];
                let key = self.key_state[vx as usize];
                if !key {
                    self.prog_counter += 2;
                }
            },
            // VX = DT
            (0xF, _, 0, 7) => {
                self.register[x as usize] = self.delay_timer;
            },
            // WAIT KEY, Loop until key is pressed
            (0xF, _, 0, 0xA) => {
                let mut keypress = false;
                for i in 0..self.key_state.len() {
                    if self.key_state[i] {
                        self.register[x as usize] = i as u8;
                        keypress = true;
                        break;
                    }
                }

                if !keypress {
                    // Go Back
                    self.prog_counter -= 2;
                }
            },
            // DT = VX
            (0xF, _, 1, 5) => {
                self.delay_timer = self.register[x as usize];
            },
            // ST = VX
            (0xF, _, 1, 8) => {
                self.audio_timer = self.register[x as usize];
            },
            // I += VX
            (0xF, _, 1, 0xE) => {
                let vx = self.register[x as usize] as usize;
                self.addr_register = self.addr_register.wrapping_add(vx);
            },
            // I = FONT at Vx
            (0xF, _, 2, 9) => {
                let num = self.register[x as usize] as usize;
                self.addr_register = num * 5; // Each sprite is 5 bytes long and starts at 0
            },
            // BCD
            (0xF, _, 3, 3) => {
                let vx = self.register[x as usize] as f32;

                // Fetch the hundreds digit by dividing by 100 and tossing the decimal
                let hundreds = (vx / 100.0).floor() as u8;
                // Fetch the tens digit by dividing by 10, tossing the ones digit and the decimal
                let tens = ((vx / 10.0) % 10.0).floor() as u8;
                // Fetch the ones digit by tossing the hundreds and the tens
                let ones = (vx % 10.0) as u8;

                self.memory[self.addr_register] = hundreds;
                self.memory[self.addr_register + 1] = tens;
                self.memory[self.addr_register + 2] = ones;
            },
            // STORE V0 - VX to mem location starting from I
            (0xF, _, 5, 5) => {
                let i = self.addr_register;
                for del_i in 0..=(x as usize) {
                    self.memory[i + del_i] = self.register[del_i];
                }
            },
            // LOAD V0 - VX - Converse of the above
            (0xF, _, 6, 5) => {
                let i = self.addr_register;
                for del_i in 0..=(x as usize) {
                    self.register[del_i] = self.memory[i + del_i];
                }
            },
            _ => unimplemented!("Unimplemented opcode: {:#04X}", opcode),
            
        }
    }

    fn push(& mut self, val: usize) {
        self.stack[self.stack_pointer as usize] = val;
        self.stack_pointer += 1;
    }

    fn pop(& mut self) -> usize {
        self.stack_pointer -= 1;
        self.stack[self.stack_pointer]
    }

}
