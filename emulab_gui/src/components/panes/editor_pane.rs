use iced::widget::text_editor;

pub struct EditorPane<'a, Message> {
    content: &'a text_editor::Content,
    on_action: Option<Box<dyn Fn(text_editor::Action) -> Message + 'a>>,
}

impl<'a, Message> EditorPane<'a, Message> {
    pub fn new(content: &'a text_editor::Content) -> Self {
        return Self {
            content,
            on_action: None,
        };
    }

    pub fn on_action<F>(mut self, f: F) -> Self
    where
        F: Fn(text_editor::Action) -> Message + 'a,
    {
        self.on_action = Some(Box::new(f));
        return self;
    }
}

impl<'a, Message: Clone + 'a> From<EditorPane<'a, Message>> for iced::Element<'a, Message> {
    fn from(comp: EditorPane<'a, Message>) -> Self {
        let mut editor = text_editor(&comp.content);

        if let Some(on_action) = comp.on_action {
            editor = editor.on_action(on_action);
        }

        return editor.into();
    }
}
