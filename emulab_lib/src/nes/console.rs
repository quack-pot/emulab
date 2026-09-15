use crate::core::console::Console;
use crate::core::utils;

use crate::nes::instructions::InstructionSetNES;

const NES_WORK_RAM_SIZE_BYTES: usize = 2048; // 2 KiB
const NES_VIDEO_RAM_SIZE_BYTES: usize = 2048; // 2 KiB
const NES_SPRITE_RAM_SIZE_BYTES: usize = 256; // 256 Bytes
const NES_MAX_CARTRIDGE_RAM_SIZE_BYTES: usize = 8192; // 8 KiB

const NES_MAX_PROGRAM_SIZE_BYTES: usize = 524288; // 512 KiB

const NES_NTSC_FRAME_CLOCK_CYCLE_BUDGET: f64 = 29780.5; // Using a floating point to be as accurate as possible.

const NES_FRAME_WIDTH: u32 = 256;
const NES_FRAME_HEIGHT: u32 = 240;
const NES_FRAME_RGBA_SIZE_BYTES: usize = (NES_FRAME_WIDTH * NES_FRAME_HEIGHT * 4) as usize;

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
    program_counter: u16,
    mapped_program_counter: u32, // Points the real instruction (simulates mapper chip)

    // Kept distinct to help with debugging later
    work_ram: Box<[u8; NES_WORK_RAM_SIZE_BYTES]>,
    video_ram: Box<[u8; NES_VIDEO_RAM_SIZE_BYTES]>,
    sprite_ram: Box<[u8; NES_SPRITE_RAM_SIZE_BYTES]>,
    cartridge_ram: Box<[u8; NES_MAX_CARTRIDGE_RAM_SIZE_BYTES]>,

    program: Box<[u8; NES_MAX_PROGRAM_SIZE_BYTES]>,
    program_size: u32,

    frame_budget: f64,
    image_rgba: Box<[u8; NES_FRAME_RGBA_SIZE_BYTES]>,

    instruction_set: InstructionSetNES,
}

impl ConsoleNES {
    pub fn new() -> Box<Self> {
        return Box::new(Self {
            reg_a: 0u8,
            reg_x: 0u8,
            reg_y: 0u8,

            status: 0u8,
            stack_pointer: 0u8,
            program_counter: 0u16,
            mapped_program_counter: 0u32,

            work_ram: vec![0u8; NES_WORK_RAM_SIZE_BYTES]
                .into_boxed_slice()
                .try_into()
                .unwrap(),

            video_ram: vec![0u8; NES_VIDEO_RAM_SIZE_BYTES]
                .into_boxed_slice()
                .try_into()
                .unwrap(),

            sprite_ram: vec![0u8; NES_SPRITE_RAM_SIZE_BYTES]
                .into_boxed_slice()
                .try_into()
                .unwrap(),

            cartridge_ram: vec![0u8; NES_MAX_CARTRIDGE_RAM_SIZE_BYTES]
                .into_boxed_slice()
                .try_into()
                .unwrap(),

            program: vec![0u8; NES_MAX_PROGRAM_SIZE_BYTES]
                .into_boxed_slice()
                .try_into()
                .unwrap(),
            program_size: 0u32,

            frame_budget: 0.0f64,
            image_rgba: utils::blank_fixed_rgba(),

            instruction_set: InstructionSetNES::new(),
        });
    }
}

impl Console for ConsoleNES {
    fn restart(&mut self) {
        // Random values set to simulate real hardware
        self.reg_a = rand::random();
        self.reg_x = rand::random();
        self.reg_y = rand::random();

        utils::fill_random(&mut *self.work_ram);
        utils::fill_random(&mut *self.video_ram);
        utils::fill_random(&mut *self.sprite_ram);
        utils::fill_random(&mut *self.cartridge_ram);

        self.status = StatusFlags::InterruptDisable as u8;
        self.stack_pointer = 0xFF - 3;
        self.program_counter = 0u16;
        self.mapped_program_counter = 0u32;

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
        self.frame_budget += NES_NTSC_FRAME_CLOCK_CYCLE_BUDGET;

        loop {
            let op_code = self.program[(self.mapped_program_counter % self.program_size) as usize];
            let arg1 =
                self.program[((self.mapped_program_counter + 1) % self.program_size) as usize];
            let arg2 =
                self.program[((self.mapped_program_counter + 2) % self.program_size) as usize];

            let instruction_time = self
                .instruction_set
                .get_instruction_cycles(op_code, arg1, arg2);

            if instruction_time > self.frame_budget {
                break;
            }

            let result = self
                .instruction_set
                .process_instruction(op_code, arg1, arg2);

            self.program_counter = result.next_program_counter;
            self.mapped_program_counter = result.next_mapped_program_counter;
            self.frame_budget -= instruction_time;
        }
    }

    fn get_frame(&self) -> (Box<[u8]>, u32, u32) {
        return (self.image_rgba.clone(), NES_FRAME_WIDTH, NES_FRAME_HEIGHT);
    }

    fn load_program(&mut self, filepath: String) {}
}
