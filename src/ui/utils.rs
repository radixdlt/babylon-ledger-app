use nanos_ui::bagls::Icon;
use nanos_ui::bitmaps::Glyph;
use nanos_ui::layout::{Draw, Layout, Location, StringPlace};

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
                        0 => Location::Top,
                        1 => Location::Middle,
                        2 => Location::Bottom,
                        _ => unreachable!(),
                    },
                    Layout::Centered,
                    bold,
                )
            });
    }
}

pub trait LeftAlignedMiddle {
    fn draw_left_aligned_middle(&self);
}

pub const DEFAULT_PADDING: usize = 14;
pub const DEFAULT_ICON_HEIGHT: usize = 14;

impl LeftAlignedMiddle for Icon<'_> {
    fn draw_left_aligned_middle(&self) {
        let icon = Icon {
            icon: self.icon,
            pos: self.pos,
        };

        icon.set_x(DEFAULT_PADDING as i16)
            .set_y(Location::Middle.get_y(DEFAULT_ICON_HEIGHT) as i16)
            .display();
    }
}

use include_gif::include_gif;

pub const RADIX_LOGO: Glyph = Glyph::from_include(include_gif!("icons/nanox_app_radix.gif"));
pub const RADIX_LOGO_ICON: Icon = Icon::from(&RADIX_LOGO);
