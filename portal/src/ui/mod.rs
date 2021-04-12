use embedded_graphics::{
    fonts::Text,
    geometry::Size,
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, Line, Rectangle},
    style::{PrimitiveStyle, PrimitiveStyleBuilder},
    text_style,
};

mod pos;

pub use {
    embedded_graphics::pixelcolor::BinaryColor::{Off as Black, On as White},
    pos::*,
};

const FONT_SETTINGS: fontdue::FontSettings = fontdue::FontSettings {
    enable_offset_bounding_box: true,
    collection_index: 0,
    scale: 30.0,
};
const FONT_AWESOME_LIGHT_BYTES: &'static [u8] = include_bytes!("../../fonts/fa5-light.otf");
const FONT_AWESOME_REGULAR_BYTES: &'static [u8] = include_bytes!("../../fonts/fa5-regular.otf");
const FONT_AWESOME_SOLID_BYTES: &'static [u8] = include_bytes!("../../fonts/fa5-solid.otf");
const FONT_AWESOME_BRANDS_BYTES: &'static [u8] = include_bytes!("../../fonts/fa5-brands.otf");
const FONT_PIXELBOY_BYTES: &'static [u8] = include_bytes!("../../fonts/Pixeboy-z8XGD.ttf");
const FONT_SOURCE_CODE_PRO_SEMI_BOLD_BYTES: &'static [u8] =
    include_bytes!("../../fonts/SourceCodePro-SemiBold.ttf");

lazy_static::lazy_static! {
    pub static ref FONT_AWESOME_LIGHT: fontdue::Font =
        fontdue::Font::from_bytes(FONT_AWESOME_LIGHT_BYTES, FONT_SETTINGS).expect("Failed to parse font");
    pub static ref FONT_AWESOME_REGULAR: fontdue::Font =
        fontdue::Font::from_bytes(FONT_AWESOME_REGULAR_BYTES, FONT_SETTINGS).expect("Failed to parse font");
    pub static ref FONT_AWESOME_SOLID: fontdue::Font =
        fontdue::Font::from_bytes(FONT_AWESOME_SOLID_BYTES, FONT_SETTINGS).expect("Failed to parse font");
    pub static ref FONT_AWESOME_BRANDS: fontdue::Font =
        fontdue::Font::from_bytes(FONT_AWESOME_BRANDS_BYTES, FONT_SETTINGS).expect("Failed to parse font");
    pub static ref FONT_PIXELBOY: fontdue::Font =
        fontdue::Font::from_bytes(FONT_PIXELBOY_BYTES, FONT_SETTINGS).expect("Failed to parse font");
    pub static ref FONT_SOURCE_CODE_PRO_SEMI_BOLD: fontdue::Font =
        fontdue::Font::from_bytes(FONT_SOURCE_CODE_PRO_SEMI_BOLD_BYTES, FONT_SETTINGS).expect("Failed to parse font");
}

pub fn draw_line<D: DrawTarget<BinaryColor>>(
    display: &mut D,
    x1: XPos,
    y1: YPos,
    x2: XPos,
    y2: YPos,
    color: BinaryColor,
    stroke_width: u32,
) -> Size {
    let display_size = display.size();
    let p1 = Point::new(
        x1.to_absolute(display_size.width),
        y1.to_absolute(display_size.height),
    );
    let p2 = Point::new(
        x2.to_absolute(display_size.width),
        y2.to_absolute(display_size.height),
    );
    let target = Line::new(p1, p2).into_styled(PrimitiveStyle::with_stroke(color, stroke_width));
    let size = target.size();
    let _ = target.draw(display);
    size
}

pub fn draw_suqare<D: DrawTarget<BinaryColor>>(
    display: &mut D,
    w: i32,
    h: i32,
    stroke_color: BinaryColor,
    stroke_width: u32,
    fill_color: BinaryColor,
    x: XPos,
    y: YPos,
) -> Size {
    let style = PrimitiveStyleBuilder::new()
        .stroke_color(stroke_color)
        .stroke_width(stroke_width)
        .fill_color(fill_color)
        .build();
    let target_size = Rectangle::new(Point::new(0, 0), Point::new(w, h))
        .into_styled(style)
        .size();
    let display_size = display.size();
    let (lx, ly) = (
        x.to_absolute_with_width(display_size.width, target_size.width),
        y.to_absolute_with_height(display_size.height, target_size.height),
    );
    let target = Rectangle::new(Point::new(lx, ly), Point::new(lx + w, ly + h)).into_styled(style);
    let size = target.size();
    let _ = target.draw(display);
    size
}

