#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Opcode {
    // 0nnn - SYS addr
    SYS_addr(usize),
    // 00E0 - CLS
    CLS,
    // 00EE - RET
    RET,
    // 1nnn - JP addr
    JP_addr(usize),
    // 2nnn - CALL addr
    CALL_addr(usize),
    // 3xkk - SE Vx, byte
    SE_Vx_byte(usize, u8),
    // 4xkk - SNE Vx, byte
    SNE_Vx_byte(usize, u8),
    // 5xy0 - SE Vx, Vy
    SE_Vx_Vy(usize, usize),
    // 6xkk - LD Vx, byte
    LD_Vx_byte(usize, u8),
    // 7xkk - ADD Vx, byte
    ADD_Vx_byte(usize, u8),
    // 8xy0 - LD Vx, Vy
    LD_Vx_Vy(usize, usize),
    // 8xy1 - OR Vx, Vy
    OR_Vx_Vy(usize, usize),
    // 8xy2 - AND Vx, Vy
    AND_Vx_Vy(usize, usize),
    // 8xy3 - XOR Vx, Vy
    XOR_Vx_Vy(usize, usize),
    // 8xy4 - ADD Vx, Vy
    ADD_Vx_Vy(usize, usize),
    // 8xy5 - SUB Vx, Vy
    SUB_Vx_Vy(usize, usize),
    // 8xy6 - SHR Vx {, Vy}
    SHR_Vx_Vy(usize, usize),
    // 8xy7 - SUBN Vx, Vy
    SUBN_Vx_Vy(usize, usize),
    // 8xyE - SHL Vx {, Vy}
    SHL_Vx_Vy(usize, usize),
    // 9xy0 - SNE Vx, Vy
    SNE_Vx_Vy(usize, usize),
    // Annn - LD I, addr
    LD_I_addr(usize),
    // Bnnn - JP V0, addr
    JP_V0_addr(usize),
    // Cxkk - RND Vx, byte
    RND_Vx_byte(usize, u8),
    // Dxyn - DRW Vx, Vy, nibble
    DRW_Vx_Vy_nibble(usize, usize, u8),
    // Ex9E - SKP Vx
    SKP_Vx(usize),
    // ExA1 - SKNP Vx
    SKNP_Vx(usize),
    // Fx07 - LD Vx, DT
    LD_Vx_DT(usize),
    // Fx0A - LD Vx, K
    LD_Vx_K(usize),
    // Fx15 - LD DT, Vx
    LD_DT_Vx(usize),
    // Fx18 - LD ST, Vx
    LD_ST_Vx(usize),
    // Fx1E - ADD I, Vx
    ADD_I_Vx(usize),
    // Fx29 - LD F, Vx
    LD_F_Vx(usize),
    // Fx33 - LD B, Vx
    LD_B_Vx(usize),
    // Fx55 - LD [I], Vx
    LD_I_Vx(usize),
    // Fx65 - LD Vx, [I]
    LD_Vx_I(usize),
}

// #[inline]
// fn dec_xy_(op: u16) -> (usize, usize) {
//     (((op >> 8) & 0xF) as usize, ((op >> 4) / 0xF) as usize)
// }

#[inline]
fn dec_xkk(op: u16) -> (usize, u8) {
    (((op >> 8) & 0xF) as usize, (op & 0xFF) as u8)
}

#[inline]
fn dec_nnn(op: u16) -> usize {
    (op & 0xFFF) as usize
}

// #[inline]
// fn dec_x__(op: u16) -> usize {
//     ((op >> 8) & 0xF) as usize
// }

#[inline]
fn dec_xyn(op: u16) -> (usize, usize, u8) {
    (
        ((op >> 8) & 0xF) as usize,
        ((op >> 4) & 0xF) as usize,
        (op & 0xF) as u8,
    )
}

impl From<u16> for Opcode {
    fn from(op: u16) -> Self {
        let (x, y, n) = dec_xyn(op);
        let (_, kk) = dec_xkk(op);
        let nnn = dec_nnn(op);
        match op {
            0x00E0 => Opcode::CLS,
            0x00EE => Opcode::RET,
            op if op & 0xF000 == 0x0000 => Opcode::SYS_addr(nnn),
            op if op & 0xF000 == 0x1000 => Opcode::JP_addr(nnn),
            op if op & 0xF000 == 0x2000 => Opcode::CALL_addr(nnn),
            op if op & 0xF000 == 0x3000 => Opcode::SE_Vx_byte(x, kk),
            op if op & 0xF000 == 0x4000 => Opcode::SNE_Vx_byte(x, kk),
            op if op & 0xF00F == 0x5000 => Opcode::SE_Vx_Vy(x, y),
            op if op & 0xF000 == 0x6000 => Opcode::LD_Vx_byte(x, kk),
            op if op & 0xF000 == 0x7000 => Opcode::ADD_Vx_byte(x, kk),
            op if op & 0xF00F == 0x8000 => Opcode::LD_Vx_Vy(x, y),
            op if op & 0xF00F == 0x8001 => Opcode::OR_Vx_Vy(x, y),
            op if op & 0xF00F == 0x8002 => Opcode::AND_Vx_Vy(x, y),
            op if op & 0xF00F == 0x8003 => Opcode::XOR_Vx_Vy(x, y),
            op if op & 0xF00F == 0x8004 => Opcode::ADD_Vx_Vy(x, y),
            op if op & 0xF00F == 0x8005 => Opcode::SUB_Vx_Vy(x, y),
            op if op & 0xF00F == 0x8006 => Opcode::SHR_Vx_Vy(x, y),
            op if op & 0xF00F == 0x8007 => Opcode::SUBN_Vx_Vy(x, y),
            op if op & 0xF00F == 0x800E => Opcode::SHL_Vx_Vy(x, y),
            op if op & 0xF00F == 0x9000 => Opcode::SNE_Vx_Vy(x, y),
            op if op & 0xF000 == 0xA000 => Opcode::LD_I_addr(nnn),
            op if op & 0xF000 == 0xB000 => Opcode::JP_V0_addr(nnn),
            op if op & 0xF000 == 0xC000 => Opcode::RND_Vx_byte(x, kk),
            op if op & 0xF000 == 0xD000 => Opcode::DRW_Vx_Vy_nibble(x, y, n),
            op if op & 0xF0FF == 0xE09E => Opcode::SKP_Vx(x),
            op if op & 0xF0FF == 0xE0A1 => Opcode::SKNP_Vx(x),
            op if op & 0xF0FF == 0xF007 => Opcode::LD_Vx_DT(x),
            op if op & 0xF0FF == 0xF00A => Opcode::LD_Vx_K(x),
            op if op & 0xF0FF == 0xF015 => Opcode::LD_DT_Vx(x),
            op if op & 0xF0FF == 0xF018 => Opcode::LD_ST_Vx(x),
            op if op & 0xF0FF == 0xF01E => Opcode::ADD_I_Vx(x),
            op if op & 0xF0FF == 0xF029 => Opcode::LD_F_Vx(x),
            op if op & 0xF0FF == 0xF033 => Opcode::LD_B_Vx(x),
            op if op & 0xF0FF == 0xF055 => Opcode::LD_I_Vx(x),
            op if op & 0xF0FF == 0xF065 => Opcode::LD_Vx_I(x),
            _ => panic!("Unknown opcode = {:X?}", op),
        }
    }
}

#[cfg(test)]
mod test {
    // TODO some much needed tests.
}
