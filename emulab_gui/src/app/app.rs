use iced::widget::{image, pane_grid, text_editor};
use std::time::Duration;

use crate::components::panes::PaneKind;
use crate::components::panes::editor_pane;
use crate::components::panes::game_pane;

use emulab_lib::core::console::Console;

const DEFAULT_FRAME_RATE_TARGET: Duration = Duration::from_millis(1000 / 60); // 60 FPS
const PANE_RESIZE_LEEWAY_PXS: u32 = 10;

struct AppState {
    game_running: bool,
    console: Box<dyn Console>,
    display_framebuffer: image::Handle,

    panes: pane_grid::State<PaneKind>,

    text_content: text_editor::Content,
}

#[derive(Debug, Clone)]
enum AppMessage {
    FrameTick,
    PaneResized(pane_grid::ResizeEvent),

    TextEdited(text_editor::Action),
}

impl AppState {
    fn new() -> Self {
        let (mut panes, editor_pane) = pane_grid::State::new(PaneKind::EditorPane);
        panes.split(pane_grid::Axis::Vertical, editor_pane, PaneKind::GamePane);

        let console = emulab_lib::nes::make_console();
        let display_framebuffer = {
            let (initial_frame, frame_width, frame_height) = console.get_frame();
            image::Handle::from_rgba(frame_width, frame_height, initial_frame)
        };

        return Self {
            game_running: false,
            console,
            display_framebuffer,

            panes: panes,

            text_content: text_editor::Content::new(),
        };
    }

    fn update(&mut self, message: AppMessage) {
        match message {
            AppMessage::FrameTick => {
                self.console.advance_frame();
                let (frame_image, width, height) = self.console.get_frame();
                self.display_framebuffer =
                    image::Handle::from_rgba(width, height, frame_image.to_vec());
            }

            AppMessage::PaneResized(event) => {
                self.panes.resize(event.split, event.ratio);
            }

            AppMessage::TextEdited(action) => {
                self.text_content.perform(action);
            }
        }
    }

    fn view(&self) -> iced::Element<'_, AppMessage> {
        return pane_grid(&self.panes, |_pane, state, _is_maximized| {
            let content: iced::Element<'_, AppMessage> = match state {
                PaneKind::EditorPane => editor_pane(&self.text_content)
                    .on_action(AppMessage::TextEdited)
                    .into(),
                PaneKind::GamePane => game_pane(&self.display_framebuffer).into(),
            };

            return pane_grid::Content::new(content);
        })
        .on_resize(PANE_RESIZE_LEEWAY_PXS, AppMessage::PaneResized)
        .into();
    }

    fn subscription(&self) -> iced::Subscription<AppMessage> {
        if self.game_running {
            return iced::time::every(DEFAULT_FRAME_RATE_TARGET).map(|_| AppMessage::FrameTick);
        }

        return iced::Subscription::none();
    }
}

pub fn emulab() -> iced::Application<impl iced_program::Program> {
    return iced::application(AppState::new, AppState::update, AppState::view)
        .subscription(AppState::subscription)
        .title("Emulab");
}
