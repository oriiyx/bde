use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Generate,
}

#[derive(serde::Deserialize)]
pub struct Settings {
    pub sql: SqlSettings,
}

#[derive(serde::Deserialize)]
pub struct SqlSettings {
    pub schemas: String,
    pub queries: String,
    pub output: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");

    let settings = config::Config::builder()
        .add_source(config::File::from(base_path.join("bde.yaml")))
        .build()?;

    settings.try_deserialize::<Settings>()
}
