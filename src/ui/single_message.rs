use nanos_sdk::buttons::{ButtonEvent, ButtonsState};
use nanos_ui::bagls::{Icon, RIGHT_ARROW, RIGHT_S_ARROW};
use nanos_ui::layout::Draw;
use nanos_ui::ui::{clear_screen, get_event};
use nanos_ui::SCREEN_WIDTH;

use crate::ui::utils::CenteredText;

pub enum MessageFeature<'a> {
    Plain,
    WithRightArrow,
    WithIcon(Icon<'a>),
}

pub struct SingleMessage<'a> {
    message: &'a str,
    bold: bool,
    feature: MessageFeature<'a>,
}

impl<'a> SingleMessage<'a> {
    pub fn new(message: &'a str) -> Self {
        SingleMessage {
            message,
            bold: false,
            feature: MessageFeature::Plain,
        }
    }

    pub fn with_right_arrow(message: &'a str) -> Self {
        SingleMessage {
            message,
            bold: false,
            feature: MessageFeature::WithRightArrow,
        }
    }

    pub fn with_icon(message: &'a str, icon: Icon<'a>) -> Self {
        SingleMessage {
            message,
            bold: false,
            feature: MessageFeature::WithIcon(icon),
        }
    }

    pub fn show(&self) {
        clear_screen();
        self.message.draw_centered(self.bold);

        match &self.feature {
            MessageFeature::Plain => {}
            MessageFeature::WithRightArrow => {
                RIGHT_ARROW.display();
                RIGHT_S_ARROW.display();
            }
            MessageFeature::WithIcon(icon) => {
                let new_icon = Icon {
                    icon: icon.icon,
                    pos: ((SCREEN_WIDTH / 2) as i16, 0),
                };

                new_icon.display();
            }
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
