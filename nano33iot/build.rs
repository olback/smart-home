use {serde::Deserialize, std::fs, toml};

const INPUT: &'static str = "Config.toml";
const TEMPLATE: &'static str = "config.rs.in";
const OUTPUT: &'static str = "src/config.rs";

#[derive(Debug, Deserialize)]
struct WiFiConfig {
    ssid: String,
    key: String,
}

#[derive(Debug, Deserialize)]
struct ServerConfig {
    apikey: String,
    host: String,
    port: u16,
    tls: bool,
}

#[derive(Debug, Deserialize)]
struct Config {
    wifi: WiFiConfig,
    server: ServerConfig,
}

fn main() {
    println!("cargo:rerun-if-changed=Config.toml");

    let config_str = fs::read_to_string(INPUT).unwrap();
    let config: Config = toml::from_str(&config_str).unwrap();

    let mut template_str = fs::read_to_string(TEMPLATE).unwrap();

    // Wifi
    template_str = template_str.replace("{ssid}", &config.wifi.ssid);
    template_str = template_str.replace("{key}", &config.wifi.key);

    // Server
    template_str = template_str.replace("{apikey}", &config.server.apikey);
    template_str = template_str.replace("{host}", &config.server.host);
    template_str = template_str.replace("{port}", &config.server.port.to_string());
    template_str = template_str.replace("{tls}", &config.server.tls.to_string());

    fs::write(OUTPUT, template_str).unwrap();
}
