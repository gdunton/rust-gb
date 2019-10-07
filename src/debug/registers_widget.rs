use crate::debug::instrumentation::get_registers;
use crate::gameboy::Gameboy;
use crate::layout::Print;

pub struct RegistersWidget<'a> {
    gb: &'a Gameboy,
}

impl<'a> RegistersWidget<'a> {
    pub fn new(gb: &'a Gameboy) -> RegistersWidget {
        RegistersWidget { gb }
    }
}

impl<'a> Print for RegistersWidget<'a> {
    fn print(&self) -> Vec<String> {
        let registers = get_registers(&self.gb);
        let register_order = vec![
            "A", "F", "AF", "B", "C", "BC", "D", "E", "DE", "H", "L", "HL", "PC", "SP",
        ];

        let mut output = Vec::new();

        output.push(String::from("----------------"));
        output.push(format!("{:<#width$} : {}", "Register", "Value", width = 8));
        output.push(String::from("----------------"));

        for register in register_order {
            output.push(format!(
                "{:<#width$} : {}",
                register,
                registers.get_register_val(&register.to_string()),
                width = 8
            ));
        }
        output
    }
}