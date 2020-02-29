use log::info;
use opcode::Opcode;
use std::io;

mod interpreter;
mod opcode;

const DISPLAY_SIZE: usize = 64 * 32;
const STACK_SIZE: usize = 16;
const MEMORY_SIZE: usize = 4096;

#[repr(u8)]
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum PixelState {
    Off = 0x0,
    On = 0xFF,
}

impl PixelState {
    pub fn toggle(&mut self) {
        match self {
            PixelState::On => *self = PixelState::Off,
            PixelState::Off => *self = PixelState::On,
        }
    }
}

impl From<u8> for PixelState {
    fn from(byte: u8) -> Self {
        match byte {
            0 => PixelState::On,
            1 => PixelState::Off,
            _ => panic!("Invalid value = {}", byte),
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum KeyState {
    Up = 0,
    Down = 1,
}

#[derive(Debug)]
pub enum CpuState {
    /// Not running
    Halt,
    /// Running
    Running,
    /// Waiting for input
    WaitInput(usize),
}

pub struct Cpu {
    registers: [u8; 16],
    i: u16,
    sp: usize,
    pc: usize,
    dt: usize,
    st: usize,
    stack: [u16; 16],
    memory: [u8; MEMORY_SIZE],
    display: [PixelState; DISPLAY_SIZE],
    keypad: [KeyState; 16],
    keypad_event: [KeyState; 16],
    state: CpuState,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            registers: [0; 16],
            i: 0,
            pc: 0,
            sp: 0,
            dt: 0,
            st: 0,
            stack: [0; STACK_SIZE],
            memory: [0; MEMORY_SIZE],
            display: [PixelState::Off; DISPLAY_SIZE],
            keypad: [KeyState::Up; 16],
            keypad_event: [KeyState::Up; 16],
            state: CpuState::Halt,
        }
    }
}

impl Cpu {
    /// Creates a CHIP8 cpu with the memory set to zero.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn halt(&mut self) {
        self.state = CpuState::Halt
    }

    pub fn state(&self) -> &CpuState {
        &self.state
    }

    pub fn registers(&self) -> &[u8; 16] {
        &self.registers
    }

    pub fn program_counter(&self) -> usize {
        self.pc
    }

    pub fn stack_pointer(&self) -> usize {
        self.sp
    }

    pub fn delay_timer(&self) -> usize {
        self.dt
    }

    pub fn sound_timer(&self) -> usize {
        self.st
    }

    pub fn stack(&self) -> &[u16; STACK_SIZE] {
        &self.stack
    }

    pub fn i(&self) -> u16 {
        self.i
    }

    /// Initialize the cpu
    pub fn load<R: AsRef<[u8]>>(&mut self, rom: R) {
        self.reset();
        self.load_interpreter();
        dump(rom.as_ref(), &mut self.memory[0x200..]);
        self.pc = 0x200;
        self.state = CpuState::Running;
    }

    fn load_interpreter(&mut self) {
        dump(interpreter::FONT, &mut self.memory[..]);
    }

    /// Resets the cpu.
    pub fn reset(&mut self) {
        std::mem::replace(self, Default::default());
    }

    /// Return memory
    #[allow(dead_code)]
    pub fn memory(&self) -> &[u8; 4096] {
        &self.memory
    }

    pub fn display(&self) -> &[PixelState; DISPLAY_SIZE] {
        &self.display
    }

    /// Set key state.
    pub fn set_key(&mut self, key: usize, state: KeyState) {
        self.keypad[key] = state;
        if state == KeyState::Down {
            self.keypad_event[key] = KeyState::Down;
        }
    }

    /// Step the simulation.
    pub fn step(&mut self) {
        let key_down = self.any_key_down();
        match (&self.state, key_down) {
            (CpuState::WaitInput(x), Some(key)) => {
                self.registers[*x] = key as _;
                self.state = CpuState::Running;
            }
            (CpuState::Halt, _) | (CpuState::WaitInput(_), _) => {}
            (CpuState::Running, _) => {
                self.fetch_and_execute();
                self.update_timers();
            }
        }
        // clear dynamic key events
        std::mem::replace(&mut self.keypad_event, [KeyState::Up; 16]);
    }

    fn any_key_down(&self) -> Option<usize> {
        self.keypad_event.iter().enumerate().find_map(|(i, k)| {
            if *k == KeyState::Down {
                Some(i)
            } else {
                None
            }
        })
    }

