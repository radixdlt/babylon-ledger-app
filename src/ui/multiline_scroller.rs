use core::cmp::min;

use ledger_device_sdk::buttons::{ButtonEvent, ButtonsState};
use ledger_device_sdk::ui::bagls::Label;
use ledger_device_sdk::ui::gadgets::{clear_screen, get_event};
use ledger_device_sdk::ui::layout::{Draw, Location};
use ledger_device_sdk::ui::SCREEN_HEIGHT;

use crate::io::UxEvent;

pub struct MultilineMessageScroller<'a> {
    message: &'a str,
    title: Option<&'a str>,
    show_right_arrow: bool,
}

const LINES_N: usize = 3;
const CHARS_PER_LINE: usize = 16;

const DEFAULT_FONT_HEIGHT: usize = 11;

// Remaining devices
const VERTICAL_SPACING: usize = DEFAULT_FONT_HEIGHT + 1;
pub const LINE1_Y: usize = LINE2_Y - VERTICAL_SPACING;
pub const LINE2_Y: usize = (SCREEN_HEIGHT - VERTICAL_SPACING) / 2;
pub const LINE3_Y: usize = LINE2_Y + VERTICAL_SPACING;

impl<'a> MultilineMessageScroller<'a> {
    pub fn with_title(title: &'a str, message: &'a str, show_right_arrow: bool) -> Self {
        MultilineMessageScroller {
            message,
            title: Some(title),
            show_right_arrow,
        }
    }

    pub fn event_loop(&self) {
        clear_screen();
        let page_len = CHARS_PER_LINE
            * if self.title.is_some() {
                LINES_N - 1
            } else {
                LINES_N
            };
        let mut buttons = ButtonsState::new();
        let page_count = (self.message.len() - 1) / page_len + 1;

        if page_count == 0 {
            return;
        }

        let mut labels: [Label; LINES_N] = [
            Label::from_const("").location(Location::Custom(LINE1_Y)),
            Label::from_const("").location(Location::Custom(LINE2_Y)),
            Label::from_const("").location(Location::Custom(LINE3_Y)),
        ];
        let mut cur_page = 0;

        let mut draw = |page: usize| {
            let start = page * page_len;
            let end = (start + page_len).min(self.message.len());
            let chunk = &self.message[start..end];
            let start_line = if self.title.is_some() { 1 } else { 0 };

            for label in labels.iter() {
                label.erase();
            }

            if let Some(title) = self.title {
                labels[0].text = title;
            }

            let mut from = 0;

            for label in labels.iter_mut().take(LINES_N).skip(start_line) {
                if from >= chunk.len() {
                    label.text = "";
                } else {
                    let to = from + min(CHARS_PER_LINE, chunk.len() - from);
                    label.text = &chunk[from..to];
                }

                from += CHARS_PER_LINE;
            }

            if page > 0 {
                labels[0].bold = self.title.is_some();
                crate::ui::utils::LEFT_ARROW_ICON.display();
            } else {
                labels[0].bold = true;
                crate::ui::utils::LEFT_ARROW_ICON.erase();
            }

            if page + 1 < page_count || self.show_right_arrow {
                crate::ui::utils::RIGHT_ARROW_ICON.display();

                if self.show_right_arrow && page + 1 == page_count {
                    crate::ui::utils::RIGHT_S_ARROW_ICON.display();
                }
            } else {
                crate::ui::utils::RIGHT_ARROW_ICON.erase();

                if page + 1 != page_count {
                    crate::ui::utils::RIGHT_S_ARROW_ICON.erase();
                }
            }

            for label in labels.iter() {
                label.instant_display();
            }
        };

        draw(cur_page);

        loop {
            let event = get_event(&mut buttons);

            if event.is_some() {
                UxEvent::wakeup();
            }

            match event {
                Some(ButtonEvent::LeftButtonPress) => {
                    crate::ui::utils::LEFT_S_ARROW_ICON.instant_display();
                }
                Some(ButtonEvent::RightButtonPress) => {
                    crate::ui::utils::RIGHT_S_ARROW_ICON.instant_display();
                }
                Some(ButtonEvent::LeftButtonRelease) => {
                    crate::ui::utils::LEFT_S_ARROW_ICON.erase();
                    cur_page = cur_page.saturating_sub(1);

                    draw(cur_page);
                }
                Some(ButtonEvent::RightButtonRelease) => {
                    crate::ui::utils::RIGHT_S_ARROW_ICON.erase();
                    if cur_page + 1 < page_count {
                        cur_page += 1;
                    } else {
                        break;
                    }

                    draw(cur_page);
                }
                Some(ButtonEvent::BothButtonsRelease) => break,
                Some(_) | None => (),
            }
        }
    }
}
