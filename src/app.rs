use envconfig::Envconfig;

#[derive(Clone)]
pub struct Application {
    config: Config,
}

#[derive(Envconfig, Clone)]
pub struct Config {
    #[envconfig(from = "DATABASE_URL")]
    pub db_url: String,
    #[envconfig(from = "BOT_VERSION", default = "unknown")]
    pub version: String,
    // #[envconfig(nested)]
    // pub bot_config: BotConfig,
    // #[envconfig(nested)]
    // pub redis_config: RedisConfig,
}

impl Application {
    pub fn new() -> Self {
        Self {
            config: Config::init_from_env().expect("Can't load config"),
        }
    }
}

impl Default for Application {
    fn default() -> Self {
        Application::new()
    }
}