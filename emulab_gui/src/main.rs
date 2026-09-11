mod app;

use crate::app::app::make_app;

fn main() -> iced::Result {
    return make_app().run();
}
