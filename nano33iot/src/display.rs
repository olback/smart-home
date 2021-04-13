use {
    arduino_nano33iot::{
        self as hal,
        clock::GenericClockController,
        gpio::{Floating, Input, Pb8, Pb9, PfD, Port},
        i2c_master,
        sercom::{I2CMaster4, Sercom4Pad0, Sercom4Pad1},
        time::Hertz,
    },
    embedded_graphics::{fonts::Text, pixelcolor::BinaryColor, prelude::*, text_style},
    ssd1306::prelude::*,
};

pub enum XPos {
    Absolute(i32),
    Left(i32),
    Center(i32),
    Right(i32),
}

pub enum YPos {
    Absolute(i32),
    Top(i32),
    Center(i32),
    Bottom(i32),
}

impl XPos {
    pub fn to_absolute(&self, display_width: u32) -> i32 {
        match self {
            Self::Absolute(n) => *n,
            Self::Left(n) => *n,
            Self::Center(n) => (display_width as i32 / 2) + n,
            Self::Right(n) => display_width as i32 - n,
        }
    }

    pub fn to_absolute_with_width(&self, display_width: u32, width: u32) -> i32 {
        match self {
            Self::Absolute(n) => *n,
            Self::Left(n) => *n,
            Self::Center(n) => (display_width as i32 / 2) - (width as i32 / 2) + n,
            Self::Right(n) => display_width as i32 - width as i32 - *n,
        }
    }
}

impl YPos {
    pub fn to_absolute(&self, display_height: u32) -> i32 {
        match self {
            Self::Absolute(n) => *n,
            Self::Top(n) => *n,
            Self::Center(n) => (display_height as i32 / 2) + n,
            Self::Bottom(n) => display_height as i32 - n,
        }
    }

    pub fn to_absolute_with_height(&self, display_height: u32, height: u32) -> i32 {
        match self {
            Self::Absolute(n) => *n,
            Self::Top(n) => *n,
            Self::Center(n) => (display_height as i32 / 2) - (height as i32 / 2) + n,
            Self::Bottom(n) => display_height as i32 - height as i32 - n,
        }
    }
}

pub struct Display {
    display: GraphicsMode<
        I2CInterface<I2CMaster4<Sercom4Pad0<Pb8<PfD>>, Sercom4Pad1<Pb9<PfD>>>>,
        DisplaySize128x64,
    >,
}

impl Display {
    pub fn new<F: Into<Hertz>>(
        clocks: &mut GenericClockController,
        bus_speed: F,
        sercom4: hal::pac::SERCOM4,
        pm: &mut hal::pac::PM,
        sda: Pb8<Input<Floating>>,
        scl: Pb9<Input<Floating>>,
        port: &mut Port,
    ) -> Self {
        let i2c = i2c_master(clocks, bus_speed, sercom4, pm, sda, scl, port);
        let interface = ssd1306::I2CDIBuilder::new().init(i2c);
        let display: GraphicsMode<_, _> = ssd1306::Builder::new().connect(interface).into();
        let mut s = Self { display };
        s.display.init().unwrap();
        s
    }

    pub fn clear(&mut self, flush: bool) {
        self.display.clear();
        if flush {
            let _ = self.display.flush();
        }
    }

    pub fn write<F: Font + Clone + Copy>(
        &mut self,
        text: &str,
        font: F,
        x: XPos,
        y: YPos,
        flush: bool,
    ) {
        let rt = Text::new(text, Point::new(0, 0)).into_styled(text_style!(
            font = font,
            text_color = BinaryColor::On,
            background_color = BinaryColor::Off
        ));
        let rt_size = rt.size();
        let (dw, dh) = self.display.get_dimensions();
        let (lx, ly) = (
            x.to_absolute_with_width(dw as u32, rt_size.width),
            y.to_absolute_with_height(dh as u32, rt_size.height),
        );
        let _ = rt.translate(Point::new(lx, ly)).draw(&mut self.display);
        if flush {
            let _ = self.display.flush();
        }
    }
}
