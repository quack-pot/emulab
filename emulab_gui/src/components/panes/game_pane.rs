use iced::widget::{image, mouse_area};

pub struct GamePane<'a, Message> {
    framebuffer: &'a image::Handle,

    on_focus: Option<Box<dyn Fn() -> Message + 'a>>,
}

impl<'a, Message> GamePane<'a, Message> {
    pub fn new(framebuffer: &'a image::Handle) -> Self {
        return Self {
            framebuffer,
            on_focus: None,
        };
    }
}

impl<'a, Message: Clone + 'a> From<GamePane<'a, Message>> for iced::Element<'a, Message> {
    fn from(comp: GamePane<'a, Message>) -> Self {
        let screen = image(comp.framebuffer);
        let mut area = mouse_area(screen);

        if let Some(on_focus) = comp.on_focus {
            area = area.on_press(on_focus());
        }

        return area.into();
    }
}
