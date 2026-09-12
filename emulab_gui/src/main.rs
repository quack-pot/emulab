mod app;
mod components;

use crate::app::app::emulab;

fn main() -> iced::Result {
    return emulab().run();
}
