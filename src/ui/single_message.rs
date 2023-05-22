use crate::ui::utils::CenteredText;
use nanos_sdk::buttons::{ButtonEvent, ButtonsState};
use nanos_ui::bagls::{RIGHT_ARROW, RIGHT_S_ARROW};
use nanos_ui::layout::Draw;
use nanos_ui::ui::{clear_screen, get_event};

pub struct SingleMessage<'a> {
    message: &'a str,
    bold: bool,
    show_right_arrow: bool,
}

impl<'a> SingleMessage<'a> {
    pub fn new(message: &'a str) -> Self {
        SingleMessage {
            message,
            bold: false,
            show_right_arrow: false,
        }
    }

    pub fn with_right_arrow(message: &'a str) -> Self {
        SingleMessage {
            message,
            bold: false,
            show_right_arrow: true,
        }
    }

    pub fn show(&self) {
        clear_screen();
        self.message.draw_centered(self.bold);
        if self.show_right_arrow {
            RIGHT_ARROW.display();
            RIGHT_S_ARROW.display();
        }
    }

    pub fn show_and_wait(&self) {
        let mut buttons = ButtonsState::new();

        self.show();

        loop {
            match get_event(&mut buttons) {
                Some(ButtonEvent::LeftButtonRelease)
                | Some(ButtonEvent::RightButtonRelease)
                | Some(ButtonEvent::BothButtonsRelease) => return,
                _ => (),
            }
        }
    }
}
