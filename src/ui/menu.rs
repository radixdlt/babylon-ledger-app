use ledger_device_sdk::buttons::{ButtonEvent, ButtonsState};
use ledger_device_sdk::ui::bagls::{Icon, LEFT_ARROW, LEFT_S_ARROW, RIGHT_ARROW, RIGHT_S_ARROW};
use ledger_device_sdk::ui::gadgets::{clear_screen, get_event};
use ledger_device_sdk::ui::layout::Draw;
use ledger_device_sdk::ui::screen_util::screen_update;

use crate::io::UxEvent;
use crate::ui::utils::{CenteredText, LeftAlignedMiddle};

pub enum MenuFeature<'a> {
    Plain,
    Icon(&'a Icon<'a>),
    OnOffState(fn() -> bool),
}

pub struct MenuItem<'a> {
    text: &'a str,
    action: fn() -> bool,
    feature: MenuFeature<'a>,
}

impl<'a> MenuItem<'a> {
    pub fn new(feature: MenuFeature<'a>, text: &'a str, action: fn() -> bool) -> Self {
        MenuItem {
            text,
            action,
            feature,
        }
    }
}

pub struct Menu<'a> {
    items: &'a [MenuItem<'a>],
    current: usize,
}

const HALF_ICON_WIDTH: usize = 7;

const ON_TEXT: &str = "\n\nEnabled";
const OFF_TEXT: &str = "\n\nDisabled";

impl<'a> Menu<'a> {
    pub const fn new(items: &'a [MenuItem<'a>]) -> Self {
        Menu { items, current: 0 }
    }

    pub fn display(&self) {
        clear_screen();

        let item = &self.items[self.current];

        item.text.draw_centered(true);

        match item.feature {
            MenuFeature::Plain => {}
            MenuFeature::Icon(icon) => icon.draw_left_aligned_middle(),
            MenuFeature::OnOffState(getter) => {
                if (getter)() { ON_TEXT } else { OFF_TEXT }.draw_centered(false)
            }
        }

        LEFT_ARROW.display();
        RIGHT_ARROW.display();

        screen_update();
    }

    pub fn event_loop(&mut self) {
        let mut buttons = ButtonsState::new();

        self.display();

        loop {
            let event = get_event(&mut buttons);

            if let Some(event) = event {
                UxEvent::wakeup();
                if self.handle(event) {
                    break;
                }
            }
        }
    }

    pub fn handle(&mut self, event: ButtonEvent) -> bool {
        match event {
            ButtonEvent::LeftButtonPress => {
                LEFT_S_ARROW.instant_display();
                false
            }
            ButtonEvent::RightButtonPress => {
                RIGHT_S_ARROW.instant_display();
                false
            }
            ButtonEvent::BothButtonsPress => {
                LEFT_S_ARROW.instant_display();
                RIGHT_S_ARROW.instant_display();
                false
            }
            ButtonEvent::LeftButtonRelease => {
                LEFT_S_ARROW.erase();
                self.current = if self.current > 0 {
                    self.current - 1
                } else {
                    self.items.len() - 1
                };
                self.display();
                false
            }
            ButtonEvent::RightButtonRelease => {
                RIGHT_S_ARROW.erase();
                self.current = if self.current < self.items.len() - 1 {
                    self.current + 1
                } else {
                    0
                };
                self.display();
                false
            }
            ButtonEvent::BothButtonsRelease => {
                LEFT_S_ARROW.erase();
                RIGHT_S_ARROW.erase();
                let result = (self.items[self.current].action)();
                self.display();
                result
            }
        }
    }
}
