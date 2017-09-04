use std::error;
use std::default::Default;

use clap;
use rusoto_ssm;
use rusoto_ssm::Ssm;
use config;

use super::params::ParamsExecuter;

pub struct ParamsGetCommand<'c> {
  config: &'c config::command::Config,
  name: &'c str,
}

impl<'c> ParamsGetCommand<'c> {
  pub fn from_args(config: &'c config::command::Config, args: &'c clap::ArgMatches<'c>) -> Self {
    debug!("ParamsGetCommand::from_args");

    let name = args.value_of("NAME").unwrap();

    ParamsGetCommand { 
      config: config,
      name: name,
    }
  }

  pub fn new(config: &'c config::command::Config, name: &'c str) -> Self {
    debug!("ParamsGetCommand::new");

    ParamsGetCommand { 
      config: config,
      name: name,
    }
  }

  pub fn run(&self) -> Result<(), Box<error::Error>> {
    debug!("ParamsGetCommand::run");
    if let Some(params_config) = self.config.params.as_ref() {

      let exec = ParamsGetExecuter::from_config(params_config);
      try!(exec.run(&self.name));
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

  pub fn run(&self, name: &str) -> Result<(), Box<error::Error>> {
    debug!("ParamsGetExecuter::run");

    let name_with_path = self.name_with_path(name);
    let with_decription = self.config.secure.is_some();

    let req = rusoto_ssm::GetParameterRequest {
      name: name_with_path,
      with_decryption: Some(with_decription),
      ..Default::default()
    };

    let client = self.client();
    let res = try!(client.get_parameter(&req));
    info!("get parameters successfully: {}", name);

    if let Some(params) = res.parameter {
      self.print(&params);
    }

    Ok(())
  }

  fn print(&self, param: &rusoto_ssm::Parameter) {
    if let Some(val) = param.value.as_ref() {
      println!("{}", val);
    }
  }

}

impl<'c> ParamsExecuter for ParamsGetExecuter<'c> {
  fn config(&self) -> &config::command::ParamsConfig {
    &self.config
  }
}
