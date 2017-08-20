use std::error;

use clap;
use hyper;
use rusoto_core::{default_tls_client, DefaultCredentialsProvider, Region};
use rusoto_ecs::{ EcsClient };
use config;

use super::error::CommandError;
use super::ecs::EcsExecuter;

pub struct RunTaskCommand<'c> {
  config: &'c config::command::Config,
  name: &'c str,
}

impl<'c> RunTaskCommand<'c> {
  pub fn from_args(config: &'c config::command::Config, args: &'c clap::ArgMatches<'c>) -> Self {
    debug!("RunTaskCommand::from_args");

    RunTaskCommand { 
      config: config,
      name: args.value_of("NAME").unwrap(),
    }
  }

  pub fn new(config: &'c config::command::Config, name: &'c str) -> Self {
    debug!("RunTaskCommand::new");

    RunTaskCommand { 
      config: config,
      name: name,
    }
  }

  pub fn run(&self) -> Result<(), Box<error::Error>> {
    debug!("RunTaskCommand::run");

    if let Some(run_task_config_group) = self.config.run_task.as_ref() {
      for run_task_config in run_task_config_group {
        if run_task_config.name != self.name {
          continue;
        }

        let ecs_run_task_cmd = RunTaskExecuter::from_config(&run_task_config);
        try!(ecs_run_task_cmd.run());
      }
    }

    Ok(())
  }
}


pub struct RunTaskExecuter<'c>
{
  ecs_client: EcsClient<DefaultCredentialsProvider, hyper::client::Client>,
  config: &'c config::command::RunTaskConfig
}

impl<'c> RunTaskExecuter<'c> {
  pub fn from_config(config: &'c config::command::RunTaskConfig) -> Self {
    debug!("RunTaskExecuter::from_config");

    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = EcsClient::new(default_tls_client().unwrap(), credentials, Region::ApNortheast1);
    RunTaskExecuter { 
      ecs_client: client,
      config: config
    }
  }

  pub fn run(&self) -> Result<(), Box<error::Error>> {

    let task_definition = try!(self.register_task_definition(&self.config.task_definition));
    let task_definition_arn = try!(task_definition.task_definition_arn.as_ref().ok_or(Box::new(CommandError::Unknown)));

    try!(self.run_task(&self.config.cluster, &task_definition_arn));

    Ok(())
  }
}

impl<'c> EcsExecuter for RunTaskExecuter<'c> {
  fn ecs_client(&self) -> &EcsClient<DefaultCredentialsProvider, hyper::client::Client> {
    &self.ecs_client
  }
}