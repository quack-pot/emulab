// use iced::widget::pane_grid;
use std::time::Duration;

const DEFAULT_FRAME_RATE_TARGET: Duration = Duration::from_millis(1000 / 60); // 60 FPS

struct AppState {}

#[derive(Debug, Clone)]
enum AppMessage {}

impl AppState {
    fn new() -> Self {
        return Self {};
    }

    fn update(&mut self, message: AppMessage) {
        match message {}
    }

    fn view(&self) -> iced::Element<'_, AppMessage> {
        return iced::widget::text("Hello World!").into();
    }

    fn subscription(&self) -> iced::Subscription<AppMessage> {
        return iced::Subscription::none();
    }
}

pub fn make_app() -> iced::Application<impl iced_program::Program> {
    return iced::application(AppState::new, AppState::update, AppState::view)
        .subscription(AppState::subscription)
        .title("Emulab");
}
