use super::cb_opcodes::cb_code_to_opcode;

pub fn code_to_opcode(code: u8, program_counter: u16, program_code: &[u8]) -> Result<&str, String> {
    match code {
        0x00 => Ok("NOP"),
        0x05 => Ok("DEC B"),
        0x06 => Ok("LD8 B d8"),
        0x0C => Ok("INC C"),
        0x0E => Ok("LD8 C d8"),
        0x11 => Ok("LD16 DE d16"),
        0x17 => Ok("RLA"),
        0x1A => Ok("LD8 A (DE)"),
        0x20 => Ok("JR NZ r8"),
        0x21 => Ok("LD16 HL d16"),
        0x22 => Ok("LD8 (HL+) A"),
        0x23 => Ok("INC HL"),
        0x31 => Ok("LD16 SP d16"),
        0x32 => Ok("LD8 (HL-) A"),
        0x3E => Ok("LD8 A d8"),
        0x4F => Ok("LD8 C A"),
        0x77 => Ok("LD8 (HL) A"),
        0xAF => Ok("XOR A"),
        0xC1 => Ok("POP BC"),
        0xC5 => Ok("PUSH BC"),
        0xCB => {
            // 0xCB is prefix and the next byte shows the actual instruction
            let cb_instruction = program_code[program_counter as usize + 1];
            return cb_code_to_opcode(cb_instruction);
        }
        0xCD => Ok("CALL a16"),
        0xE0 => Ok("LD8 (a8) A"),
        0xE2 => Ok("LD8 (C) A"),
        _ => Err(format!(
            "Unknown command {:#X} at address: {:#X}",
            code, program_counter
        )),
    }
}
