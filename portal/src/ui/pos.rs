#[derive(Debug)]
pub enum XPos {
    Absolute(i32),
    Left(i32),
    Center(i32),
    Right(i32),
}

#[derive(Debug)]
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