pub fn draw_circle<D: DrawTarget<BinaryColor>>(
    display: &mut D,
    radius: u32,
    stroke_color: BinaryColor,
    stroke_width: u32,
    fill_color: BinaryColor,
    x: XPos,
    y: YPos,
) -> Size {
    let style = PrimitiveStyleBuilder::new()
        .stroke_color(stroke_color)
        .stroke_width(stroke_width)
        .fill_color(fill_color)
        .build();
    let display_size = display.size();
    let (lx, ly) = (
        x.to_absolute(display_size.width),
        y.to_absolute(display_size.height),
    );
    let target = Circle::new(Point::new(lx, ly), radius).into_styled(style);
    let size = target.size();
    let _ = target.draw(display);
    size
}

pub fn draw_text<D: DrawTarget<BinaryColor>, F: Font + Clone + Copy>(
    display: &mut D,
    font: F,
    text: &str,
    color: BinaryColor,
    background: BinaryColor,
    x: XPos,
    y: YPos,
) -> Size {
    let target_size = Text::new(text, Point::new(0, 0))
        .into_styled(text_style!(
            font = font,
            text_color = color,
            background_color = background
        ))
        .size();
    let display_size = display.size();
    let (lx, ly) = (
        x.to_absolute_with_width(display_size.width, target_size.width),
        y.to_absolute_with_height(display_size.height, target_size.height),
    );
    let target = Text::new(text, Point::new(lx, ly)).into_styled(text_style!(
        font = font,
        text_color = color,
        background_color = background
    ));
    let size = target.size();
    let _ = target.draw(display);
    size
}

pub fn draw_text_fontdue<D: DrawTarget<BinaryColor>>(
    display: &mut D,
    text: &[(&str, &fontdue::Font)],
    size: f32,
    color: BinaryColor,
    x: XPos,
    y: YPos,
) -> Size {
    let mut layout = fontdue::layout::Layout::new(fontdue::layout::CoordinateSystem::PositiveYDown);
    for i in 0..text.len() {
        layout.append(
            text.iter()
                .map(|v| v.1)
                .collect::<Vec<&fontdue::Font>>()
                .as_ref(),
            &fontdue::layout::TextStyle::with_user_data(text[i].0, size, i, ()),
        );
    }

    let display_size = display.size();
    let mut lsize = Size::new(0, layout.height() as u32);
    let glyphs = layout.glyphs();
    let rasterized_glyphs = glyphs
        .iter()
        .map(|g| text[g.key.font_index].1.rasterize(g.key.c, size))
        .collect::<Vec<_>>();
    lsize.width = rasterized_glyphs
        .iter()
        .fold(0, |acc, rg| acc + rg.0.advance_width as u32);

    for (n, (metrics, bitmap)) in rasterized_glyphs.iter().enumerate() {
        for i in 0..metrics.width {
            for j in 0..metrics.height {
                if bitmap[(j * metrics.width) + i] > 127 {
                    draw_pixel(
                        display,
                        color,
                        XPos::Absolute(
                            glyphs[n].x as i32
                                + i as i32
                                + x.to_absolute_with_width(display_size.width, lsize.width),
                        ),
                        YPos::Absolute(
                            glyphs[n].y as i32
                                + j as i32
                                + y.to_absolute_with_height(display_size.height, lsize.height),
                        ),
                    );
                }
            }
        }
    }

    lsize
}

pub fn draw_pixel<D: DrawTarget<BinaryColor>>(
    display: &mut D,
    color: BinaryColor,
    x: XPos,
    y: YPos,
) -> Size {
    let display_size = display.size();
    let (lx, ly) = (
        x.to_absolute_with_width(display_size.width, 1),
        y.to_absolute_with_height(display_size.height, 1),
    );
    let _ = Pixel(Point::new(lx, ly), color).draw(display);
    Size::new(1, 1)
}

pub fn draw_glyph<D: DrawTarget<BinaryColor>>(
    display: &mut D,
    font: &fontdue::Font,
    glyph: char,
    size: f32,
    color: BinaryColor,
    x: XPos,
    y: YPos,
) -> fontdue::Metrics {
    let display_size = display.size();
    let (metrics, bitmap) = font.rasterize(glyph, size);
    for i in 0..metrics.width {
        for j in 0..metrics.height {
            if bitmap[(j * metrics.width) + i] > 127 {
                draw_pixel(
                    display,
                    color,
                    XPos::Absolute(
                        i as i32
                            + x.to_absolute_with_width(
                                display_size.width,
                                metrics.advance_width as u32,
                            )
                            - metrics.xmin,
                    ),
                    YPos::Absolute(
                        j as i32
                            + y.to_absolute_with_height(
                                display_size.height,
                                metrics.advance_height as u32,
                            )
                            - metrics.ymin,
                    ),
                );
            }
        }
    }
    metrics
}
