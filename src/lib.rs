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
    pub framebuffer: [bool; Self::WIDTH * Self::HEIGHT],
    pub keys: [bool; 16],
    pc: usize,
    memory: [u8; Self::MEMORY_SIZE],
    index_reg: u16,
    stack: Vec<u16>,
    delay_timer: u8,
    sound_timer: u8,
    ds_timer: Timer, // delay-sound timer
    program_timer: Timer,
    variable_reg: [u8; 16],
    config: Chip8Config,
}

pub struct Chip8Config {
    instructions_per_second: usize,
    program_start: usize,
    default_font: [u8; Self::FONT_CHAR_SIZE * 16],
    font_start: usize,

    // Backwards-compat flags
    copy_vy_while_shifting: bool,
    increment_index_during_save_load: bool,
    index_overflow_flag: bool,
}
impl Default for Chip8Config {
    fn default() -> Self {
        Self {
            instructions_per_second: Self::INSTRUCTIONS_PER_SECOND,
            program_start: Self::PROGRAM_START,
            default_font: Self::DEFAULT_FONT,
            font_start: Self::FONT_START,
            copy_vy_while_shifting: false,
            increment_index_during_save_load: false,
            index_overflow_flag: false,
        }
    }
}

impl Chip8Config {
    pub const INSTRUCTIONS_PER_SECOND: usize = 700;
    pub const PROGRAM_START: usize = 0x200;
    /// In bytes
    pub const FONT_CHAR_SIZE: usize = 5;
    pub const DEFAULT_FONT: [u8; Self::FONT_CHAR_SIZE * 16] = [
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
    pub const FONT_START: usize = 0x050;
}

impl Chip8 {
    pub const WIDTH: usize = 64;
    pub const HEIGHT: usize = 32;
    /// In bytes
    pub const MEMORY_SIZE: usize = 4096;

    pub fn new(config: Chip8Config) -> Self {
        let mut memory = [0; Self::MEMORY_SIZE];
        memory[config.font_start..config.font_start + config.default_font.len()]
            .copy_from_slice(&config.default_font);
        assert!(memory[0x050] == 0xF0);
        Self {
            framebuffer: [false; Self::WIDTH * Self::HEIGHT],
            keys: [false; 16],
            pc: config.program_start,
            memory,
            index_reg: 0,
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
            variable_reg: [0; 16],
            ds_timer: Timer::new(1.0),
            program_timer: Timer::new(1.0 / config.instructions_per_second as f32),
            config,
        }
    }

    pub fn set_program(&mut self, program: &[u8]) {
        self.memory[self.config.program_start..self.config.program_start + program.len()]
            .copy_from_slice(program);
    }
    pub fn should_play_sound(&self) -> bool {
        self.sound_timer > 0
    }

