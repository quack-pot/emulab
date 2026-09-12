use iced::widget::{image, mouse_area};

pub struct GamePane<'a, Message> {
    framebuffer: image::Handle,
    on_focus: Option<Box<dyn Fn() -> Message + 'a>>,
}

impl<'a, Message> GamePane<'a, Message> {
    pub fn new(width: u32, height: u32) -> Self {
        let image_byte_count: usize = (width * height * 4) as usize;
        let mut blank = vec![0u8; image_byte_count];

        for idx in (3..image_byte_count).step_by(4) {
            blank[idx] = 255u8;
        }

        return Self {
            framebuffer: image::Handle::from_rgba(width, height, blank),
            on_focus: None,
        };
    }
}

impl<'a, Message: Clone + 'a> From<GamePane<'a, Message>> for iced::Element<'a, Message> {
    fn from(comp: GamePane<'a, Message>) -> Self {
        let screen = image(comp.framebuffer.clone());
        let mut area = mouse_area(screen);

        if let Some(on_focus) = comp.on_focus {
            area = area.on_press(on_focus());
        }

        return area.into();
    }
}
