use {
    delay::Delay,
    embedded_graphics::{
        fonts::{Font24x32, Text},
        prelude::*,
        text_style,
    },
    embedded_hal::blocking::delay::DelayMs,
    epd_waveshare::{
        epd7in5_v2::{Display7in5, EPD7in5},
        prelude::*,
    },
    rppal::{gpio::Gpio, spi, system::DeviceInfo},
};

mod delay;

fn main() {
    let dev_info = DeviceInfo::new().expect("Failed to obtain device info");
    println!("{:#?}", dev_info);

    let mut spi = spi::Spi::new(
        spi::Bus::Spi0,
        spi::SlaveSelect::Ss1,
        4_000_000,
        spi::Mode::Mode0,
    )
    .expect("Failed to get spi");
    let gpio = Gpio::new().expect("Failed to get gpio");
    let mut cs = gpio.get(8).expect("Failed to get cs pin").into_output();
    cs.set_low();
    let busy = gpio.get(24).expect("Failed to get busy pin").into_input();
    let dc = gpio.get(25).expect("Failed to get dc pin").into_output();
    let rst = gpio.get(17).expect("Failed to get rst pin").into_output();

    println!("1");

    let mut epd7in5v2 =
        EPD7in5::new(&mut spi, cs, busy, dc, rst, Delay).expect("Failed to init display");

    println!("2");

    let mut display = Display7in5::default();
    display.set_rotation(epd_waveshare::graphics::DisplayRotation::Rotate180);
    display.clear(epd_waveshare::color::White);
    draw_text(&mut display, "test", 100, 100);

    // epd7in5v2.set_background_color(epd_waveshare::color::Color::White);

    println!("3");

    epd7in5v2
        .update_frame(&mut spi, display.buffer())
        .expect("Failed to update/display frame");

    epd7in5v2.display_frame(&mut spi);

    println!("4");

    std::thread::sleep(std::time::Duration::from_secs(5));

    epd7in5v2
        .sleep(&mut spi)
        .expect("Failed to put display in sleep");

    println!("done");
}

fn draw_text(display: &mut Display7in5, text: &str, x: i32, y: i32) {
    let _ = Text::new(text, Point::new(x, y))
        .into_styled(text_style!(
            font = Font24x32,
            text_color = epd_waveshare::color::Black,
            background_color = epd_waveshare::color::White
        ))
        .draw(display);
}
