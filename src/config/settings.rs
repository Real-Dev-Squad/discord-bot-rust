use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Discord {
  pub token: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
  pub debug: bool,
  pub discord: Discord,
}

impl Settings {
  pub fn new() -> Result<Self, ConfigError> {
    let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

    let s = Config::builder()
      // Start off by merging in the "default" configuration file
      .add_source(File::with_name("src/config/default"))
      // Add in the current environment file
      // Default to 'development' env
      // Note that this file is _optional_
      .add_source(File::with_name(&format!("src/config/{}", run_mode)).required(false))
      // Add in a local configuration file
      // This file shouldn't be checked in to git
      .add_source(File::with_name("src/config/local").required(false))
      // Add in settings from the environment (with a prefix of APP)
      // Eg.. `APP__DEBUG=1 ./target/app` would set the `debug` key
      .add_source(Environment::with_prefix("app").separator("__"))
      .build()?;

    // You can deserialize (and thus freeze) the entire configuration as
    s.try_deserialize()
  }
}
