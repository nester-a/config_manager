use crate::config::AppConfig;
use std::collections::HashMap;

fn get_default_config() -> AppConfig {
    AppConfig::new(
        String::from("localhost"),
        5433,
        HashMap::from([(String::from("dark_mode"), true)]),
    )
}

pub fn load_config() -> Result<AppConfig, String> {
    let config = get_default_config();
    if config.is_ok() {
        Ok(config)
    }
    else {
        Err(String::from("default config doesn't exists"))
    }
}
