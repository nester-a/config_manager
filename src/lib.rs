mod config;
mod loader;

use config::AppConfig;
pub fn load_config() -> Result<AppConfig, String> {
    loader::load_config()
}
