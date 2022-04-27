#[derive(serde::Deserialize, Clone, Debug)]
pub struct Settings {
    pub deribit: DeribitSettings,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct DeribitSettings {
    pub client_id: String,
    pub client_secret: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    // Read the "default" configuration file
    settings.merge(config::File::from(configuration_directory.join("base")).required(true))?;

    // Detect the running environment.
    // Default to 'local' if unspecified.
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT.");

    // Layer on the environment-specifict values.
    let data =
        config::File::from(configuration_directory.join(environment.as_str())).required(true);
    settings.merge(data)?;

    // Add in settings from environment variables (with a prefix of APP and '__' as separator)
    // E.g. 'APP__PORT=5001' would set 'Settings.application.port'
    // settings.merge(config::Environment::with_prefix("app").separator("__"))?;

    settings.try_into()
}

// The possible runtime environment for our application.
pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either 'local' or 'production'.",
                other
            )),
        }
    }
}
