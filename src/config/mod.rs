use std::collections::HashMap;

#[derive(Debug)]
pub struct Config {
    pub env: HashMap<String, String>,
}

impl Config {
    pub fn load() -> Result<Self, Error> {
        let config = Self { env: load_env() };

        // debug!("{}", config);

        Ok(config)
    }
}

fn load_env() -> HashMap<String, String> {
    let mut env = HashMap::new();

    env
}
