#[macro_export]
macro_rules! delay {
    () => {
        unsafe { crate::DELAY.as_mut().unwrap() }
    };
}

#[macro_export]
macro_rules! delay_ms {
    ($t:expr) => {
        delay!().delay_ms($t)
    };
}

#[macro_export]
macro_rules! delay_us {
    ($t:expr) => {
        delay!().delay_us($t)
    };
}

pub fn round(n: f32) -> f32 {
    n as i32 as f32
}
