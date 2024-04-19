use ledger_device_sdk::buttons::{ButtonEvent, ButtonsState};
use ledger_device_sdk::ui::bagls::{Icon, RIGHT_ARROW, RIGHT_S_ARROW};
use ledger_device_sdk::ui::gadgets::{clear_screen, get_event};
use ledger_device_sdk::ui::layout::Draw;
use ledger_device_sdk::ui::SCREEN_WIDTH;

use crate::io::UxEvent;
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

    pub fn with_bold(message: &'a str) -> Self {
        SingleMessage {
            message,
            bold: true,
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
                Icon {
                    icon: icon.icon,
                    pos: ((SCREEN_WIDTH / 2) as i16, -1),
                }
                .display();
            }
        }
    }

    pub fn show_and_wait(&self) {
        let mut buttons = ButtonsState::new();

        self.show();

        loop {
            let event = get_event(&mut buttons);

            if event.is_some() {
                UxEvent::wakeup();
            }

            match event {
                Some(ButtonEvent::LeftButtonRelease)
                | Some(ButtonEvent::RightButtonRelease)
                | Some(ButtonEvent::BothButtonsRelease) => return,
                _ => (),
            }
        }
    }

    pub fn show_and_wait_both_click(&self) {
        let mut buttons = ButtonsState::new();

        self.show();

        loop {
            let event = get_event(&mut buttons);

            if event.is_some() {
                UxEvent::wakeup();
            }

            if let Some(ButtonEvent::BothButtonsRelease) = event {
                return;
            }
        }
    }
}
