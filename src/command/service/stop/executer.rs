use std::error;

use rusoto_core::Region;
use rusoto_ecs::EcsClient;

use config;
use output;

use command::ecs::Executer as EcsExecuter;
use command::error::CommandError;

pub struct ExecuterOptions {
    pub no_wait: bool,
}

#[allow(dead_code)]
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
        trace!("command::service::stop::Executer::from_config");

        let client = EcsClient::new(Region::ApNortheast1);
        Executer {
            ecs_client: client,
            config: config,
            options: options,
        }
    }

    pub fn run(&self) -> Result<(), Box<error::Error>> {
        trace!("command::service::stop::Executer::run");

        let service_conf = &self.config.service;
        let cluster = &self.config.cluster;

        let maybe_service = try!(self.describe_service(cluster, &service_conf));

        if maybe_service.is_none() {
            output::PrintLine::info("Service has not been exist.");
            return Ok(());
        }
        let maybe_task_definition =
            try!(self.describe_latest_task_definition(&service_conf.task_definition.family,));
        if maybe_task_definition.is_none() {
            output::PrintLine::error("Could not find task_definition");
            return Err(Box::new(CommandError::Unknown));
        }
        let task_definition = maybe_task_definition.unwrap();

        let zero_task_service = config::ecs::Service {
            name: service_conf.name.to_owned(),
            desired_count: 0,
            deployment_configuration: service_conf.deployment_configuration.to_owned(),
            load_balancers: service_conf.load_balancers.to_owned(),
            task_definition: service_conf.task_definition.to_owned(),
            role: service_conf.role.to_owned(),
            launch_type: service_conf.launch_type.to_owned(),
            network_configuration: service_conf.network_configuration.to_owned(),
            service_registries: service_conf.service_registries.to_owned(),
            platform_version: service_conf.platform_version.to_owned(),
        };

        output::PrintLine::info("Starting to update the service");
        try!(self.update_service(cluster, &zero_task_service, &task_definition,));
        output::PrintLine::info("Finished updating the service");

        // if !self.options.no_wait {
        // }

        output::PrintLine::success("The service stopped");
        Ok(())
    }
}

impl<'c> EcsExecuter for Executer<'c> {
    fn ecs_client(&self) -> &EcsClient {
        &self.ecs_client
    }
}
