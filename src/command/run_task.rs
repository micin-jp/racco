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
  args: &'c clap::ArgMatches<'c>,
}

impl<'c> RunTaskCommand<'c> {
  pub fn from_config(config: &'c config::command::Config, args: &'c clap::ArgMatches<'c>) -> Self {
    debug!("RunTaskCommand::new");

    RunTaskCommand { 
      config: config,
      args: args,
    }
  }

  pub fn run(&self) -> Result<(), Box<error::Error>> {
    debug!("RunTaskCommand::run");

    let name = self.args.value_of("NAME").unwrap();

    if let Some(run_task_config_group) = self.config.run_task.as_ref() {
      for run_task_config in run_task_config_group {
        if run_task_config.name != name {
          continue;
        }

        let ecs_run_task_cmd = EcsRunTaskExecuter::from_config(&run_task_config);
        try!(ecs_run_task_cmd.run());
      }
    }

    Ok(())
  }
}


pub struct EcsRunTaskExecuter<'c>
{
  client: EcsClient<DefaultCredentialsProvider, hyper::client::Client>,
  config: &'c config::command::RunTaskConfig
}

impl<'c> EcsRunTaskExecuter<'c> {
  pub fn from_config(config: &'c config::command::RunTaskConfig) -> Self {
    debug!("EcsRunTaskCommand::new");

    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = EcsClient::new(default_tls_client().unwrap(), credentials, Region::ApNortheast1);
    EcsRunTaskExecuter { 
      client: client,
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

impl<'c> EcsExecuter for EcsRunTaskExecuter<'c> {
  fn client(&self) -> &EcsClient<DefaultCredentialsProvider, hyper::client::Client> {
    &self.client
  }
}