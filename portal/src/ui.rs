use embedded_graphics::{
    fonts::Text,
    geometry::Size,
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, Line, Rectangle},
    style::{PrimitiveStyle, PrimitiveStyleBuilder},
    text_style,
};

macro_rules! draw_ret_size {
    ($d:expr, $v:expr) => {{
        let a = $v;
        let s = a.size();
        let _ = a.draw($d);
        s
    }};
}

const FONT_SETTINGS: fontdue::FontSettings = fontdue::FontSettings {
    enable_offset_bounding_box: true,
    collection_index: 0,
    scale: 30.0,
};
const FONT_AWESOME_REGULAR_BYTES: &'static [u8] =
    include_bytes!("/home/olback/Downloads/Font Awesome 5 Pro-Regular-400.otf");
const FONT_AWESOME_SOLID_BYTES: &'static [u8] =
    include_bytes!("/home/olback/Downloads/Font Awesome 5 Pro-Solid-900.otf");
const FONT_AWESOME_BRANDS_BYTES: &'static [u8] =
    include_bytes!("/home/olback/Downloads/Font Awesome 5 Brands-Regular-400.otf");

lazy_static::lazy_static! {
    pub static ref FONT_AWESOME_REGULAR: fontdue::Font =
        fontdue::Font::from_bytes(FONT_AWESOME_REGULAR_BYTES, FONT_SETTINGS).expect("Failed to parse font");
    pub static ref FONT_AWESOME_SOLID: fontdue::Font =
        fontdue::Font::from_bytes(FONT_AWESOME_SOLID_BYTES, FONT_SETTINGS).expect("Failed to parse font");
    pub static ref FONT_AWESOME_BRANDS: fontdue::Font =
        fontdue::Font::from_bytes(FONT_AWESOME_BRANDS_BYTES, FONT_SETTINGS).expect("Failed to parse font");
}

pub use embedded_graphics::pixelcolor::BinaryColor::{Off as Black, On as White};

pub fn draw_line<D: DrawTarget<BinaryColor>>(
    display: &mut D,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    stroke_width: u32,
) -> Size {
    draw_ret_size!(
        display,
        Line::new(Point::new(x1, y1), Point::new(x2, y2))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::Off, stroke_width))
    )
}

pub fn draw_suqare<D: DrawTarget<BinaryColor>>(
    display: &mut D,
    w: i32,
    h: i32,
    stroke_color: BinaryColor,
    stroke_width: u32,
    fill_color: BinaryColor,
    x: i32,
    y: i32,
) -> Size {
    let style = PrimitiveStyleBuilder::new()
        .stroke_color(stroke_color)
        .stroke_width(stroke_width)
        .fill_color(fill_color)
        .build();
    draw_ret_size!(
        display,
        Rectangle::new(Point::new(x, y), Point::new(x + w, y + h)).into_styled(style)
    )
}

pub fn draw_circle<D: DrawTarget<BinaryColor>>(
    display: &mut D,
    radius: u32,
    stroke_color: BinaryColor,
    stroke_width: u32,
    fill_color: BinaryColor,
    x: i32,
    y: i32,
) -> Size {
    let style = PrimitiveStyleBuilder::new()
        .stroke_color(stroke_color)
        .stroke_width(stroke_width)
        .fill_color(fill_color)
        .build();
    draw_ret_size!(
        display,
        Circle::new(Point::new(x, y), radius).into_styled(style)
    )
}

pub fn draw_text<D: DrawTarget<BinaryColor>, F: Font + Clone + Copy>(
    display: &mut D,
    font: F,
    text: &str,
    color: BinaryColor,
    background: BinaryColor,
    x: i32,
    y: i32,
) -> Size {
    draw_ret_size!(
        display,
        Text::new(text, Point::new(x, y)).into_styled(text_style!(
            font = font,
            text_color = color,
            background_color = background
        ))
    )
}

pub fn draw_text_fontdue<D: DrawTarget<BinaryColor>>(
    display: &mut D,
    text: &[(&str, &fontdue::Font)],
    size: f32,
    color: BinaryColor,
    x: i32,
    y: i32,
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

    let mut lsize = Size::new(0, layout.height() as u32);
    for g in layout.glyphs() {
        lsize.width += draw_glyph(
            display,
            text[g.key.font_index].1,
            g.key.c,
            size,
            color,
            x + g.x as i32,
            y + g.y as i32,
        )
        .advance_width as u32;
    }
    lsize
}

pub fn draw_text_right<D: DrawTarget<BinaryColor>, F: Font + Clone + Copy>(
    display: &mut D,
    font: F,
    text: &str,
    color: BinaryColor,
    background: BinaryColor,
    x: i32,
    y: i32,
) -> Size {
    let scrap = Text::new(text, Point::new(x, y)).into_styled(text_style!(
        font = font,
        text_color = color,
        background_color = background
    ));
    let new_x = display.size().width - (x as u32) - scrap.size().width;
    draw_text(display, font, text, color, background, new_x as i32, y)
}

pub fn draw_pixel<D: DrawTarget<BinaryColor>>(
    display: &mut D,
    color: BinaryColor,
    x: i32,
    y: i32,
) -> Size {
    let _ = Pixel(Point::new(x, y), color).draw(display);
    Size::new(1, 1)
}

pub fn draw_glyph<D: DrawTarget<BinaryColor>>(
    display: &mut D,
    font: &fontdue::Font,
    glyph: char,
    size: f32,
    color: BinaryColor,
    x: i32,
    y: i32,
) -> fontdue::Metrics {
    let (metrics, bitmap) = font.rasterize(glyph, size);
    for i in 0..metrics.width {
        for j in 0..metrics.height {
            if bitmap[(j * metrics.width) + i] > 127 {
                draw_pixel(display, color, i as i32 + x, j as i32 + y);
            }
        }
    }
    metrics
}
