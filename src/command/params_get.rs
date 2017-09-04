use std::error;
use std::default::Default;

use clap;
use rusoto_ssm;
use rusoto_ssm::Ssm;
use config;

use super::error::CommandError;
use super::params::ParamsExecuter;

pub struct ParamsGetCommand<'c> {
  config: &'c config::command::Config,
  names: Vec<&'c str>,
}

impl<'c> ParamsGetCommand<'c> {
  pub fn from_args(config: &'c config::command::Config, args: &'c clap::ArgMatches<'c>) -> Self {
    debug!("ParamsGetCommand::from_args");

    let names = args.value_of("NAMES").unwrap()
        .split(",")
        .collect::<Vec<&str>>();

    ParamsGetCommand { 
      config: config,
      names: names,
    }
  }

  pub fn new(config: &'c config::command::Config, names: &'c Vec<&'c str>) -> Self {
    debug!("ParamsGetCommand::new");

    ParamsGetCommand { 
      config: config,
      names: names.to_owned(),
    }
  }

  pub fn run(&self) -> Result<(), Box<error::Error>> {
    debug!("ParamsGetCommand::run");
    if let Some(params_config) = self.config.params.as_ref() {

      let exec = ParamsGetExecuter::from_config(params_config);
      try!(exec.run(&self.names));
    }
    Ok(())
  }

}

pub struct ParamsGetExecuter<'c> {
  config: &'c config::command::ParamsConfig,
}

impl<'c> ParamsGetExecuter<'c> {

  pub fn from_config(config: &'c config::command::ParamsConfig) -> Self {
    debug!("ParamsGetExecuter::new");

    ParamsGetExecuter { 
      config: config,
    }
  }

  pub fn run(&self, names: &Vec<&str>) -> Result<(), Box<error::Error>> {
    debug!("ParamsGetExecuter::run");

    let names_with_path = names.iter().map(|n| self.name_with_path(n)).collect();

    let with_decription = self.config.secure.is_some();

    let req = rusoto_ssm::GetParametersRequest {
      names: names_with_path,
      with_decryption: Some(with_decription),
      ..Default::default()
    };

    let client = self.client();
    let res = try!(client.get_parameters(&req));
    info!("get parameters successfully: {}", names.join(""));

    if let Some(params) = res.parameters {
      self.print(&params);
    }

    Ok(())
  }

  fn print(&self, params: &Vec<rusoto_ssm::Parameter>) {
    for param in params.iter() {
      if let (Some(name_with_path), Some(value)) = (param.name.as_ref(), param.value.as_ref()) {
        if let Ok(name) = self.strip_path(name_with_path) {
          println!("{}={} ", name, value);
        }
      }
    }
  }

  fn strip_path<'a>(&self, name: &'a str) -> Result<&'a str, Box<error::Error>> {
    let path = self.path();
    if name.starts_with(&path) {
      return Ok(name.trim_left_matches(&path))
    } else {
      Err(Box::new(CommandError::Unknown))
    }
  }

}

impl<'c> ParamsExecuter for ParamsGetExecuter<'c> {
  fn config(&self) -> &config::command::ParamsConfig {
    &self.config
  }
}
