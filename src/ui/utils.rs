use crate::ui::multiline_scroller::{LINE1_Y, LINE2_Y, LINE3_Y};
use include_gif::include_gif;
use ledger_device_sdk::ui::bagls::Icon;
use ledger_device_sdk::ui::bitmaps::{Glyph, BACK};
use ledger_device_sdk::ui::layout::{Draw, Layout, Location, StringPlace};
use ledger_device_sdk::ui::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub trait CenteredText {
    fn draw_centered(&self, bold: bool);
}

impl CenteredText for &str {
    fn draw_centered(&self, bold: bool) {
        self.split('\n')
            .chain(core::iter::repeat(""))
            .take(3)
            .enumerate()
            .for_each(|(index, line)| {
                line.place(
                    match index {
                        0 => Location::Custom(LINE1_Y),
                        1 => Location::Custom(LINE2_Y),
                        2 => Location::Custom(LINE3_Y),
                        _ => unreachable!(),
                    },
                    Layout::Centered,
                    bold,
                )
            });
    }
}

pub trait TopCenter {
    fn draw_top_center(&self);
}

pub const DEFAULT_PADDING: usize = 11;
pub const DEFAULT_ICON_HEIGHT: usize = 14;

impl TopCenter for Icon<'_> {
    fn draw_top_center(&self) {
        Icon {
            icon: self.icon,
            pos: (
                SCREEN_WIDTH as i16 / 2 - self.icon.width as i16 / 2,
                SCREEN_HEIGHT as i16 / 2 - self.icon.height as i16,
            ),
        }
        .display();
    }
}

pub const RADIX_LOGO: Glyph = Glyph::from_include(include_gif!("icons/nanox_app_radix.gif"));
pub const RADIX_LOGO_ICON: Icon = Icon::from(&RADIX_LOGO);
pub const BACK_ICON: Icon = Icon::from(&BACK);
