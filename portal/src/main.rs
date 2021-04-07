use {
    delay::Delay,
    embedded_graphics::prelude::*,
    epd_waveshare::{
        epd7in5_v2::{Display7in5, EPD7in5},
        prelude::*,
    },
    rppal::{gpio::Gpio, spi, system::DeviceInfo},
};

mod delay;
mod screen;
mod ui;

fn main() {
    println!("Hello!");
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

    let mut epd7in5v2 =
        EPD7in5::new(&mut spi, cs, busy, dc, rst, &mut Delay).expect("Failed to init display");

    let mut display = Display7in5::default();
    display.set_rotation(epd_waveshare::graphics::DisplayRotation::Rotate180);

    // screen::Screen::Full {
    //     time: "01:56",
    //     date: "Wed 7 Apr",
    //     outdoor: (-1.2, 90.3),
    //     indoor: (21.8, 40.0),
    // }
    // .render(&mut display);

    screen::Screen::Clock("12:34").render(&mut display);
    // screen::Screen::Blank.render(&mut display);

    epd7in5v2
        .update_frame(&mut spi, display.buffer(), &mut Delay)
        .expect("Failed to update/display frame");

    epd7in5v2
        .display_frame(&mut spi, &mut Delay)
        .expect("Failed to display frame");

    epd7in5v2
        .sleep(&mut spi, &mut Delay)
        .expect("Failed to put display in sleep");

    std::thread::sleep(std::time::Duration::from_secs(5));

    println!("done");
}
