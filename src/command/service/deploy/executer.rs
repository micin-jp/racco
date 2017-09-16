use std::error;
use std::time::Duration;
use std::thread::sleep;

use hyper;
use rusoto_core::{default_tls_client, DefaultCredentialsProvider, Region};
use rusoto_ecs;
use rusoto_ecs::EcsClient;

use config;
use output;

use command::error::CommandError;
use command::ecs::Executer as EcsExecuter;

pub struct Executer<'c> {
    ecs_client: EcsClient<DefaultCredentialsProvider, hyper::client::Client>,
    config: &'c config::command::DeployConfig,
}

impl<'c> Executer<'c> {
    pub fn from_config(config: &'c config::command::DeployConfig) -> Self {
        trace!("command::service::deploy::Executer::from_config");

        let credentials = DefaultCredentialsProvider::new().unwrap();
        let client = EcsClient::new(
            default_tls_client().unwrap(),
            credentials,
            Region::ApNortheast1,
        );
        Executer {
            ecs_client: client,
            config: config,
        }
    }

    pub fn run(&self) -> Result<(), Box<error::Error>> {
        trace!("command::service::deploy::Executer::run");

        let service_conf = &self.config.service;
        let cluster = &self.config.cluster;

        let maybe_latest_task_definition = try!(self.describe_latest_task_definition(
            &service_conf.task_definition.family,
        ));

        let task_definition = if let Some(latest_task_definition) = maybe_latest_task_definition {
            if self.detect_task_definition_changes(
                &service_conf.task_definition,
                &latest_task_definition,
            )
            {
                output::PrintLine::info("Registering a task definition");
                try!(self.register_task_definition(&service_conf.task_definition))
            } else {
                latest_task_definition
            }
        } else {
            output::PrintLine::info("Registering a task definition");
            try!(self.register_task_definition(&service_conf.task_definition))
        };

        let task_definition_arn = try!(task_definition.task_definition_arn.as_ref().ok_or(
            Box::new(
                CommandError::Unknown,
            ),
        ));

        let maybe_service = try!(self.describe_service(cluster, &service_conf));

        let _service: rusoto_ecs::Service = match maybe_service {
            Some(s) => s,
            None => {
                output::PrintLine::info("Service has not been exist. Creating...");
                try!(self.create_service(
                    cluster,
                    &service_conf,
                    &task_definition_arn,
                ))
            }
        };

        output::PrintLine::info("Starting to update the service");
        try!(self.update_service(
            cluster,
            &service_conf,
            &task_definition,
        ));
        output::PrintLine::info("Finished updating the service");

        try!(self.wait_for_green(&service_conf));

        output::PrintLine::success("Deployment completed");
        Ok(())
    }

    fn wait_for_green(&self, service_conf: &config::ecs::Service) -> Result<(), Box<error::Error>> {
        trace!("command::service::deploy::Executer::wait_for_green");
        let cluster = &self.config.cluster;

        // TODO: Timeout
        loop {
            let maybe_service = try!(self.describe_service(cluster, service_conf));
            let service = try!(maybe_service.ok_or(Box::new(CommandError::Unknown)));

            let maybe_primary = service.deployments.as_ref().and_then(|deployments| {
                deployments
                    .iter()
                    .filter(|deployment| match deployment.status.as_ref() {
                        Some(s) => s == "PRIMARY",
                        _ => false,
                    })
                    .nth(0)
            });

            if let Some(primary) = maybe_primary {
                if let (Some(desired_count), Some(running_count)) =
                    (primary.desired_count, primary.running_count)
                {
                    if desired_count == running_count {
                        output::PrintLine::info(&format!(
                            "New tasks are now running. (desired_count:{}, running_count:{}",
                            desired_count,
                            running_count
                        ));
                        break;
                    } else {
                        output::PrintLine::info(&format!(
                            "Waiting for new tasks to run... (desired_count:{}, running_count:{}",
                            desired_count,
                            running_count
                        ));
                    }
                }
            }
            sleep(Duration::from_millis(2000));
        }

        Ok(())
    }
}

impl<'c> EcsExecuter for Executer<'c> {
    fn ecs_client(&self) -> &EcsClient<DefaultCredentialsProvider, hyper::client::Client> {
        &self.ecs_client
    }
}
