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

    pub fn is_ok(&self) -> bool {
        self.database_url.len() > 0 && self.port > 0 && !self.feature_flags.is_empty()
    }
}
