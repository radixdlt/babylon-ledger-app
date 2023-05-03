use core::cmp::min;
use nanos_sdk::buttons::{ButtonEvent, ButtonsState};
use nanos_ui::bagls::{Label, LEFT_ARROW, LEFT_S_ARROW, RIGHT_ARROW, RIGHT_S_ARROW};
use nanos_ui::layout::{Draw, Location};
use nanos_ui::ui::{clear_screen, get_event};
use nanos_ui::SCREEN_HEIGHT;

/// A horizontal scroller that
/// splits any given message
/// over several panes in chunks
/// of CHARS_PER_LINE*LINES_N characters.
/// Press both buttons to exit.
pub struct MultilineMessageScroller<'a> {
    message: &'a str,
}

const LINES_N: usize = 3;
const CHARS_PER_LINE: usize = 16;
const CHAR_N: usize = CHARS_PER_LINE * LINES_N;

const DEFAULT_FONT_HEIGHT: usize = 11;

// Nano S with smallest screen resolution
#[cfg(target_os = "nanos")]
const LINE1_Y: usize = 0;
#[cfg(target_os = "nanos")]
const LINE2_Y: usize = (SCREEN_HEIGHT - DEFAULT_FONT_HEIGHT) / 2;
#[cfg(target_os = "nanos")]
const LINE3_Y: usize = SCREEN_HEIGHT - DEFAULT_FONT_HEIGHT;

// Remaining devices
#[cfg(not(target_os = "nanos"))]
const VERTICAL_SPACING: usize = DEFAULT_FONT_HEIGHT + 1;
#[cfg(not(target_os = "nanos"))]
const LINE1_Y: usize = LINE2_Y - VERTICAL_SPACING;
#[cfg(not(target_os = "nanos"))]
const LINE2_Y: usize = (SCREEN_HEIGHT - VERTICAL_SPACING) / 2;
#[cfg(not(target_os = "nanos"))]
const LINE3_Y: usize = LINE2_Y + VERTICAL_SPACING;

impl<'a> MultilineMessageScroller<'a> {
    pub fn new(message: &'a str) -> Self {
        MultilineMessageScroller { message }
    }

    pub fn event_loop(&self) {
        clear_screen();
        let mut buttons = ButtonsState::new();
        let page_count = (self.message.len() - 1) / CHAR_N + 1;

        if page_count == 0 {
            return;
        }

        let mut labels: [Label; LINES_N] = [
            Label::from("").location(Location::Custom(LINE1_Y)),
            Label::from("").location(Location::Custom(LINE2_Y)),
            Label::from("").location(Location::Custom(LINE3_Y)),
        ];
        let mut cur_page = 0;

        // A closure to draw common elements of the screen
        // cur_page passed as parameter to prevent borrowing
        let mut draw = |page: usize| {
            let start = page * CHAR_N;
            let end = (start + CHAR_N).min(self.message.len());
            let chunk = &self.message[start..end];

            for label in labels.iter() {
                label.erase();
            }

            let mut from = 0;

            for label in labels.iter_mut() {
                if from >= chunk.len() {
                    label.text = "";
                } else {
                    let to = from + min(CHARS_PER_LINE, chunk.len() - from);
                    label.text = &chunk[from..to];
                }

                from += CHARS_PER_LINE;
            }

            if page > 0 {
                LEFT_ARROW.display();
            } else {
                LEFT_ARROW.erase();
            }

            if page + 1 < page_count {
                RIGHT_ARROW.display();
            } else {
                RIGHT_ARROW.erase();
            }

            for label in labels.iter() {
                label.instant_display();
            }
        };

        draw(cur_page);

        loop {
            match get_event(&mut buttons) {
                Some(ButtonEvent::LeftButtonPress) => {
                    LEFT_S_ARROW.instant_display();
                }
                Some(ButtonEvent::RightButtonPress) => {
                    RIGHT_S_ARROW.instant_display();
                }
                Some(ButtonEvent::LeftButtonRelease) => {
                    LEFT_S_ARROW.erase();
                    cur_page = cur_page.saturating_sub(1);
                    // We need to draw anyway to clear button press arrow
                    draw(cur_page);
                }
                Some(ButtonEvent::RightButtonRelease) => {
                    RIGHT_S_ARROW.erase();
                    if cur_page + 1 < page_count {
                        cur_page += 1;
                    }
                    // We need to draw anyway to clear button press arrow
                    draw(cur_page);
                }
                Some(ButtonEvent::BothButtonsRelease) => break,
                Some(_) | None => (),
            }
        }
    }
}
