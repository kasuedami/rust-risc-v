pub const LUI: u32 = 0b0110111;
pub const AUIPC: u32 = 0b0010111;
pub const JAL: u32 = 0b1101111;
pub const JALR: u32 = 0b1100111;
pub const BRANCH: u32 = 0b1100011;
pub const LOAD: u32 = 0b0000011;
pub const STORE: u32 = 0b0100011;
pub const MATHI: u32 = 0b0010011;
pub const MATH: u32 = 0b0110011;
pub const FENCE: u32 = 0b0001111;
pub const CSR: u32 = 0b1110011;

pub fn lui(destination: u8, value: u32) -> u32 {
    let truncated_value = value & 0xFFFFF000;
    let shifted_destination = (destination as u32) << 7;

    truncated_value | shifted_destination | LUI
}
