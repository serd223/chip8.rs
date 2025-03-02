pub struct Timer {
    raw: f32,
    length: f32,
}

impl Timer {
    /// Length is in seconds
    pub fn new(length: f32) -> Self {
        Self { raw: 0., length }
    }
    pub fn check(&mut self, delta: f32) -> bool {
        self.raw += delta;

        if self.raw >= self.length {
            self.raw = 0.0;
            true
        } else {
            false
        }
    }
}

pub struct Chip8 {
    pub framebuffer: [u8; Self::WIDTH * Self::HEIGHT],
    pub keys: [bool; 16],
    pc: usize,
    memory: [u8; Self::MEMORY_SIZE],
    index_reg: u16,
    stack: Vec<u16>,
    delay_timer: u16,
    sound_timer: u16,
    ds_timer: Timer, // delay-sound timer
    program_timer: Timer,
    variable_reg: [u8; 16],
}

impl Chip8 {
    pub const WIDTH: usize = 64;
    pub const HEIGHT: usize = 32;
    /// In bytes
    pub const MEMORY_SIZE: usize = 4096;
    pub const PROGRAM_START: usize = 0x200;
    pub const DEFAULT_FONT: [u8; 80] = [
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
    pub const INSTRUCTIONS_PER_SECOND: usize = 700;

    pub fn new() -> Self {
        let mut memory = [0; Self::MEMORY_SIZE];
        memory[0x050..0x09F].copy_from_slice(&Self::DEFAULT_FONT);
        assert!(memory[0x050] == 0xF0);
        Self {
            framebuffer: [0; Self::WIDTH * Self::HEIGHT],
            keys: [false; 16],
            pc: Self::PROGRAM_START,
            memory,
            index_reg: 0,
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
            variable_reg: [0; 16],
            ds_timer: Timer::new(1.0),
            program_timer: Timer::new(1.0 / Self::INSTRUCTIONS_PER_SECOND as f32),
        }
    }

    /// delta is in seconds
    pub fn frame(&mut self, delta: f32) {
        if self.ds_timer.check(delta) {
            if self.delay_timer > 0 {
                self.delay_timer -= 1;
            }
            if self.sound_timer > 0 {
                self.sound_timer -= 1;
            }
        }

        if self.program_timer.check(delta) {
            let nibble_0 = self.memory[self.pc] >> 4;
            let nibble_1 = self.memory[self.pc] & 0b00001111;
            let nibble_2 = self.memory[self.pc + 1] >> 4;
            let nibble_3 = self.memory[self.pc + 1] & 0b00001111;
            self.pc += 2;
            match nibble_0 {
                0x0 => {
                    if nibble_2 == 0xE {
                        if nibble_3 == 0xE {
                            todo!("return")
                        } else if nibble_3 == 0x0 {
                            todo!("clear")
                        } else {
                            eprintln!(
                                "[ERROR] Unknown instruction: {:0>2X}{:0>2X}{:0>2X}{:0>2X}",
                                nibble_0, nibble_1, nibble_2, nibble_3
                            );
                        }
                    }
                }
                0x1 => {
                    let nnn =
                        ((nibble_1 as u16) << 16) | ((nibble_2 as u16) << 8) | (nibble_3 as u16);
                    todo!("jump NNN")
                }
                0x2 => {
                    let nnn =
                        ((nibble_1 as u16) << 16) | ((nibble_2 as u16) << 8) | (nibble_3 as u16);
                    todo!("call subroutine NNN")
                }
                0x3 => {
                    let vx = self.variable_reg[nibble_1 as usize];
                    let nn = ((nibble_2 << 4) | (nibble_3)) as u16;
                    todo!("if vx != NN then")
                }
                0x4 => {
                    let vx = self.variable_reg[nibble_1 as usize];
                    let nn = ((nibble_2 << 4) | (nibble_3)) as u16;
                    todo!("if vx == NN then")
                }
                0x5 => {
                    let vx = self.variable_reg[nibble_1 as usize];
                    let vy = self.variable_reg[nibble_2 as usize];
                    todo!("if vx != vy then")
                }
                0x6 => {
                    let vx = &mut self.variable_reg[nibble_1 as usize];
                    let nn = ((nibble_2 << 4) | (nibble_3)) as u16;
                    todo!("vx := NN")
                }
                0x7 => {
                    let vx = &mut self.variable_reg[nibble_1 as usize];
                    let nn = ((nibble_2 << 4) | (nibble_3)) as u16;
                    todo!("vx += NN")
                }
                0x8 => {
                    let vx = &mut self.variable_reg[nibble_1 as usize];
                    let vy = self.variable_reg[nibble_2 as usize];
                    match nibble_3 {
                        0x0 => {
                            todo!("vx := vy")
                        }
                        0x1 => {
                            todo!("vx |= vy")
                        }
                        0x2 => {
                            todo!("vx &= vy")
                        }
                        0x3 => {
                            todo!("vx ^= vy")
                        }
                        0x4 => {
                            todo!("vx += vy")
                        }
                        0x5 => {
                            todo!("vx -= vy")
                        }
                        0x6 => {
                            todo!("vx >>= vy")
                        }
                        0x7 => {
                            todo!("vx =- vy")
                        }
                        0xE => {
                            todo!("vx <<= vy")
                        }
                        _ => {
                            eprintln!(
                                "[ERROR] Unknown instruction: {:0>2X}{:0>2X}{:0>2X}{:0>2X}",
                                nibble_0, nibble_1, nibble_2, nibble_3
                            );
                        }
                    }
                }
                0x9 => {
                    let vx = self.variable_reg[nibble_1 as usize];
                    let vy = self.variable_reg[nibble_2 as usize];
                    todo!("if vx == vy then")
                }
                0xA => {
                    let nnn =
                        ((nibble_1 as u16) << 16) | ((nibble_2 as u16) << 8) | (nibble_3 as u16);
                    todo!("index_reg := NNN")
                }
                0xB => {
                    let nnn =
                        ((nibble_1 as u16) << 16) | ((nibble_2 as u16) << 8) | (nibble_3 as u16);
                    todo!("jump NNN + v0")
                }
                0xC => {
                    let vx = &mut self.variable_reg[nibble_1 as usize];
                    let nn = ((nibble_2 << 4) | (nibble_3)) as u16;
                    todo!("vx := random & NN")
                }
                0xD => {
                    let vx = self.variable_reg[nibble_1 as usize];
                    let vy = self.variable_reg[nibble_2 as usize];
                    let n = nibble_3 as u16;
                    todo!("sprite vx vy N")
                }
                0xE => {
                    let vx = self.variable_reg[nibble_1 as usize];
                    if nibble_2 == 0x9 && nibble_3 == 0xE {
                        todo!("if key = vx not pressed then")
                    } else if nibble_2 == 0xA && nibble_3 == 0x1 {
                        todo!("if key = vx is pressed then")
                    } else {
                        eprintln!(
                            "[ERROR] Unknown instruction: {:0>2X}{:0>2X}{:0>2X}{:0>2X}",
                            nibble_0, nibble_1, nibble_2, nibble_3
                        );
                    }
                }
                0xF => {
                    let vx = &mut self.variable_reg[nibble_1 as usize];
                    if nibble_2 == 0x0 {
                        if nibble_3 == 0x7 {
                            todo!("vx := delay")
                        } else if nibble_3 == 0xA {
                            todo!("vx := key")
                        } else {
                            eprintln!(
                                "[ERROR] Unknown instruction: {:0>2X}{:0>2X}{:0>2X}{:0>2X}",
                                nibble_0, nibble_1, nibble_2, nibble_3
                            );
                        }
                    } else if nibble_2 == 0x1 {
                        if nibble_3 == 0x5 {
                            todo!("delay := vx")
                        } else if nibble_3 == 0x8 {
                            todo!("sound := vx")
                        } else if nibble_3 == 0xE {
                            todo!("index_reg += vx")
                        } else {
                            eprintln!(
                                "[ERROR] Unknown instruction: {:0>2X}{:0>2X}{:0>2X}{:0>2X}",
                                nibble_0, nibble_1, nibble_2, nibble_3
                            );
                        }
                    } else if nibble_2 == 0x2 && nibble_3 == 0x9 {
                        let vx = self.variable_reg[nibble_1 as usize];
                        todo!("index_reg := hex vx")
                    } else if nibble_2 == 0x3 && nibble_3 == 0x3 {
                        let vx = self.variable_reg[nibble_1 as usize];
                        todo!("bcd vx // Decode vx into binary-coded decimal")
                    } else if nibble_2 == 0x5 && nibble_3 == 0x5 {
                        let vx = self.variable_reg[nibble_1 as usize];
                        todo!("save vx // Save v0-vx to i through (i+x)")
                    } else if nibble_2 == 0x6 && nibble_3 == 0x5 {
                        let vx = &mut self.variable_reg[nibble_1 as usize];
                        todo!("load vx // Load v0-vx from i through (i+x)")
                    } else {
                        eprintln!(
                            "[ERROR] Unknown instruction: {:0>2X}{:0>2X}{:0>2X}{:0>2X}",
                            nibble_0, nibble_1, nibble_2, nibble_3
                        );
                    }
                }
                _ => {
                    eprintln!(
                        "[ERROR] Unknown instruction: {:0>2X}{:0>2X}{:0>2X}{:0>2X}",
                        nibble_0, nibble_1, nibble_2, nibble_3
                    );
                }
            }
        }
    }
}
