use std::fs::File;
use std::error;
use std::fmt;
use std::io::prelude::*;

use serde;
use serde_yaml;

use super::ecs;

#[derive(Debug)]
pub enum ConfigError {
  ParseError(serde_yaml::Error),
}

impl fmt::Display for ConfigError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      ConfigError::ParseError(ref yaml_err) => write!(f, "Parse Error: {}", yaml_err),
    }
  }
}

impl error::Error for ConfigError {
  fn description(&self) -> &str {
      match *self {
        ConfigError::ParseError(ref yaml_err) => yaml_err.description(),
      }
  }

  fn cause(&self) -> Option<&error::Error> {
      match *self {
        ConfigError::ParseError(ref yaml_err) => Some(yaml_err),
      }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
  pub deploy: Option<DeployConfigGroup>,
  pub run_task: Option<RunTaskConfigGroup>,
  pub params: Option<ParamsConfig>
}

impl Config {

  pub fn from_file(file_name: &str) -> Result<Config, Box<error::Error>> {
    debug!("Config::from_file");

    let mut file = try!(File::open(file_name));
    let mut contents = String::new();

    try!(file.read_to_string(&mut contents));

    debug!("Config::from_file - Yaml file: {}", contents);

    match serde_yaml::from_str::<Config>(&contents) {
      Ok(c) => {
        debug!("Config::from_file - Serialize reversely: {}", serde_yaml::to_string(&c).unwrap());
        Ok(c)
      },
      Err(e) => Err(Box::new(ConfigError::ParseError(e)))
    }
  }
}

pub type DeployConfigGroup = Vec<DeployConfig>;
 
#[derive(Debug, Serialize, Deserialize)]
pub struct DeployConfig {
  pub cluster: String,
  pub service: ecs::Service,
}

pub type RunTaskConfigGroup = Vec<RunTaskConfig>;

#[derive(Debug, Serialize, Deserialize)]
pub struct RunTaskConfig {
  pub cluster: String,
  pub task_definition: ecs::TaskDefinition,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParamsConfig {
  pub path: String,
}