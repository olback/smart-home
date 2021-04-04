macro_rules! impl_delay_us {
    ($t:ty) => {
        impl embedded_hal::blocking::delay::DelayUs<$t> for Delay {
            fn delay_us(&mut self, us: $t) {
                std::thread::sleep(std::time::Duration::from_micros(us as u64))
            }
        }
        impl embedded_hal::blocking::delay::DelayUs<$t> for &mut Delay {
            fn delay_us(&mut self, us: $t) {
                std::thread::sleep(std::time::Duration::from_micros(us as u64))
            }
        }
    };
}

macro_rules! impl_delay_ms {
    ($t:ty) => {
        impl embedded_hal::blocking::delay::DelayMs<$t> for Delay {
            fn delay_ms(&mut self, ms: $t) {
                std::thread::sleep(std::time::Duration::from_millis(ms as u64))
            }
        }
        impl embedded_hal::blocking::delay::DelayMs<$t> for &mut Delay {
            fn delay_ms(&mut self, ms: $t) {
                std::thread::sleep(std::time::Duration::from_millis(ms as u64))
            }
        }
    };
}

pub struct Delay;

impl_delay_us!(u8);
impl_delay_us!(u16);
impl_delay_us!(u32);
impl_delay_us!(u64);
impl_delay_us!(usize);

impl_delay_ms!(u8);
impl_delay_ms!(u16);
impl_delay_ms!(u32);
impl_delay_ms!(u64);
impl_delay_ms!(usize);
