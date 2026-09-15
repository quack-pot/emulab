pub struct InstructionResultNES {
    pub next_program_counter: u16,
    pub next_mapped_program_counter: u32,
}

pub struct InstructionSetNES {}

impl InstructionSetNES {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn get_instruction_cycles(&self, op_code: u8, arg1: u8, arg2: u8) -> f64 {
        return 5.0f64;
    }

    pub fn process_instruction(&mut self, op_code: u8, arg1: u8, arg2: u8) -> InstructionResultNES {
        // TODO: Write this lol
        return InstructionResultNES {
            next_program_counter: 0,
            next_mapped_program_counter: 0,
        };
    }
}
