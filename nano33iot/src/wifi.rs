use arduino_nano33iot::{
    self as hal,
    clock::GenericClockController,
    gpio::{Floating, Input, Pa12, Pa13, Pa15, PfC, Port},
    sercom::{PadPin, SPIMaster2},
    time::MegaHertz,
};

pub fn nina_spi_master(
    clocks: &mut GenericClockController,
    pm: &mut hal::pac::PM,
    sercom: hal::pac::SERCOM2,
    miso: Pa13<Input<Floating>>,
    mosi: Pa12<Input<Floating>>,
    sck: Pa15<Input<Floating>>,
    port: &mut Port,
) -> SPIMaster2<
    hal::sercom::Sercom2Pad1<Pa13<PfC>>,
    hal::sercom::Sercom2Pad0<Pa12<PfC>>,
    hal::sercom::Sercom2Pad3<Pa15<PfC>>,
> {
    let gclk0 = clocks.gclk0();
    SPIMaster2::new(
        &clocks.sercom2_core(&gclk0).unwrap(),
        MegaHertz(8),
        embedded_hal::spi::Mode {
            phase: embedded_hal::spi::Phase::CaptureOnFirstTransition,
            polarity: embedded_hal::spi::Polarity::IdleLow,
        },
        sercom,
        pm,
        (miso.into_pad(port), mosi.into_pad(port), sck.into_pad(port)),
    )
}