    /// `delta` is in seconds
    pub fn frame(&mut self, delta: f32, keypress: Option<u8>, random_source: impl FnOnce() -> u8) {
        if let Some(keypress) = keypress {
            self.keys[keypress as usize] = true;
        }
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
                    if nibble_1 == 0x0 {
                        if nibble_2 == 0xE {
                            if nibble_3 == 0xE {
                                // INST 00EE
                                self.pc =
                                    self.stack.pop().expect("Tried to return with empty stack.")
                                        as usize;
                            } else if nibble_3 == 0x0 {
                                // INST 00E0 : clear
                                self.framebuffer.fill(false);
                            } else {
                                eprintln!(
                                    "[ERROR] Unknown instruction: {:0>2X}{:0>2X}{:0>2X}{:0>2X}",
                                    nibble_0, nibble_1, nibble_2, nibble_3
                                );
                                return;
                            }
                        }
                    } else {
                        eprintln!(
                            "[ERROR] Unknown instruction: {:0>2X}{:0>2X}{:0>2X}{:0>2X}",
                            nibble_0, nibble_1, nibble_2, nibble_3
                        );
                        return;
                    }
                }
                0x1 => {
                    // INST 1NNN : jump NNN
                    let nnn =
                        ((nibble_1 as u16) << 8) | ((nibble_2 as u16) << 4) | (nibble_3 as u16);
                    self.pc = nnn as usize;
                }
                0x2 => {
                    // INST 2NNN
                    let nnn =
                        ((nibble_1 as u16) << 8) | ((nibble_2 as u16) << 4) | (nibble_3 as u16);
                    self.stack.push(self.pc as u16);
                    self.pc = nnn as usize;
                }
                0x3 => {
                    // INST 3XNN : if vx != NN then
                    let vx = self.variable_reg[nibble_1 as usize];
                    let nn = (nibble_2 << 4) | (nibble_3);
                    if vx == nn {
                        self.pc += 2;
                    }
                }
                0x4 => {
                    // INST 4XNN : if vx == NN then
                    let vx = self.variable_reg[nibble_1 as usize];
                    let nn = (nibble_2 << 4) | (nibble_3);
                    if vx != nn {
                        self.pc += 2;
                    }
                }
                0x5 => {
                    // INST 5XY0 : if vx != vy then
                    let vx = self.variable_reg[nibble_1 as usize];
                    let vy = self.variable_reg[nibble_2 as usize];
                    if vx == vy {
                        self.pc += 2;
                    }
                }
                0x6 => {
                    // INST 6XNN : vx := NN
                    let vx = &mut self.variable_reg[nibble_1 as usize];
                    let nn = (nibble_2 << 4) | (nibble_3);
                    *vx = nn;
                }
                0x7 => {
                    // INST 7XNN : vx += NN
                    let vx = &mut self.variable_reg[nibble_1 as usize];
                    let nn = (nibble_2 << 4) | (nibble_3);
                    *vx += nn;
                }
                0x8 => {
                    let vy = self.variable_reg[nibble_2 as usize];
                    let vx = &mut self.variable_reg[nibble_1 as usize];
                    match nibble_3 {
                        0x0 => {
                            // INST 8XY0 : vx := vy
                            *vx = vy;
                        }
                        0x1 => {
                            // INST 8XY1 : vx |= vy
                            *vx |= vy;
                        }
                        0x2 => {
                            // INST 8XY2 : vx &= vy
                            *vx &= vy;
                        }
                        0x3 => {
                            // INST 8XY3 : vx ^= vy
                            *vx ^= vy;
                        }
                        0x4 => {
                            // INST 8XY4 : vx += vy
                            *vx += vy;
                        }
                        0x5 => {
                            // INST 8XY5 : vx -= vy
                            *vx -= vy;
                        }
                        0x6 => {
                            // INST 8XY6 : vx >>= vy
                            self.variable_reg[0xF] = *vx & 0b1;
                            if self.config.copy_vy_while_shifting {
                                // LEGACY : Old interpreters would copy vy into vx before shifting
                                let vy = self.variable_reg[nibble_2 as usize];
                                let vx = &mut self.variable_reg[nibble_1 as usize];
                                *vx = vy;
                                *vx >>= 1;
                            } else {
                                let vx = &mut self.variable_reg[nibble_1 as usize];
                                *vx >>= 1;
                            }
                        }
                        0x7 => {
                            // INST 8XY7 : vx = vy - vx
                            *vx = vy - *vx;
                        }
                        0xE => {
                            // INST 8XYE : vx <<= vy
                            self.variable_reg[0xF] = *vx & 0b10000000;
                            if self.config.copy_vy_while_shifting {
                                // LEGACY : Old interpreters would copy vy into vx before shifting
                                let vy = self.variable_reg[nibble_2 as usize];
                                let vx = &mut self.variable_reg[nibble_1 as usize];
                                *vx = vy;
                                *vx <<= 1;
                            } else {
                                let vx = &mut self.variable_reg[nibble_1 as usize];
                                *vx <<= 1;
                            }
                        }
                        _ => {
                            eprintln!(
                                "[ERROR] Unknown instruction: {:0>2X}{:0>2X}{:0>2X}{:0>2X}",
                                nibble_0, nibble_1, nibble_2, nibble_3
                            );
                            return;
                        }
                    }
                }
                0x9 => {
                    // INST 9XY0 : if vx == vy then
                    let vx = self.variable_reg[nibble_1 as usize];
                    let vy = self.variable_reg[nibble_2 as usize];
                    if vx != vy {
                        self.pc += 2;
                    }
                }
                0xA => {
                    // INST ANNN : index_reg := NNN
                    let nnn =
                        ((nibble_1 as u16) << 8) | ((nibble_2 as u16) << 4) | (nibble_3 as u16);
                    self.index_reg = nnn;
                }
                0xB => {
                    // INST BNNN : jump NNN + v0
                    let nnn =
                        ((nibble_1 as u16) << 8) | ((nibble_2 as u16) << 4) | (nibble_3 as u16);
                    let v0 = self.variable_reg[0x0] as u16;
                    self.pc = (nnn + v0) as usize;
                }
                0xC => {
                    // INST CXNN
                    let vx = &mut self.variable_reg[nibble_1 as usize];
                    let nn = (nibble_2 << 4) | (nibble_3);
                    *vx = random_source() & nn;
                }
                0xD => {
                    // INST DXYN : sprite vx vy N
                    let vx = self.variable_reg[nibble_1 as usize];
                    let vy = self.variable_reg[nibble_2 as usize];
                    let n = nibble_3 as u16;

                    let x = vx as usize % Self::WIDTH;
                    let mut y = vy as usize % Self::HEIGHT;
                    self.variable_reg[0xF] = 0;
                    for i in 0..n {
                        let data = self.memory[(self.index_reg + i) as usize];
                        let mut x = x;
                        for j in 0..8 {
                            let data_bit = ((0b10000000 >> j) & data) > 0;
                            let current_pixel = &mut self.framebuffer[y * Self::WIDTH + x];
                            if data_bit {
                                if *current_pixel {
                                    *current_pixel = false;
                                    self.variable_reg[0xF] = 1;
                                } else {
                                    *current_pixel = true;
                                }
                            }
                            x += 1;
                            if x >= Self::WIDTH {
                                break;
                            }
                        }
                        y += 1;
                        if y >= Self::HEIGHT {
                            break;
                        }
                    }
                }
                0xE => {
                    let vx = self.variable_reg[nibble_1 as usize];
                    if vx < self.keys.len() as u8 {
                        if nibble_2 == 0x9 && nibble_3 == 0xE {
                            // INST EX9E : if key = vx not pressed then
                            if self.keys[vx as usize] {
                                self.pc += 2;
                            }
                        } else if nibble_2 == 0xA && nibble_3 == 0x1 {
                            // INST EXA1 : if key = vx is pressed then
                            if !self.keys[vx as usize] {
                                self.pc += 2;
                            }
                        } else {
                            eprintln!(
                                "[ERROR] Unknown instruction: {:0>2X}{:0>2X}{:0>2X}{:0>2X}",
                                nibble_0, nibble_1, nibble_2, nibble_3
                            );
                            return;
                        }
                    } else {
                        eprintln!("[WARN] Tried to check whether key {vx} is pressed but max value of key is {}.", self.keys.len())
                    }
                }
                0xF => {
                    let vx = &mut self.variable_reg[nibble_1 as usize];
                    if nibble_2 == 0x0 {
                        if nibble_3 == 0x7 {
                            // INST FX07 : vx := delay
                            *vx = self.delay_timer;
                        } else if nibble_3 == 0xA {
                            // INST FX0A : vx := key // Wait for a keypress
                            if let Some(keypress) = keypress {
                                *vx = keypress;
                            } else {
                                self.pc -= 2; // wait
                            }
                        } else {
                            eprintln!(
                                "[ERROR] Unknown instruction: {:0>2X}{:0>2X}{:0>2X}{:0>2X}",
                                nibble_0, nibble_1, nibble_2, nibble_3
                            );
                        }
                    } else if nibble_2 == 0x1 {
                        if nibble_3 == 0x5 {
                            // INST FX15 : delay := vx
                            self.delay_timer = *vx;
                        } else if nibble_3 == 0x8 {
                            // INST FX18 : sound := vx
                            self.sound_timer = *vx;
                        } else if nibble_3 == 0xE {
                            // INST FX1E : index_reg += vx
                            self.index_reg += *vx as u16;
                            if self.config.index_overflow_flag {
                                // LEGACY : The interpreter for Amiga would treat index_reg going above 0x0FFF as a special overflow and would set vf := 1 in that case
                                // The game called "Spacefight 2091!" relies on this.
                                if self.index_reg > 0x0FFF {
                                    self.variable_reg[0xF] = 1;
                                }
                            }
                        } else {
                            eprintln!(
                                "[ERROR] Unknown instruction: {:0>2X}{:0>2X}{:0>2X}{:0>2X}",
                                nibble_0, nibble_1, nibble_2, nibble_3
                            );
                        }
                    } else if nibble_2 == 0x2 && nibble_3 == 0x9 {
                        // INST FX29 : index_reg := hex vx
                        let vx = self.variable_reg[nibble_1 as usize];
                        let ch = (vx & 0b00001111) as u16;
                        self.index_reg =
                            self.config.font_start as u16 + ch * Chip8Config::FONT_CHAR_SIZE as u16;
                    } else if nibble_2 == 0x3 && nibble_3 == 0x3 {
                        // INST FX33 : bcd vx // Decode vx into binary-coded decimal
                        let vx = self.variable_reg[nibble_1 as usize];
                        self.memory[self.index_reg as usize] = vx / 100;
                        self.memory[self.index_reg as usize + 1] = (vx / 10) % 10;
                        self.memory[self.index_reg as usize + 2] = (vx % 100) % 10;
                    } else if nibble_2 == 0x5 && nibble_3 == 0x5 {
                        // INST FX55 : save vx // Save v0-vx to index_reg through (index_reg+x)
                        for x in 0..=nibble_1 as usize {
                            if self.config.increment_index_during_save_load {
                                // LEGACY : Old interpreters used to increment the index register along the way.
                                self.memory[self.index_reg as usize] = self.variable_reg[x];
                                self.index_reg += 1;
                            } else {
                                self.memory[self.index_reg as usize + x] = self.variable_reg[x];
                            }
                        }
                    } else if nibble_2 == 0x6 && nibble_3 == 0x5 {
                        // INST FX65 : load vx // Load v0-vx from index_reg through (index_reg+x)
                        for x in 0..=nibble_1 as usize {
                            if self.config.increment_index_during_save_load {
                                // LEGACY : Old interpreters used to increment the index register along the way.
                                self.variable_reg[x] = self.memory[self.index_reg as usize];
                                self.index_reg += 1;
                            } else {
                                self.variable_reg[x] = self.memory[self.index_reg as usize + x];
                            }
                        }
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
