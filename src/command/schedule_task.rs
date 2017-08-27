use std::error;

use clap;
use hyper;
use rusoto_core::{default_tls_client, DefaultCredentialsProvider, Region};
use rusoto_ecs::{ EcsClient };
use rusoto_events::{ CloudWatchEventsClient };
use config;

use super::error::CommandError;
use super::ecs::EcsExecuter;
use super::cloudwatch_events::CloudWatchEventsExecuter;

pub struct ScheduleTaskCommand<'c> {
  config: &'c config::command::Config,
  name: &'c str,
}

impl<'c> ScheduleTaskCommand<'c> {
  pub fn from_args(config: &'c config::command::Config, args: &'c clap::ArgMatches<'c>) -> Self {
    debug!("ScheduleTaskCommand::from_args");

    ScheduleTaskCommand { 
      config: config,
      name: args.value_of("NAME").unwrap(),
    }
  }

  pub fn new(config: &'c config::command::Config, name: &'c str) -> Self {
    debug!("ScheduleTaskCommand::new");

    ScheduleTaskCommand { 
      config: config,
      name: name,
    }
  }

  pub fn run(&self) -> Result<(), Box<error::Error>> {
    debug!("ScheduleTaskCommand::run");

    if let Some(schedule_config_group) = self.config.schedule_task.as_ref() {
      for schedule_config in schedule_config_group {
        if schedule_config.name != self.name {
          continue;
        }

        let schedule_cmd = ScheduleTaskExecuter::from_config(&schedule_config);
        try!(schedule_cmd.run());
      }
    }

    Ok(())
  }
}


pub struct ScheduleTaskExecuter<'c>
{
  ecs_client: EcsClient<DefaultCredentialsProvider, hyper::client::Client>,
  events_client: CloudWatchEventsClient<DefaultCredentialsProvider, hyper::client::Client>,
  config: &'c config::command::ScheduleTaskConfig
}

impl<'c> ScheduleTaskExecuter<'c> {
  pub fn from_config(config: &'c config::command::ScheduleTaskConfig) -> Self {
    debug!("ScheduleTaskExecuter::from_config");

    let ecs_client = EcsClient::new(default_tls_client().unwrap(), DefaultCredentialsProvider::new().unwrap(), Region::ApNortheast1);
    let events_client = CloudWatchEventsClient::new(default_tls_client().unwrap(), DefaultCredentialsProvider::new().unwrap(), Region::ApNortheast1);

    ScheduleTaskExecuter { 
      ecs_client: ecs_client,
      events_client: events_client,
      config: config
    }
  }

  pub fn run(&self) -> Result<(), Box<error::Error>> {
    debug!("ScheduleTaskExecuter::run");

    let maybe_ecs_cluster = try!(self.describe_cluster(&self.config.cluster));
    let ecs_cluster = try!(maybe_ecs_cluster.ok_or(Box::new(CommandError::Unknown)));
    let ecs_cluster_arn = try!(ecs_cluster.cluster_arn.as_ref().ok_or(Box::new(CommandError::Unknown)));

    let task_definition = try!(self.register_task_definition(&self.config.task_definition));
    let task_definition_arn = try!(task_definition.task_definition_arn.as_ref().ok_or(Box::new(CommandError::Unknown)));

    let role_arn = self.config.rule_targets_role_arn.as_ref().map(String::as_str);

    try!(self.put_rule(&self.config.rule));
    try!(self.put_ecs_task_target(&self.config.rule, role_arn, ecs_cluster_arn, task_definition_arn));

    Ok(())
  }
}

impl<'c> EcsExecuter for ScheduleTaskExecuter<'c> {
  fn ecs_client(&self) -> &EcsClient<DefaultCredentialsProvider, hyper::client::Client> {
    &self.ecs_client
  }
}

impl<'c> CloudWatchEventsExecuter for ScheduleTaskExecuter<'c> {
  fn events_client(&self) -> &CloudWatchEventsClient<DefaultCredentialsProvider, hyper::client::Client> {
    &self.events_client
  }
}