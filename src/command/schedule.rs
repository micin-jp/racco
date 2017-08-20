use std::error;

use clap;
use hyper;
use rusoto_core::{default_tls_client, DefaultCredentialsProvider, Region};
use rusoto_ecs::{ EcsClient };
use config;

use super::error::CommandError;
use super::ecs::EcsExecuter;

pub struct ScheduleCommand<'c> {
  config: &'c config::command::Config,
  name: &'c str,
}

impl<'c> ScheduleCommand<'c> {
  pub fn from_args(config: &'c config::command::Config, args: &'c clap::ArgMatches<'c>) -> Self {
    debug!("ScheduleCommand::from_args");

    ScheduleCommand { 
      config: config,
      name: args.value_of("NAME").unwrap(),
    }
  }

  pub fn new(config: &'c config::command::Config, name: &'c str) -> Self {
    debug!("ScheduleCommand::new");

    ScheduleCommand { 
      config: config,
      name: name,
    }
  }

  pub fn run(&self) -> Result<(), Box<error::Error>> {
    debug!("ScheduleCommand::run");

    if let Some(schedule_config_group) = self.config.schedule.as_ref() {
      for schedule_config in schedule_config_group {
        if schedule_config.name != self.name {
          continue;
        }

        let schedule_cmd = ScheduleExecuter::from_config(&schedule_config);
        try!(schedule_cmd.run());
      }
    }

    Ok(())
  }
}


pub struct ScheduleExecuter<'c>
{
  ecs_client: EcsClient<DefaultCredentialsProvider, hyper::client::Client>,
  config: &'c config::command::ScheduleConfig
}

impl<'c> ScheduleExecuter<'c> {
  pub fn from_config(config: &'c config::command::ScheduleConfig) -> Self {
    debug!("ScheduleExecuter::from_config");

    let credentials = DefaultCredentialsProvider::new().unwrap();
    let ecs_client = EcsClient::new(default_tls_client().unwrap(), credentials, Region::ApNortheast1);
    ScheduleExecuter { 
      ecs_client: ecs_client,
      config: config
    }
  }

  pub fn run(&self) -> Result<(), Box<error::Error>> {

    
    let task_definition = try!(self.register_task_definition(&self.config.task_definition));
    let task_definition_arn = try!(task_definition.task_definition_arn.as_ref().ok_or(Box::new(CommandError::Unknown)));


    Ok(())
  }
}

impl<'c> EcsExecuter for ScheduleExecuter<'c> {
  fn ecs_client(&self) -> &EcsClient<DefaultCredentialsProvider, hyper::client::Client> {
    &self.ecs_client
  }
}