use {crate::config::CONFIG, dht_sensor::dht22};

pub struct Measurement {
    location: &'static str,
    name: &'static str,
    temperature: f32,
    relative_humidity: f32,
}

impl Measurement {
    pub fn new(location: &'static str, name: &'static str, reading: dht22::Reading) -> Self {
        Self {
            location,
            name,
            temperature: reading.temperature,
            relative_humidity: reading.relative_humidity,
        }
    }

    pub fn to_http_req(&self, endpoint: &'static str) -> alloc::string::String {
        let content = alloc::format!(
            "location={location}&name={name}&temperature={temperature}&relative_humidity={relative_humidity}",
            location = self.location,
            name = self.name,
            temperature = self.temperature,
            relative_humidity = self.relative_humidity
        );
        alloc::format!(
            "POST {endpoint} HTTP/1.1\r\nHost: {host}:{port}\r\nUser-Agent: {user_agent}\r\nContent-Type: application/x-www-form-urlencoded\r\nContent-Length: {content_length}\r\nConnection: Close\r\n\r\n{content}\r\n\r\n",
            endpoint = endpoint,
            host = CONFIG.server.host,
            port = CONFIG.server.port,
            user_agent = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
            content_length = content.len(),
            content = content
        )
    }
}
