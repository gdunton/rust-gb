#[cfg(test)]
mod ret_test {
    use crate::gameboy::Gameboy;
    use crate::gameboy::RegisterLabel16;
    use rust_catch::tests;

    tests! {
        test("Ret jumps back to correct place") {
            // 0x00, 0x01, 0x02
            let mut gb = Gameboy::new(vec![0xC9, 0x34, 0x12]); // RET

            // 0x00 : (0xC9) The RET instruction
            // 0x01 : (0x34) The lower byte of the return address
            // 0x02 : (0x12) The higher byte of the return address

            // The stack pointer points to 0x01 which means the value
            // at the top of the stack is 0x1234
            gb.set_register_16(RegisterLabel16::StackPointer, 0x01);

            section("RET takes 16 cycles") {
                let cycles = gb.step_once();
                assert_eq!(cycles, 16);
            }

            section("RET instruction works") {
                let _ = gb.step_once();

                // Program counter is now in the correct place
                assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x1234);

                // Stack pointer is also in the right place
                assert_eq!(gb.get_register_16(RegisterLabel16::StackPointer), 0x03);
            }

        }
    }

    #[test]
    fn run_instruction_is_1_byte_big() {
        // I'm not happy with this use of decode_instruction but
        // I cannot see another way of checking the size of the RET
        // instruction.
        use super::super::super::opcodes::decode_instruction;
        let instructions = vec![0xC9];
        let opcode = decode_instruction(0x00, &instructions).unwrap();
        assert_eq!(opcode.size(), 1);
    }
}
