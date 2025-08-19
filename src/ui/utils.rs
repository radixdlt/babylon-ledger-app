use crate::ui::multiline_scroller::{LINE1_Y, LINE2_Y, LINE3_Y};
use include_gif::include_gif;
use ledger_device_sdk::ui::bagls::Icon;
use ledger_device_sdk::ui::bitmaps::Glyph;
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

pub const OUTER_PADDING: usize = 2;
pub const SCREENW: i16 = (SCREEN_WIDTH - OUTER_PADDING) as i16;

pub const RADIX_LOGO: Glyph = Glyph::from_include(include_gif!("icons/nanox_app_radix.gif"));
pub const RADIX_LOGO_ICON: Icon = Icon::from(&RADIX_LOGO);


pub static BACK_ICON: Icon = Icon::from(&ledger_device_sdk::ui::bitmaps::BACK);
pub static LEFT_ARROW_ICON: Icon = Icon::from(&ledger_device_sdk::ui::bitmaps::LEFT_ARROW).set_x(OUTER_PADDING as i16);
pub static LEFT_S_ARROW_ICON: Icon = LEFT_ARROW_ICON.shift_h(4);
pub static RIGHT_ARROW_ICON: Icon =
    Icon::from(&ledger_device_sdk::ui::bitmaps::RIGHT_ARROW).set_x(SCREENW - ledger_device_sdk::ui::bitmaps::RIGHT_ARROW.width as i16);
pub static RIGHT_S_ARROW_ICON: Icon = RIGHT_ARROW_ICON.shift_h(-4);
pub static CROSSMARK_ICON: Icon = Icon::from(&ledger_device_sdk::ui::bitmaps::CROSSMARK);
pub static VALIDATE_14_ICON: Icon = Icon::from(&ledger_device_sdk::ui::bitmaps::VALIDATE_14);
pub static WARNING_ICON: Icon = Icon::from(&ledger_device_sdk::ui::bitmaps::WARNING);
pub static PROCESSING_ICON: Icon = Icon::from(&ledger_device_sdk::ui::bitmaps::PROCESSING);
pub static CERTIFICATE_ICON: Icon = Icon::from(&ledger_device_sdk::ui::bitmaps::CERTIFICATE);
pub static COGGLE_ICON: Icon = Icon::from(&ledger_device_sdk::ui::bitmaps::COGGLE);
pub static DASHBOARD_X_ICON: Icon = Icon::from(&ledger_device_sdk::ui::bitmaps::DASHBOARD_X);
