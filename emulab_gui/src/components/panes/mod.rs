mod editor_pane;
mod game_pane;

use iced::widget::{image, text_editor};

pub enum PaneKind {
    GamePane,
    EditorPane,
}

pub fn editor_pane<'a, Message>(
    content: &'a text_editor::Content,
) -> editor_pane::EditorPane<'a, Message> {
    return editor_pane::EditorPane::new(content);
}

pub fn game_pane<'a, Message>(framebuffer: &'a image::Handle) -> game_pane::GamePane<'a, Message> {
    return game_pane::GamePane::new(framebuffer);
}
