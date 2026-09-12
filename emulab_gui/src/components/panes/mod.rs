mod editor_pane;
mod game_pane;

use iced::widget::text_editor;

pub enum PaneKind {
    GamePane,
    EditorPane,
}

pub fn editor_pane<'a, Message>(
    content: &'a text_editor::Content,
) -> editor_pane::EditorPane<'a, Message> {
    return editor_pane::EditorPane::new(content);
}

pub fn game_pane<'a, Message>(width: u32, height: u32) -> game_pane::GamePane<'a, Message> {
    assert!(width > 0, "Game screen width must be greater than zero.");
    assert!(height > 0, "Game screen height must be greater than zero.");

    return game_pane::GamePane::new(width, height);
}
