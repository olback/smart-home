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
