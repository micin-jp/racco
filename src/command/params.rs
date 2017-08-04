use std::error;
use std::default::Default;

use clap;
use hyper;
use rusoto_core::{default_tls_client, DefaultCredentialsProvider, Region};
use rusoto_ssm;
use rusoto_ssm::{Ssm, SsmClient};
use config;

use super::error::CommandError;

pub trait ParamsExecuter {

  fn client(&self) -> SsmClient<DefaultCredentialsProvider, hyper::client::Client> {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    return SsmClient::new(default_tls_client().unwrap(), credentials, Region::ApNortheast1);
  }

  fn config(&self) -> &config::command::ParamsConfig;

  fn name_with_path(&self, name: &str) -> String {
    let mut path = self.path();
    path.push_str(name);
    path
  }

  fn path(&self) -> String {
    let mut path = self.config().path.to_owned();
    if !path.ends_with("/") {
      path.push_str("/");
    }
    if !path.starts_with("/") {
      path = format!("/{}", path);
    }
    path
  }

}

pub struct ParamsGetCommand<'c> {
  config: &'c config::command::Config,
  args: &'c clap::ArgMatches<'c>,
}

impl<'c> ParamsGetCommand<'c> {
  pub fn from_config(config: &'c config::command::Config, args: &'c clap::ArgMatches<'c>) -> Self {
    debug!("ParamsGetCommand::new");

    ParamsGetCommand { 
      config: config,
      args: args,
    }
  }

  pub fn run(&self) -> Result<(), Box<error::Error>> {
    debug!("ParamsGetCommand::run");
    if let Some(params_config) = self.config.params.as_ref() {

      let names = self.args.value_of("NAMES").unwrap()
          .split(",")
          .collect::<Vec<&str>>();

      let exec = ParamsGetExecuter::from_config(params_config);
      try!(exec.run(&names));
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

  fn print(&self, params: &rusoto_ssm::ParameterList) {
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


pub struct ParamsPutCommand<'c> {
  config: &'c config::command::Config,
  args: &'c clap::ArgMatches<'c>,
}

impl<'c> ParamsPutCommand<'c> {
  pub fn from_config(config: &'c config::command::Config, args: &'c clap::ArgMatches<'c>) -> Self {
    debug!("ParamsPutCommand::new");
    ParamsPutCommand { 
      config: config,
      args: args,
    }
  }

  pub fn run(&self) -> Result<(), Box<error::Error>> {
    debug!("ParamsPutCommand::run");
    if let Some(params_config) = self.config.params.as_ref() {
      let name = self.args.value_of("NAME").unwrap();
      let val = self.args.value_of("VALUE").unwrap();

      let exec = ParamsPutExecuter::from_config(&params_config);

      try!(exec.run(name, val));
    }
    Ok(())
  }

}

pub struct ParamsPutExecuter<'c> {
  config: &'c config::command::ParamsConfig,
}

impl<'c> ParamsPutExecuter<'c> {
  pub fn from_config(config: &'c config::command::ParamsConfig) -> Self {
    debug!("ParamsPutExecuter::new");

    ParamsPutExecuter { 
      config: config,
    }
  }
  
  pub fn run(&self, name: &str, value: &str) -> Result<(), Box<error::Error>> {
    debug!("ParamsPutExecuter::run");

    let (type_, key_id) = if let Some(secure) = self.config.secure.as_ref() {
      (String::from("SecureString"), Some(secure.key.to_owned()))
    } else {
      (String::from("String"), None)
    };

    let req = rusoto_ssm::PutParameterRequest {
      name: self.name_with_path(name),
      value: value.to_owned(),
      type_: type_,
      key_id: key_id,
      overwrite: Some(true),
      ..Default::default()
    };

    let client = self.client();
    try!(client.put_parameter(&req));
    info!("put parameter successfully: {} => {}", name, value);

    Ok(())
  }

}

impl<'c> ParamsExecuter for ParamsPutExecuter<'c> {
  fn config(&self) -> &config::command::ParamsConfig {
    &self.config
  }
}


pub struct ParamsDeleteCommand<'c> {
  config: &'c config::command::Config,
  args: &'c clap::ArgMatches<'c>,
}

impl<'c> ParamsDeleteCommand<'c> {
  pub fn from_config(config: &'c config::command::Config, args: &'c clap::ArgMatches<'c>) -> Self {
    debug!("ParamsDeleteCommand::new");

    ParamsDeleteCommand { 
      config: config,
      args: args,
    }
  }

  pub fn run(&self) -> Result<(), Box<error::Error>> {
    debug!("ParamsDeleteCommand::run");

    if let Some(params_config) = self.config.params.as_ref() {
      let name = self.args.value_of("NAME").unwrap();

      let exec = ParamsDeleteExecuter::from_config(&params_config);

      try!(exec.run(name));
    }
    Ok(())
  }
}

pub struct ParamsDeleteExecuter<'c> {
  config: &'c config::command::ParamsConfig,
}

impl<'c> ParamsDeleteExecuter<'c> {
  pub fn from_config(config: &'c config::command::ParamsConfig) -> Self {
    debug!("ParamsDeleteExecuter::new");

    ParamsDeleteExecuter { 
      config: config,
    }
  }

  pub fn run(&self, name: &str) -> Result<(), Box<error::Error>> {
    debug!("ParamsDeleteExecuter::run");

    let req = rusoto_ssm::DeleteParameterRequest {
      name: self.name_with_path(name),
      ..Default::default()
    };

    let client = self.client();
    try!(client.delete_parameter(&req));
    info!("delete parameter successfully: {}", name);

    Ok(())
  }
}

impl<'c> ParamsExecuter for ParamsDeleteExecuter<'c> {
  fn config(&self) -> &config::command::ParamsConfig {
    &self.config
  }
}