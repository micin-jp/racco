use std::error;
use std::default::Default;

use hyper;
use rusoto_core::{default_tls_client, DefaultCredentialsProvider, Region};
use rusoto_ecs;
use rusoto_ecs::{
  Ecs, EcsClient };

use config;
use super::error::CommandError;


pub trait EcsExecuter {

  fn client(&self) -> &EcsClient<DefaultCredentialsProvider, hyper::client::Client>;

  fn describe_latest_task_definition(&self, family: &str) -> Result<Option<rusoto_ecs::TaskDefinition>, Box<error::Error>> {
    debug!("EcsCommand::describe_latest_task_definition");

    let req = rusoto_ecs::DescribeTaskDefinitionRequest {
      task_definition: family.to_owned()
    };
    

    match self.client().describe_task_definition(&req) {
      Ok(res) => {
        info!("Completed to describe task_definition successfully");
        Ok(res.task_definition)
      }
      Err(rusoto_ecs::DescribeTaskDefinitionError::Client(s)) => {
        info!("Not found the task-definition: {}", family);
        Ok(None)
      }
      Err(e) => {
        Err(Box::new(e))
      }
    }
  }

  fn register_task_definition(&self, task_definition_conf: &config::ecs::TaskDefinition) -> Result<rusoto_ecs::TaskDefinition, Box<error::Error>> {
    debug!("EcsCommand::register_task_definition");
    let req = rusoto_ecs::RegisterTaskDefinitionRequest {
      family: task_definition_conf.family.to_owned(),
      task_role_arn: task_definition_conf.task_role_arn.to_owned(),
      network_mode: task_definition_conf.network_mode.to_owned(),
      container_definitions: task_definition_conf.container_definitions.iter().map(|cd| cd.to_rusoto()).collect(),
      ..Default::default()
    };

    let res = try!(self.client().register_task_definition(&req));
    info!("Completed to register task_definition successfully");

    res.task_definition.ok_or(Box::new(CommandError::Unknown))
  }

  fn create_service(&self, cluster: &str, service_conf: &config::ecs::Service, task_definition: &str) -> Result<rusoto_ecs::Service, Box<error::Error>> {
    debug!("EcsCommand::register_task_definition");
    
    let req = rusoto_ecs::CreateServiceRequest {
      cluster: Some(cluster.to_owned()),
      service_name: service_conf.name.to_owned(),
      desired_count: service_conf.desired_count,
      //deployment_configuration: service_conf.deployment_configuration.as_ref().map(|d| d.to_rusoto()),
      load_balancers: service_conf.load_balancers.as_ref().map(|lbs| lbs.iter().map(|lb| lb.to_rusoto()).collect()),
      role: service_conf.role.to_owned(),
      task_definition: task_definition.to_owned(),
      ..Default::default()
    };

    let res = try!(self.client().create_service(&req));
    info!("Completed to create service successfully");

    res.service.ok_or(Box::new(CommandError::Unknown))
  }

  fn describe_service(&self, cluster: &str, service_conf: &config::ecs::Service) -> Result<Option<rusoto_ecs::Service>, Box<error::Error>> {
    debug!("EcsCommand::describe_service");

    let req = rusoto_ecs::DescribeServicesRequest {
      cluster: Some(cluster.to_owned()),
      services: vec![service_conf.name.to_owned()]
    };

    let res = try!(self.client().describe_services(&req));
    info!("Completed to describe services successfully");

    match res.services {
      Some(services) => {
        let actives = services.iter().filter(|service| service.status.is_some() && service.status.as_ref().unwrap() == "ACTIVE").collect::<Vec<&rusoto_ecs::Service>>();
        Ok(actives.first().cloned().cloned())
      },
      _ => Err(Box::new(CommandError::Unknown))
    }
  }

  fn update_service(&self, cluster: &str, service_conf: &config::ecs::Service, service: &rusoto_ecs::Service, task_definition: &rusoto_ecs::TaskDefinition) -> Result<rusoto_ecs::Service, Box<error::Error>> {
    debug!("EcsCommand::update_service");

    if task_definition.task_definition_arn.is_none() {
      return Err(Box::new(CommandError::Unknown))
    }

    let req = rusoto_ecs::UpdateServiceRequest {
      service: service_conf.name.to_owned(),
      cluster: Some(cluster.to_owned()),
      desired_count: Some(service_conf.desired_count),
      deployment_configuration: service_conf.deployment_configuration.as_ref().map(|d| d.to_rusoto()),
      task_definition: task_definition.task_definition_arn.to_owned(),
      ..Default::default()
    };

    let res = try!(self.client().update_service(&req));
    info!("Completed to update service successfully");

    let service = res.service.map(|s| { s.to_owned() });
    service.ok_or(Box::new(CommandError::Unknown))
  }

  fn run_task(&self, cluster: &str, task_definition_arn: &str) -> Result<(), Box<error::Error>> {

    let req = rusoto_ecs::RunTaskRequest {
      cluster: Some(cluster.to_owned()),
      task_definition: task_definition_arn.to_owned(),
      ..Default::default()
    };

    let res = try!(self.client().run_task(&req));
    info!("Completed to run task successfully");

    Ok(())
  }

  fn detect_task_definition_changes(&self, task_definition_conf: &config::ecs::TaskDefinition, current_task_definitions: &rusoto_ecs::TaskDefinition) -> bool {
    if current_task_definitions.family.is_none() || current_task_definitions.family.as_ref().unwrap().as_str() != task_definition_conf.family.as_str() {
      return true
    }
    // if current_task_definitions.task_role_arn.is_none() || current_task_definitions.task_role_arn.as_ref().unwrap().as_str() != task_definition_conf.task_role_arn.as_str() {
    //   return true
    // }
    // if current_task_definitions.network_mode.is_none() || current_task_definitions.network_mode.as_ref().unwrap().as_str() != task_definition_conf.network_mode.as_str() {
    //   return true
    // }

    // TODO: detect difference between config of container definition and current one
    true
    
    //false
  }

}
