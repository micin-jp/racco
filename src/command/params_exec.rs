use std::error;
use std::default::Default;

use clap;
use rusoto_ssm;
use rusoto_ssm::Ssm;

use config;

use super::params::ParamsExecuter;

use std::process::Command;


type Program<'a> =&'a str;
type Arguments<'a> = Vec<&'a str>;

pub struct ParamsExecCommand<'c> {
  config: &'c config::command::Config,
  program: Program<'c>,
  args: Arguments<'c>,
}

impl<'c> ParamsExecCommand<'c> {
  pub fn from_args(config: &'c config::command::Config, clap_args: &'c clap::ArgMatches<'c>) -> Self {
    debug!("ParamsExecCommand::from_args");

    let program = clap_args.value_of("PROGRAM").unwrap();
    let args = match clap_args.values_of("ARGS") {
      Some(args) => args.collect(),
      None => Vec::new()
    };

    ParamsExecCommand { 
      config: config,
      program: program,
      args: args
    }
  }

  pub fn new(config: &'c config::command::Config, program: &'c Program<'c>, args: &'c Arguments<'c>) -> Self {
    debug!("ParamsExecCommand::new");

    ParamsExecCommand { 
      config: config,
      program: program,
      args: args.to_owned(),
    }
  }

  pub fn run(&self) -> Result<(), Box<error::Error>> {
    debug!("ParamsExecCommand::run");
    if let Some(params_config) = self.config.params.as_ref() {
      let exec = ParamsExecExecuter::from_config(params_config);
      try!(exec.run(&self.program, &self.args));
    }
    Ok(())
  }
}

pub struct ParamsExecExecuter<'c> {
  config: &'c config::command::ParamsConfig,
}

impl<'c> ParamsExecExecuter<'c> {

  pub fn from_config(config: &'c config::command::ParamsConfig) -> Self {
    debug!("ParamsGetExecuter::new");

    ParamsExecExecuter { 
      config: config,
    }
  }

  pub fn run(&self, program: &'c Program<'c>, args: &'c Arguments<'c>) -> Result<(), Box<error::Error>> {
    debug!("ParamsExecExecuter::run");

    info!("exec: {} {}", program, args.join(" "));
    let maybe_params = try!(self.params());
    let mut cmd = Command::new(program);

    cmd.args(args);

    if let Some(params) = maybe_params {
      for param in params.iter() {
        if let (Some(name_with_path), Some(value)) = (param.name.as_ref(), param.value.as_ref()) {
          if let Ok(name) = self.strip_path(name_with_path) {
            cmd.env(name, value);
          }
        }
      }
    }

    let _result = cmd.spawn();

    Ok(())
  }

  pub fn params(&self) -> Result<Option<Vec<rusoto_ssm::Parameter>>, Box<error::Error>> {
    let path = self.path();
    let with_decription = self.config.secure.is_some();

    let req = rusoto_ssm::GetParametersByPathRequest {
      path: path,
      with_decryption: Some(with_decription),
      ..Default::default()
    };

    let client = self.client();
    let res = try!(client.get_parameters_by_path(&req));
    info!("get parameters-by-path successfully");

    Ok(res.parameters)
  }

}

impl<'c> ParamsExecuter for ParamsExecExecuter<'c> {
  fn config(&self) -> &config::command::ParamsConfig {
    &self.config
  }
}
