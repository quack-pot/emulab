use crate::core::console::Console;

#[repr(u8)]
enum StatusFlags {
    Carry = 1 << 0,
    Zero = 1 << 1,
    InterruptDisable = 1 << 2,
    Decimal = 1 << 3,
    NA1 = 1 << 4,
    NA2 = 1 << 5,
    Overflow = 1 << 6,
    Negative = 1 << 7,
}

pub struct ConsoleNES {
    reg_a: u8,
    reg_x: u8,
    reg_y: u8,

    status: u8,
    stack_pointer: u8,
    program_counter: u8,
}

impl ConsoleNES {
    pub fn new() -> Self {
        return Self {
            reg_a: 0u8,
            reg_x: 0u8,
            reg_y: 0u8,

            status: 0u8,
            stack_pointer: 0u8,
            program_counter: 0u8,
        };
    }
}

impl Console for ConsoleNES {
    fn reset(&mut self) {
        self.reg_a = 0u8;
        self.reg_x = 0u8;
        self.reg_y = 0u8;

        self.status = StatusFlags::InterruptDisable as u8;
        self.stack_pointer = 0u8;
        self.program_counter = 0u8;
    }
}
