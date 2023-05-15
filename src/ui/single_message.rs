use nanos_sdk::buttons::{ButtonEvent, ButtonsState};
use nanos_ui::bagls::EYE_ICON;
use nanos_ui::layout::{Draw, Layout, Location, StringPlace};
use nanos_ui::ui::{clear_screen, get_event};
use nanos_ui::PADDING;

const EYE_ICON_HEIGHT: usize = 14;

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

        self.message
            .split('\n')
            .take(3)
            .enumerate()
            .for_each(|(index, line)| {
                line.place(
                    match index {
                        0 => Location::Top,
                        1 => Location::Middle,
                        2 => Location::Bottom,
                        _ => unreachable!(),
                    },
                    Layout::Centered,
                    self.bold,
                )
            });

        EYE_ICON
            .set_x(PADDING as i16)
            .set_y(Location::Middle.get_y(EYE_ICON_HEIGHT) as i16)
            .display();
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
