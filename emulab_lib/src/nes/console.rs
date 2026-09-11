use crate::core::console::Console;
use crate::core::utils;

const NES_WORK_RAM_SIZE_BYTES: usize = 2048; // 2 KiB
const NES_VIDEO_RAM_SIZE_BYTES: usize = 2048; // 2 KiB
const NES_SPRITE_RAM_SIZE_BYTES: usize = 256; // 256 Bytes
const NES_MAX_CARTRIDGE_RAM_SIZE_BYTES: usize = 8192; // 8 KiB

const NES_MAX_PROGRAM_SIZE_BYTES: usize = 524288; // 512 KiB

const NES_NTSC_FRAME_CLOCK_CYCLE_BUDGET: f64 = 29780.5; // Using a floating point to be as accurate as possible.

#[repr(u8)]
enum StatusFlags {
    Carry = 1 << 0,
    Zero = 1 << 1,
    InterruptDisable = 1 << 2,
    Decimal = 1 << 3,
    NA1 = 1 << 4, // Unused
    NA2 = 1 << 5, // Unused
    Overflow = 1 << 6,
    Negative = 1 << 7,
}

#[derive(Debug, Copy, Clone)]
#[repr(u32)]
enum PhysicalActions {
    Reset,
}

impl TryFrom<u32> for PhysicalActions {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            x if x == PhysicalActions::Reset as u32 => Ok(PhysicalActions::Reset),
            _ => Err(()),
        }
    }
}

pub struct ConsoleNES {
    reg_a: u8,
    reg_x: u8,
    reg_y: u8,

    status: u8,
    stack_pointer: u8,
    program_counter: u8,

    // Kept distinct to help with debugging later
    work_ram: [u8; NES_WORK_RAM_SIZE_BYTES],
    video_ram: [u8; NES_VIDEO_RAM_SIZE_BYTES],
    sprite_ram: [u8; NES_SPRITE_RAM_SIZE_BYTES],
    cartridge_ram: [u8; NES_MAX_CARTRIDGE_RAM_SIZE_BYTES],

    program: [u8; NES_MAX_PROGRAM_SIZE_BYTES],

    frame_budget: f64,
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

            work_ram: [0u8; NES_WORK_RAM_SIZE_BYTES],
            video_ram: [0u8; NES_VIDEO_RAM_SIZE_BYTES],
            sprite_ram: [0u8; NES_SPRITE_RAM_SIZE_BYTES],
            cartridge_ram: [0u8; NES_MAX_CARTRIDGE_RAM_SIZE_BYTES],

            program: [0u8; NES_MAX_PROGRAM_SIZE_BYTES],

            frame_budget: 0.0f64,
        };
    }
}

impl Console for ConsoleNES {
    fn restart(&mut self) {
        // Random values set to simulate real hardware
        self.reg_a = rand::random();
        self.reg_x = rand::random();
        self.reg_y = rand::random();

        utils::fill_random(&mut self.work_ram);
        utils::fill_random(&mut self.video_ram);
        utils::fill_random(&mut self.sprite_ram);
        utils::fill_random(&mut self.cartridge_ram);

        self.status = StatusFlags::InterruptDisable as u8;
        self.stack_pointer = 0xFF - 3;
        self.program_counter = 0u8;

        // Keep the program in the same state (assume cartridge still in)

        self.frame_budget = 0.0f64;
    }

    fn run_console_action(&mut self, action_code: u32) {
        match PhysicalActions::try_from(action_code) {
            Ok(PhysicalActions::Reset) => {}
            Err(_) => {} // Unknown action
        }
    }

    fn advance_frame(&mut self) {
        let mut instruction_time: f64 = 0.0f64;
        self.frame_budget += NES_NTSC_FRAME_CLOCK_CYCLE_BUDGET;

        while instruction_time < self.frame_budget {
            let op_code = self.program[self.program_counter as usize];

            // TODO: Fetch next instruction
            instruction_time = op_code as f64 * 5.0; // TODO: Get time from instruction and CPU state

            // TODO: Execute the next instruction and advance the program counter as needed

            self.frame_budget -= instruction_time;
        }
    }

    fn get_frame(&self) {}

    fn load_program(&mut self, filepath: String) {}
}
