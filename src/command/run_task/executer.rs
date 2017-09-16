use std::error;

use hyper;
use rusoto_core::{default_tls_client, DefaultCredentialsProvider, Region};
use rusoto_ecs::EcsClient;

use config;
use output;

use super::super::error::CommandError;
use command::ecs::Executer as EcsExecuter;

pub struct Executer<'c> {
    ecs_client: EcsClient<DefaultCredentialsProvider, hyper::client::Client>,
    config: &'c config::command::RunTaskConfig,
}

impl<'c> Executer<'c> {
    pub fn from_config(config: &'c config::command::RunTaskConfig) -> Self {
        debug!("RunTaskExecuter::from_config");

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
        debug!("RunTaskExecuter::run");

        output::PrintLine::info("Registering a task definition");
        let task_definition = try!(self.register_task_definition(&self.config.task_definition));
        let task_definition_arn = try!(task_definition.task_definition_arn.as_ref().ok_or(
            Box::new(
                CommandError::Unknown,
            ),
        ));

        output::PrintLine::info("Starting to run the task");
        try!(self.run_task(&self.config.cluster, &task_definition_arn));

        output::PrintLine::success("Finished running the task");
        Ok(())
    }
}

impl<'c> EcsExecuter for Executer<'c> {
    fn ecs_client(&self) -> &EcsClient<DefaultCredentialsProvider, hyper::client::Client> {
        &self.ecs_client
    }
}
