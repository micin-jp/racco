use std::error;

use clap;
use hyper;
use rusoto_core::{default_tls_client, DefaultCredentialsProvider, Region};
use rusoto_ecs::{ EcsClient };
use rusoto_events::{ CloudWatchEventsClient };
use config;

use super::ecs::EcsExecuter;
use super::cloudwatch_events::CloudWatchEventsExecuter;

pub struct ScheduleTaskDeleteCommand<'c> {
  config: &'c config::command::Config,
  name: &'c str,
}

impl<'c> ScheduleTaskDeleteCommand<'c> {
  pub fn from_args(config: &'c config::command::Config, args: &'c clap::ArgMatches<'c>) -> Self {
    debug!("ScheduleTaskDeleteCommand::from_args");

    ScheduleTaskDeleteCommand { 
      config: config,
      name: args.value_of("NAME").unwrap(),
    }
  }

  pub fn new(config: &'c config::command::Config, name: &'c str) -> Self {
    debug!("ScheduleTaskDeleteCommand::new");

    ScheduleTaskDeleteCommand { 
      config: config,
      name: name,
    }
  }

  pub fn run(&self) -> Result<(), Box<error::Error>> {
    debug!("ScheduleTaskDeleteCommand::run");

    let schedule_del_exec = ScheduleTaskDeleteExecuter::new();
    try!(schedule_del_exec.run(self.name));

    Ok(())
  }
}


pub struct ScheduleTaskDeleteExecuter
{
  ecs_client: EcsClient<DefaultCredentialsProvider, hyper::client::Client>,
  events_client: CloudWatchEventsClient<DefaultCredentialsProvider, hyper::client::Client>,
}

impl ScheduleTaskDeleteExecuter {
  pub fn new() -> Self {
    debug!("ScheduleTaskDeleteExecuter::new");

    let ecs_client = EcsClient::new(default_tls_client().unwrap(), DefaultCredentialsProvider::new().unwrap(), Region::ApNortheast1);
    let events_client = CloudWatchEventsClient::new(default_tls_client().unwrap(), DefaultCredentialsProvider::new().unwrap(), Region::ApNortheast1);

    ScheduleTaskDeleteExecuter { 
      ecs_client: ecs_client,
      events_client: events_client,
    }
  }

  pub fn run(&self, rule_name: &str) -> Result<(), Box<error::Error>> {
    debug!("ScheduleTaskDeleteExecuter::run");

    try!(self.delete_rule(rule_name));

    Ok(())
  }
}

impl EcsExecuter for ScheduleTaskDeleteExecuter {
  fn ecs_client(&self) -> &EcsClient<DefaultCredentialsProvider, hyper::client::Client> {
    &self.ecs_client
  }
}

impl CloudWatchEventsExecuter for ScheduleTaskDeleteExecuter {
  fn events_client(&self) -> &CloudWatchEventsClient<DefaultCredentialsProvider, hyper::client::Client> {
    &self.events_client
  }
}