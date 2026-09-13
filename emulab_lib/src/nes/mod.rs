mod console;

use crate::core::console::Console;

pub fn make_console() -> Box<impl Console> {
    return console::ConsoleNES::new();
}
