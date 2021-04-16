use {
    arduino_nano33iot::{
        self as hal,
        clock::GenericClockController,
        gpio::{Floating, Input, OpenDrain, Output, Pa12, Pa13, Pa14, Pa15, Pa28, Pa8, PfC, Port},
        sercom::SPIMaster2,
        time::MegaHertz,
    },
    core::time::Duration,
    no_std_net::Ipv4Addr,
    wifi_nina::{transport::SpiTransport, types::ProtocolMode},
};

type WiFiTransport = SpiTransport<
    SPIMaster2<
        hal::sercom::Sercom2Pad1<Pa13<PfC>>,
        hal::sercom::Sercom2Pad0<Pa12<PfC>>,
        hal::sercom::Sercom2Pad3<Pa15<PfC>>,
    >,
    Pa28<Input<Floating>>,
    Pa8<Output<OpenDrain>>,
    Pa14<Output<OpenDrain>>,
    fn(core::time::Duration),
>;

type WiFiError = &'static str;
type WiFiResult<T> = core::result::Result<T, WiFiError>;

pub struct WiFi {
    client: wifi_nina::Client<WiFiTransport>,
    wifi: wifi_nina::Wifi<WiFiTransport>,
    config: wifi_nina::types::Config<'static>,
}

impl WiFi {
    pub fn new(
        config: wifi_nina::types::Config<'static>,
        clocks: &mut GenericClockController,
        pm: &mut hal::pac::PM,
        sercom2: hal::pac::SERCOM2,
        miso: Pa13<Input<Floating>>,
        mosi: Pa12<Input<Floating>>,
        sck: Pa15<Input<Floating>>,
        nina_ack: Pa28<Input<Floating>>,
        nina_resetn: Pa8<Input<Floating>>,
        nina_cs: Pa14<Input<Floating>>,
        delay: fn(core::time::Duration) -> (),
        port: &mut Port,
    ) -> WiFiResult<Self> {
        let spi_master = arduino_nano33iot::spi_master_wifi_nina(
            clocks,
            MegaHertz(4),
            sercom2,
            pm,
            sck,
            mosi,
            miso,
            port,
        );
        let transport = SpiTransport::start(
            spi_master,
            nina_ack.into_floating_input(port),
            nina_resetn.into_open_drain_output(port),
            nina_cs.into_open_drain_output(port),
            delay,
        )
        .unwrap();
        let mut wifi = wifi_nina::Wifi::new(transport);
        Ok(Self {
            config,
            client: wifi.new_client().map_err(|_| "Failed to create client")?,
            wifi,
        })
    }

    pub fn configure(&mut self, timeout: Option<Duration>) -> WiFiResult<()> {
        self.wifi
            .configure(self.config.clone(), timeout)
            .map_err(|_| "Failed to connect to network")
    }

    fn connect_ipv4(&mut self, ip: Ipv4Addr, port: u16, max_tries: u8) -> WiFiResult<()> {
        match self
            .client
            .connect_ipv4(&mut self.wifi, ip, port, ProtocolMode::Tcp)
        {
            Ok(_) => Ok(()),
            Err(e) => match e {
                wifi_nina::Error::StartClientByIp if max_tries > 0 => {
                    log::error!("{} {:?}", max_tries, e);
                    self.configure(Some(Duration::from_secs(10)))?;
                    self.connect_ipv4(ip, port, max_tries - 1)?;
                    Ok(())
                }
                _ => Err("Failed to connect to host"),
            },
        }
    }

    pub fn http_post(
        &mut self,
        ip: Ipv4Addr,
        port: u16,
        data: &str,
        max_tries: u8,
    ) -> WiFiResult<()> {
        self.connect_ipv4(ip, port, max_tries)?;
        self.client
            .send_all(&mut self.wifi, data.as_bytes())
            .map_err(|_| "Failed to send data")?;
        Ok(())
    }
}
