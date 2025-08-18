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

pub const BACK: Glyph = Glyph::from_include(include_gif!("icons/icon_back.gif"));
pub const BACK_ICON: Icon = Icon::from(&BACK);

pub const LEFT_ARROW: Glyph = Glyph::from_include(include_gif!("icons/icon_left.gif"));
pub const LEFT_ARROW_ICON: Icon = Icon::from(&LEFT_ARROW).set_x(OUTER_PADDING as i16);
pub const LEFT_S_ARROW_ICON: Icon = LEFT_ARROW_ICON.shift_h(4);

pub const RIGHT_ARROW: Glyph = Glyph::from_include(include_gif!("icons/icon_right.gif"));
pub const RIGHT_ARROW_ICON: Icon =
    Icon::from(&RIGHT_ARROW).set_x(SCREENW - RIGHT_ARROW.width as i16);
pub const RIGHT_S_ARROW_ICON: Icon = RIGHT_ARROW_ICON.shift_h(-4);

pub const CROSSMARK: Glyph = Glyph::from_include(include_gif!("icons/icon_crossmark.gif"));
pub const CROSSMARK_ICON: Icon = Icon::from(&CROSSMARK);

pub const VALIDATE_14: Glyph = Glyph::from_include(include_gif!("icons/icon_validate_14.gif"));
pub const VALIDATE_14_ICON: Icon = Icon::from(&VALIDATE_14);

pub const WARNING: Glyph = Glyph::from_include(include_gif!("icons/icon_warning.gif"));
pub const WARNING_ICON: Icon = Icon::from(&WARNING);

pub const PROCESSING: Glyph = Glyph::from_include(include_gif!("icons/icon_processing.gif"));
pub const PROCESSING_ICON: Icon = Icon::from(&PROCESSING);

pub const CERTIFICATE: Glyph = Glyph::from_include(include_gif!("icons/icon_certificate.gif"));
pub const CERTIFICATE_ICON: Icon = Icon::from(&CERTIFICATE);

pub const COGGLE: Glyph = Glyph::from_include(include_gif!("icons/icon_coggle.gif"));
pub const COGGLE_ICON: Icon = Icon::from(&COGGLE);

pub const DASHBOARD_X: Glyph = Glyph::from_include(include_gif!("icons/icon_dashboard_x.gif"));
pub const DASHBOARD_X_ICON: Icon = Icon::from(&DASHBOARD_X);
