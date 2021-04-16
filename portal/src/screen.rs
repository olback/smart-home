use {
    crate::ui::{self, XPos, YPos},
    embedded_graphics::{pixelcolor::BinaryColor, prelude::*},
};

pub enum Screen<'a> {
    Blank,
    Clock(&'a str),
    Full {
        time: &'a str,
        date: &'a str,
        indoor: (f32, f32),
        outdoor: (f32, f32),
    },
}

impl<'a> Screen<'a> {
    pub fn render<D>(&self, display: &mut D)
    where
        D: DrawTarget<BinaryColor>,
        D::Error: std::fmt::Debug,
    {
        display
            .clear(epd_waveshare::color::White.invert())
            .expect("Failed to clear display");

        match self {
            Self::Blank => {}
            Self::Clock(time) => {
                ui::draw_text_fontdue(
                    display,
                    &[(*time, &ui::FONT_SOURCE_CODE_PRO_SEMI_BOLD)],
                    240.0,
                    ui::Black,
                    XPos::Center(0),
                    YPos::Center(0),
                );
            }
            Self::Full {
                time,
                date,
                indoor,
                outdoor,
            } => {
                // TODO: Reimplement with the new coordinate system
                // let mut xoff = 0;
                // xoff += ui::draw_text_right(
                //     display,
                //     profont::ProFont24Point,
                //     &format!("{} {}", date, time),
                //     ui::Black,
                //     ui::White,
                //     0,
                //     0,
                // )
                // .width
                //     + 20;

                // xoff += ui::draw_glyph(
                //     display,
                //     &ui::FONT_AWESOME_REGULAR,
                //     '\u{f1bb}',
                //     30.0,
                //     ui::Black,
                //     (800 - xoff - 27) as i32,
                //     0,
                // )
                // .width as u32
                //     + 1;

                // xoff += ui::draw_text_right(
                //     display,
                //     profont::ProFont24Point,
                //     &format!(
                //         "{}°/{}%",
                //         outdoor.0.round() as i32,
                //         outdoor.1.round() as u32
                //     ),
                //     ui::Black,
                //     ui::White,
                //     xoff as i32,
                //     0,
                // )
                // .width
                //     + 30;

                // xoff += ui::draw_glyph(
                //     display,
                //     &ui::FONT_AWESOME_REGULAR,
                //     '\u{f015}',
                //     30.0,
                //     ui::Black,
                //     (800 - xoff - 27) as i32,
                //     0,
                // )
                // .width as u32
                //     + 1;

                // xoff += ui::draw_text_right(
                //     display,
                //     profont::ProFont24Point,
                //     &format!("{}°/{}%", indoor.0.round() as i32, indoor.1.round() as u32),
                //     ui::Black,
                //     ui::White,
                //     xoff as i32,
                //     0,
                // )
                // .width
                //     + 10;
            }
        }
    }
}
