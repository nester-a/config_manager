use std::collections::HashMap;

pub struct AppConfig {
    database_url: String,
    port: u16,
    feature_flags: HashMap<String, bool>,
}

impl AppConfig{
    pub fn new(database_url: String, port: u16, feature_flags:HashMap<String, bool>) -> AppConfig{
        AppConfig { database_url, port, feature_flags }
    }
}
