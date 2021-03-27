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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DatabaseConfig {
    address: String,
    port: u16,
    username: String,
    password: String,
    database: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub rocket: RocketConfig,
    pub database: DatabaseConfig,
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
