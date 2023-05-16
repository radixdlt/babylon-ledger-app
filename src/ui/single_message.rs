use nanos_sdk::buttons::{ButtonEvent, ButtonsState};
use nanos_ui::ui::{clear_screen, get_event};
use crate::ui::utils::{CenteredText};

pub struct SingleMessage<'a> {
    message: &'a str,
    bold: bool,
}

impl<'a> SingleMessage<'a> {
    pub fn new(message: &'a str, bold: bool) -> Self {
        SingleMessage { message, bold }
    }

    pub fn show(&self) {
        clear_screen();
        self.message.draw_centered(self.bold);
    }
    /// Display the message and wait
    /// for any kind of button release
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
