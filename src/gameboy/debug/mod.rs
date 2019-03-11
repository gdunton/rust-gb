use std::collections::HashMap;

pub struct Registers {
    pub registers: HashMap<String, String>,
}

impl Registers {
    pub fn get_register_val(&self, register: &String) -> String {
        self.registers
            .get(register)
            .map(|x| x.clone())
            .unwrap_or("Invalid register".to_string())
    }
}

pub struct Instruction {
    pub address: u16,
    pub opcode: String,
}

impl Instruction {
    pub fn get_address(&self) -> u16 {
        self.address
    }

    pub fn get_opcode(&self) -> String {
        self.opcode.clone()
    }
}