    fn update_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }
        if self.st > 0 {
            self.st -= 1;
        }
    }

    fn fetch_and_execute(&mut self) {
        let hi = self.memory[self.pc] as u16;
        let lo = self.memory[self.pc + 1] as u16;

        match Opcode::from(hi << 8 | lo) {
            Opcode::SYS_addr(_addr) => {
                //
                info!("SYS instruction ignored");
            }
            Opcode::CLS => self.clear_display(),
            Opcode::RET => {
                self.pc = self.stack[self.sp] as usize;
                self.sp -= 1;
            }
            Opcode::JP_addr(addr) => self.pc = addr as usize - 2,
            Opcode::CALL_addr(addr) => {
                self.sp += 1;
                self.stack[self.sp as usize] = self.pc as u16;
                self.pc = addr - 2;
            }
            Opcode::SE_Vx_byte(x, b) => {
                if self.registers[x] == b {
                    self.pc += 2;
                }
            }
            Opcode::SNE_Vx_byte(x, b) => {
                if self.registers[x] != b {
                    self.pc += 2;
                }
            }
            Opcode::SE_Vx_Vy(x, y) => {
                if self.registers[x] == self.registers[y] {
                    self.pc += 2;
                }
            }
            Opcode::LD_Vx_byte(x, b) => self.registers[x] = b,
            Opcode::ADD_Vx_byte(x, b) => {
                let tmp = self.registers[x] as u64;
                self.registers[x] = ((tmp + b as u64) & 0xFF) as u8;
                //self.registers[x] += b
            }
            Opcode::LD_Vx_Vy(x, y) => self.registers[x] = self.registers[y],
            Opcode::OR_Vx_Vy(x, y) => self.registers[x] |= self.registers[y],
            Opcode::AND_Vx_Vy(x, y) => self.registers[x] &= self.registers[y],
            Opcode::XOR_Vx_Vy(x, y) => self.registers[x] ^= self.registers[y],
            Opcode::ADD_Vx_Vy(x, y) => {
                let sum = self.registers[x] as u16 + self.registers[y] as u16;
                if sum > 0xFF {
                    self.registers[0xF] = 1;
                }
                self.registers[x] = (sum & 0xFF) as u8;
            }
            Opcode::SUB_Vx_Vy(x, y) => {
                if self.registers[x] > self.registers[y] {
                    self.registers[0xF] = 1;
                    self.registers[x] -= self.registers[y];
                } else {
                    self.registers[0xF] = 0;
                }
            }
            Opcode::SHR_Vx_Vy(x, _y) => {
                self.registers[0xF] = self.registers[x] & 1;
                self.registers[x] >>= 1;
            }
            Opcode::SUBN_Vx_Vy(x, y) => {
                if self.registers[y] > self.registers[x] {
                    self.registers[0xF] = 1;
                    self.registers[y] -= self.registers[x];
                } else {
                    self.registers[0xF] = 0;
                }
            }
            Opcode::SHL_Vx_Vy(x, _y) => {
                self.registers[0xF] = self.registers[x] >> 7;
                self.registers[x] <<= 1;
            }
            Opcode::SNE_Vx_Vy(x, y) => {
                if self.registers[x] != self.registers[y] {
                    self.pc += 2;
                }
            }
            Opcode::LD_I_addr(addr) => self.i = addr as u16,
            Opcode::JP_V0_addr(addr) => self.pc = addr + self.registers[0x0] as usize - 2,
            Opcode::RND_Vx_byte(x, b) => self.registers[x] = rand::random::<u8>() & b,
            Opcode::DRW_Vx_Vy_nibble(x, y, nibble) => self.drw_x_y_nibble(x, y, nibble),
            Opcode::SKP_Vx(x) => {
                if self.keypad[self.registers[x] as usize] == KeyState::Down {
                    self.pc += 2;
                }
            }
            Opcode::SKNP_Vx(x) => {
                if self.keypad[self.registers[x] as usize] == KeyState::Up {
                    self.pc += 2;
                }
            }
            Opcode::LD_Vx_DT(x) => self.registers[x] = self.dt as _,
            Opcode::LD_Vx_K(x) => self.state = CpuState::WaitInput(x),
            Opcode::LD_DT_Vx(x) => self.dt = self.registers[x] as _,
            Opcode::LD_ST_Vx(x) => self.st = self.registers[x] as _,
            Opcode::ADD_I_Vx(x) => self.i += self.registers[x] as u16,
            Opcode::LD_F_Vx(x) => self.i = interpreter::sprite_addr(self.registers[x]),
            Opcode::LD_B_Vx(x) => {
                let digit = self.registers[x] as u32;
                self.memory[self.i as usize + 0] = (digit % 1000) as u8;
                self.memory[self.i as usize + 1] = (digit % 100) as u8;
                self.memory[self.i as usize + 2] = (digit % 10) as u8;
            }
            Opcode::LD_I_Vx(x) => {
                let total = x as usize + 1;
                let offset = self.i as usize;
                dump(&self.registers[..total], &mut self.memory[offset..]);
            }
            Opcode::LD_Vx_I(x) => {
                let start = self.i as usize;
                let total = x as usize + 1;
                dump(
                    &self.memory[start..start + total],
                    &mut self.registers[..total],
                );
            }
        }

        // advance program counter
        self.pc += 2;
    }

    fn clear_display(&mut self) {
        std::mem::replace(&mut self.display, [PixelState::Off; 64 * 32]);
    }

    fn drw_x_y_nibble(&mut self, x: usize, y: usize, nibble: u8) {
        let x = self.registers[x] as usize;
        let y = self.registers[y] as usize;
        let rows = nibble as usize;
        self.registers[0xF] = 0;
        for row in 0..rows {
            let byte = self.memory[self.i as usize + row];
            for bit in 0..8 {
                if byte & (0x80 >> bit as u8) != 0 {
                    let row = (y + row) % 32;
                    let col = (x + bit) % 64;
                    self.display[64 * row + col].toggle();
                    self.registers[0xF] = 1;
                }
            }
        }
    }
}

fn dump(src: &[u8], dst: &mut [u8]) {
    io::copy(&mut io::Cursor::new(src), &mut io::Cursor::new(dst)).expect("Out of bounds");
}
