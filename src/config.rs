pub struct AppConfig {
    database_ulr: String,
    port: u16,
    feature_flags: HashMap<String, bool>,
}
