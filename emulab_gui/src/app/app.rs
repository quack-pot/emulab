use iced::widget::pane_grid;
use iced::widget::text_editor;
use std::time::Duration;

use crate::components::panes::PaneKind;
use crate::components::panes::editor_pane;
use crate::components::panes::game_pane;

const DEFAULT_FRAME_RATE_TARGET: Duration = Duration::from_millis(1000 / 60); // 60 FPS
const PANE_RESIZE_LEEWAY_PXS: u32 = 10;

struct AppState {
    game_running: bool,
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

        return Self {
            game_running: false,
            panes: panes,

            text_content: text_editor::Content::new(),
        };
    }

    fn update(&mut self, message: AppMessage) {
        match message {
            AppMessage::FrameTick => {
                // TODO: Advance the game state and update the screen image
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
                PaneKind::GamePane => game_pane(256, 240).into(),
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
