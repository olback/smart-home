use {
    figment::{providers::Serialized, Figment},
    serde::{Deserialize, Serialize},
    std::collections::HashMap,
    toml,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RocketConfig {
    address: String,
    port: u16,
    keep_alive: u32,
    secret_key: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DatabaseConfig {
    address: String,
    port: u16,
    username: String,
    password: String,
    database: String,
}

#[derive(Debug, Deserialize)]
pub struct DateTimeConfig {
    #[serde(deserialize_with = "timezone_deserialize")]
    pub timezone: chrono_tz::Tz,
}

#[derive(Debug, Deserialize)]
pub struct SonosConfig {}

#[derive(Debug, Deserialize)]
pub struct TradfriConfig {
    pub ip: std::net::Ipv4Addr,
    pub key_name: String,
    pub key: String,
}

#[derive(Debug, Deserialize)]
pub struct WeatherConfig {
    pub api_key: String,
    pub lat: f32,
    pub long: f32,
}

#[derive(Debug, Deserialize)]
pub struct ShoplisticConfig {
    pub address: String,
    pub api_key: String,
}

#[derive(Debug, Deserialize)]
pub struct EmailConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub folder: String,
}

#[derive(Debug, Deserialize)]
pub struct GithubConfig {
    api_key: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    // Server
    pub rocket: RocketConfig,
    pub database: DatabaseConfig,
    // Features
    pub datetime: Option<DateTimeConfig>,
    pub sonos: Option<SonosConfig>,
    pub tradfri: Option<TradfriConfig>,
    pub weather: Option<WeatherConfig>,
    pub shoplistic: Option<ShoplisticConfig>,
    pub email: Option<HashMap<String, EmailConfig>>,
    pub github: Option<GithubConfig>,
}

impl DatabaseConfig {
    fn connection_string(&self) -> String {
        format!(
            "postgres://{user}:{password}@{address}:{port}/{dbname}",
            user = self.username,
            password = self.password,
            address = self.address,
            port = self.port,
            dbname = self.database
        )
    }
}

impl Config {
    pub fn load(path: &str) -> Result<Self, std::io::Error> {
        let config_str = std::fs::read_to_string(path)?;
        let maybe_self = toml::from_str(&config_str)?;
        Ok(maybe_self)
    }

    pub fn rocket(&self) -> Figment {
        // This should not the the way to configure a database in Rocket...
        // Update: There is a better way: https://api.rocket.rs/master/rocket_contrib/databases/index.html#procedurally
        let mut url_map = HashMap::<&'static str, String>::with_capacity(1);
        url_map.insert("url", self.database.connection_string());

        let mut rmb_map = HashMap::<&'static str, HashMap<_, _>>::with_capacity(1);
        rmb_map.insert("smart-home", url_map);

        let mut databases = HashMap::<&'static str, HashMap<_, HashMap<_, _>>>::with_capacity(1);
        databases.insert("databases", rmb_map);

        Figment::from(rocket::Config::default())
            .merge(Serialized::defaults(self.rocket.clone()))
            .merge(Serialized::defaults(databases))
    }
}

fn timezone_deserialize<'de, D>(de: D) -> Result<chrono_tz::Tz, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let tz: &str = serde::de::Deserialize::deserialize(de)?;
    tz.parse().map_err(serde::de::Error::custom)
}
