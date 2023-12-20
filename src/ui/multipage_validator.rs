use ledger_device_sdk::buttons::{ButtonEvent, ButtonsState};
use ledger_device_sdk::ui::bagls::{
    Icon, CROSSMARK_ICON, LEFT_ARROW, LEFT_S_ARROW, RIGHT_ARROW, RIGHT_S_ARROW, VALIDATE_14_ICON,
    WARNING_ICON,
};
use ledger_device_sdk::ui::gadgets::{clear_screen, get_event};
use ledger_device_sdk::ui::layout::{Draw, Layout, Location, StringPlace};
use ledger_device_sdk::ui::screen_util;

use crate::io::UxEvent;

pub struct MultipageValidator<'a> {
    message: &'a [&'a str],
    confirm: &'a [&'a str],
    cancel: &'a [&'a str],
}

const HALF_ICON_WIDTH: usize = 7;

impl<'a> MultipageValidator<'a> {
    pub const fn new(
        message: &'a [&'a str],
        confirm: &'a [&'a str],
        cancel: &'a [&'a str],
    ) -> Self {
        MultipageValidator {
            message,
            confirm,
            cancel,
        }
    }

    pub fn ask(&self) -> bool {
        clear_screen();
        let page_count = 3;
        let mut cur_page = 0;

        let draw_icon_and_text = |icon: Icon, strings: &[&str], bold: bool| {
            // Draw icon on the center if there is no text.
            let x = match strings.len() {
                0 => 60,
                _ => 18,
            };
            icon.set_x(x).display();

            match strings.len() {
                0 => {}
                1 => {
                    strings[0].place(Location::Middle, Layout::Centered, bold);
                }
                _ => {
                    strings[..2].place(Location::Middle, Layout::Centered, bold);
                }
            }
        };

        let draw = |page: usize| {
            clear_screen();
            if page == page_count - 2 {
                draw_icon_and_text(VALIDATE_14_ICON, self.confirm, true);
                RIGHT_ARROW.display();
            } else if page == page_count - 1 {
                draw_icon_and_text(CROSSMARK_ICON, self.cancel, true);
            } else {
                draw_icon_and_text(WARNING_ICON, self.message, false);
                RIGHT_ARROW.display();
            }
            if page > 0 {
                LEFT_ARROW.display();
            }
            screen_util::screen_update();
        };

        draw(cur_page);

        let mut buttons = ButtonsState::new();
        loop {
            let event = get_event(&mut buttons);

            if let Some(_) = event {
                UxEvent::wakeup();
            }

            match event {
                Some(ButtonEvent::LeftButtonPress) => {
                    LEFT_S_ARROW.instant_display();
                }
                Some(ButtonEvent::RightButtonPress) => {
                    RIGHT_S_ARROW.instant_display();
                }
                Some(ButtonEvent::LeftButtonRelease) => {
                    LEFT_S_ARROW.erase();
                    cur_page = cur_page.saturating_sub(1);
                    draw(cur_page);
                }
                Some(ButtonEvent::RightButtonRelease) => {
                    RIGHT_S_ARROW.erase();
                    if cur_page < page_count - 1 {
                        cur_page += 1;
                    }
                    draw(cur_page);
                }
                Some(ButtonEvent::BothButtonsRelease) => {
                    if cur_page == page_count - 2 {
                        // Confirm
                        return true;
                    } else if cur_page == page_count - 1 {
                        // Abort
                        return false;
                    }
                    draw(cur_page);
                }
                _ => (),
            }
        }
    }
}
