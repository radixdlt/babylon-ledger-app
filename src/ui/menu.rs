use nanos_sdk::buttons::ButtonEvent;
use nanos_ui::bagls::{Icon, LEFT_ARROW, LEFT_S_ARROW, RIGHT_ARROW, RIGHT_S_ARROW};
use nanos_ui::layout::Draw;
use nanos_ui::screen_util::screen_update;
use nanos_ui::ui::clear_screen;

use crate::ui::utils::{CenteredText, LeftAlignedMiddle};

pub struct MenuItem<'a> {
    text: &'a str,
    icon: &'a Icon<'a>,
    action: fn() -> (),
}

impl<'a> MenuItem<'a> {
    pub fn new(icon: &'a Icon<'a>, text: &'a str, action: fn() -> ()) -> Self {
        MenuItem { text, icon, action }
    }
}

pub struct Menu<'a> {
    items: &'a [MenuItem<'a>],
    current: usize,
}

const HALF_ICON_WIDTH: usize = 7;

impl<'a> Menu<'a> {
    pub const fn new(items: &'a [MenuItem<'a>]) -> Self {
        Menu { items, current: 0 }
    }

    pub fn display(&self) {
        clear_screen();

        self.items[self.current].icon.draw_left_aligned_middle();
        self.items[self.current].text.draw_centered(true);

        if self.current > 0 {
            LEFT_ARROW.display();
        }

        if self.current < self.items.len() - 1 {
            RIGHT_ARROW.display();
        }

        screen_update();
    }

    pub fn handle(&mut self, event: ButtonEvent) {
        match event {
            ButtonEvent::LeftButtonPress => {
                LEFT_S_ARROW.instant_display();
            }
            ButtonEvent::RightButtonPress => {
                RIGHT_S_ARROW.instant_display();
            }
            ButtonEvent::BothButtonsPress => {
                LEFT_S_ARROW.instant_display();
                RIGHT_S_ARROW.instant_display();
            }
            ButtonEvent::LeftButtonRelease => {
                LEFT_S_ARROW.erase();
                self.current = if self.current > 0 {
                    self.current - 1
                } else {
                    self.items.len() - 1
                };
                self.display();
            }
            ButtonEvent::RightButtonRelease => {
                RIGHT_S_ARROW.erase();
                self.current = if self.current < self.items.len() - 1 {
                    self.current + 1
                } else {
                    0
                };
                self.display();
            }
            ButtonEvent::BothButtonsRelease => {
                LEFT_S_ARROW.erase();
                RIGHT_S_ARROW.erase();
                (self.items[self.current].action)();
                self.display();
            }
        }
    }
}
