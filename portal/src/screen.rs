use {
    crate::ui,
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
                // Math does not make sense...
                const SQUARE_SIZE: i32 = 19;
                const XOFF: i32 = 25;
                const YOFF: i32 = 160;
                let my_vec = bitfont::bitmap_bool(time).unwrap();
                for (y, my_row) in my_vec.iter().enumerate() {
                    for (x, my_char) in my_row.iter().enumerate() {
                        if *my_char {
                            ui::draw_glyph(
                                display,
                                &ui::FONT_AWESOME_SOLID,
                                '\u{f45c}',
                                SQUARE_SIZE as f32,
                                ui::Black,
                                (x as i32 * SQUARE_SIZE) + XOFF,
                                (y as i32 * SQUARE_SIZE) + YOFF,
                            );
                        } else {
                            ui::draw_glyph(
                                display,
                                &ui::FONT_AWESOME_SOLID,
                                '\u{f45c}',
                                SQUARE_SIZE as f32,
                                ui::White,
                                (x as i32 * SQUARE_SIZE) + XOFF,
                                (y as i32 * SQUARE_SIZE) + YOFF,
                            );
                        }
                    }
                }
            }
            Self::Full {
                time,
                date,
                indoor,
                outdoor,
            } => {
                let mut xoff = 0;
                xoff += ui::draw_text_right(
                    display,
                    profont::ProFont24Point,
                    &format!("{} {}", date, time),
                    ui::Black,
                    ui::White,
                    0,
                    0,
                )
                .width
                    + 20;

                xoff += ui::draw_glyph(
                    display,
                    &ui::FONT_AWESOME_REGULAR,
                    '\u{f1bb}',
                    30.0,
                    ui::Black,
                    (800 - xoff - 27) as i32,
                    0,
                )
                .width as u32
                    + 1;

                xoff += ui::draw_text_right(
                    display,
                    profont::ProFont24Point,
                    &format!(
                        "{}°/{}%",
                        outdoor.0.round() as i32,
                        outdoor.1.round() as u32
                    ),
                    ui::Black,
                    ui::White,
                    xoff as i32,
                    0,
                )
                .width
                    + 30;

                xoff += ui::draw_glyph(
                    display,
                    &ui::FONT_AWESOME_REGULAR,
                    '\u{f015}',
                    30.0,
                    ui::Black,
                    (800 - xoff - 27) as i32,
                    0,
                )
                .width as u32
                    + 1;

                xoff += ui::draw_text_right(
                    display,
                    profont::ProFont24Point,
                    &format!("{}°/{}%", indoor.0.round() as i32, indoor.1.round() as u32),
                    ui::Black,
                    ui::White,
                    xoff as i32,
                    0,
                )
                .width
                    + 10;
            }
        }
    }
}
