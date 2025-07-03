#[derive(Clone)]
pub struct VerihubsConfig {
    pub api_key: String,
    pub app_id: String,
}

impl VerihubsConfig {
    pub fn new(api_key: String, app_id: String) -> Self {
        Self {
            api_key,
            app_id,
        }
    }
}