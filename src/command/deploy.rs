use std::error;
use std::time::Duration;
use std::thread::sleep;

use clap;
use hyper;
use rusoto_core::{default_tls_client, DefaultCredentialsProvider, Region};
use rusoto_ecs;
use rusoto_ecs::{ EcsClient };

use config;
use output;

use super::error::CommandError;
use super::ecs::EcsExecuter;

pub struct DeployCommand<'c>
{
  config: &'c config::command::Config,
  name: Option<&'c str>,
}

impl<'c> DeployCommand<'c> {
  pub fn from_args(config: &'c config::command::Config, args: &'c clap::ArgMatches<'c>) -> Self {
    debug!("DeployCommand::from_args");

    DeployCommand { 
      config: config,
      name: args.value_of("NAME"),
    }
  }

  pub fn new(config: &'c config::command::Config, name: Option<&'c str>) -> Self {
    debug!("DeployCommand::new");

    DeployCommand { 
      config: config,
      name: name,
    }
  }

  pub fn run(&self) -> Result<(), Box<error::Error>> {
    debug!("DeployCommand::run");

    if let Some(deploy_config_group) = self.config.deploy.as_ref() {
      for deploy_config in deploy_config_group {
        if let Some(name) = self.name {
          if name != deploy_config.name {
            continue;
          }
        }

        let ecs_deploy_cmd = DeployExecuter::from_config(&deploy_config);
        try!(ecs_deploy_cmd.run());
      }
    }

    Ok(())
  }
}


pub struct DeployExecuter<'c>
{
  ecs_client: EcsClient<DefaultCredentialsProvider, hyper::client::Client>,
  config: &'c config::command::DeployConfig
}

impl<'c> DeployExecuter<'c> {
  pub fn from_config(config: &'c config::command::DeployConfig) -> Self {
    debug!("DeployExecuter::from_config");

    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = EcsClient::new(default_tls_client().unwrap(), credentials, Region::ApNortheast1);
    DeployExecuter { 
      ecs_client: client,
      config: config
    }
  }

  pub fn run(&self) -> Result<(), Box<error::Error>> {
    debug!("DeployExecuter::run");

    let service_conf = &self.config.service;
    let cluster = &self.config.cluster;

    let maybe_latest_task_definition = try!(self.describe_latest_task_definition(&service_conf.task_definition.family));

    let task_definition = 
      if let Some(latest_task_definition) = maybe_latest_task_definition {
        if self.detect_task_definition_changes(&service_conf.task_definition, &latest_task_definition) {
          output::PrintLine::info("Registering a task definition");
          try!(self.register_task_definition(&service_conf.task_definition))
        } else {
          latest_task_definition
        }
      } else {
        output::PrintLine::info("Registering a task definition");
        try!(self.register_task_definition(&service_conf.task_definition))
      };
    
    let task_definition_arn = try!(task_definition.task_definition_arn.as_ref().ok_or(Box::new(CommandError::Unknown)));

    let maybe_service = try!(self.describe_service(cluster, &service_conf));

    let _service: rusoto_ecs::Service = match maybe_service {
      Some(s) => s,
      None => {
        output::PrintLine::info("Service has not been exist. Creating...");
        try!(self.create_service(cluster, &service_conf, &task_definition_arn))
      }
    };
    
    output::PrintLine::info("Starting to update the service");
    try!(self.update_service(cluster, &service_conf, &task_definition));
    output::PrintLine::info("Finished updating the service");

    try!(self.wait_for_green(&service_conf));

    output::PrintLine::success("Deployment completed");
    Ok(())
  }

  fn wait_for_green(&self, service_conf: &config::ecs::Service) -> Result<(), Box<error::Error>> {
    let cluster = &self.config.cluster;

    loop {
      let maybe_service = try!(self.describe_service(cluster, service_conf));
      let service = try!(maybe_service.ok_or(Box::new(CommandError::Unknown)));

      let maybe_primary = service.deployments.as_ref().and_then(|deployments| {
        deployments.iter().filter(|deployment| match deployment.status.as_ref() {
          Some(s) => s == "PRIMARY",
          _ => false
        }).nth(0)
      });

      if let Some(primary) = maybe_primary {
        if let (Some(desired_count), Some(running_count)) = (primary.desired_count, primary.running_count) {
          if desired_count == running_count {
            output::PrintLine::info(&format!("New tasks are now running. (desired_count:{}, running_count:{}", desired_count, running_count));
            break;
          } else {
            output::PrintLine::info(&format!("Waiting for new tasks to run... (desired_count:{}, running_count:{}", desired_count, running_count));
          }
        }
      }
      sleep(Duration::from_millis(2000));
    }

    Ok(())
  }
}

impl<'c> EcsExecuter for DeployExecuter<'c> {
  fn ecs_client(&self) -> &EcsClient<DefaultCredentialsProvider, hyper::client::Client> {
    &self.ecs_client
  }
}
