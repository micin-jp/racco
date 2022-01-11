use std::error;
use std::thread::sleep;
use std::time::Duration;

use rusoto_core::Region;
use rusoto_ecs;
use rusoto_ecs::EcsClient;

use crate::command::ecs::Executer as EcsExecuter;
use crate::command::error::CommandError;
use crate::config;
use crate::output;

pub struct ExecuterOptions {
    pub no_wait: bool,
}

pub struct Executer<'c> {
    ecs_client: EcsClient,
    config: &'c config::command::ServiceConfig,
    options: &'c ExecuterOptions,
}

impl<'c> Executer<'c> {
    pub fn from_config(
        config: &'c config::command::ServiceConfig,
        options: &'c ExecuterOptions,
    ) -> Self {
        trace!("command::service::deploy::Executer::from_config");

        let client = EcsClient::new(Region::ApNortheast1);
        Executer {
            ecs_client: client,
            config: config,
            options: options,
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn error::Error>> {
        trace!("command::service::deploy::Executer::run");

        let service_conf = &self.config.service;
        let cluster = &self.config.cluster;

        let maybe_latest_task_definition =
            self.describe_latest_task_definition(&service_conf.task_definition.family)?;

        let task_definition = if let Some(latest_task_definition) = maybe_latest_task_definition {
            if self.detect_task_definition_changes(
                &service_conf.task_definition,
                &latest_task_definition,
            ) {
                output::PrintLine::info("Registering a task definition");
                self.register_task_definition(&service_conf.task_definition)?
            } else {
                latest_task_definition
            }
        } else {
            output::PrintLine::info("Registering a task definition");
            self.register_task_definition(&service_conf.task_definition)?
        };

        let task_definition_arn = task_definition
            .task_definition_arn
            .as_ref()
            .ok_or(Box::new(CommandError::Unknown))?;

        let maybe_service = self.describe_service(cluster, &service_conf)?;

        let _service: rusoto_ecs::Service = match maybe_service {
            Some(s) => s,
            None => {
                output::PrintLine::info("Service has not been exist. Creating...");
                self.create_service(cluster, &service_conf, &task_definition_arn)?
            }
        };

        output::PrintLine::info("Starting to update the service");
        self.update_service(cluster, &service_conf, &task_definition)?;
        output::PrintLine::info("Finished updating the service");

        if !self.options.no_wait {
            self.wait_for_green(&service_conf)?;
        }

        output::PrintLine::success("Deployment completed");
        Ok(())
    }

    fn wait_for_green(
        &self,
        service_conf: &config::ecs::Service,
    ) -> Result<(), Box<dyn error::Error>> {
        trace!("command::service::deploy::Executer::wait_for_green");
        let cluster = &self.config.cluster;

        // TODO: Timeout
        loop {
            let maybe_service = self.describe_service(cluster, service_conf)?;
            let service = maybe_service.ok_or(Box::new(CommandError::Unknown))?;

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
                            desired_count, running_count
                        ));
                        break;
                    } else {
                        output::PrintLine::info(&format!(
                            "Waiting for new tasks to run... (desired_count:{}, running_count:{})",
                            desired_count, running_count
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
    fn ecs_client(&self) -> &EcsClient {
        &self.ecs_client
    }
}
