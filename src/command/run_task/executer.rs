use std::error;
use std::thread::sleep;
use std::time::Duration;

use rusoto_core::Region;
use rusoto_ecs::EcsClient;

use config;
use output;

use super::super::error::CommandError;
use command::ecs::Executer as EcsExecuter;
use command::ecs::TaskDescription;

pub struct ExecuterOptions {
    pub no_wait: bool,
}

pub struct Executer<'c> {
    ecs_client: EcsClient,
    config: &'c config::command::RunTaskConfig,
    options: &'c ExecuterOptions,
}

impl<'c> Executer<'c> {
    pub fn from_config(
        config: &'c config::command::RunTaskConfig,
        options: &'c ExecuterOptions,
    ) -> Self {
        trace!("command::run_task::Executer::from_config");

        let client = EcsClient::new(Region::ApNortheast1);
        Executer {
            ecs_client: client,
            config: config,
            options: options,
        }
    }

    pub fn run(&self) -> Result<(), Box<error::Error>> {
        trace!("command::run_task::Executer::run");

        output::PrintLine::info("Registering a task definition");
        let task_definition = try!(self.register_task_definition(&self.config.task_definition));
        let task_definition_arn = try!(
            task_definition
                .task_definition_arn
                .as_ref()
                .ok_or(Box::new(CommandError::Unknown,),)
        );

        output::PrintLine::info("Starting to run the task");
        let running_task = try!(self.run_task(&self.config.cluster, &task_definition_arn));

        if !self.options.no_wait {
            try!(self.wait_for_stopped(&running_task));
        }

        output::PrintLine::success("Finished running the task");
        Ok(())
    }

    fn wait_for_stopped(&self, running_task: &TaskDescription) -> Result<(), Box<error::Error>> {
        trace!("command::run-task::Executer::wait_for_stopped");

        fn check_stopped(current_task: &TaskDescription) -> Result<bool, Box<error::Error>> {
            if let Some(failure) = current_task.failure.as_ref() {
                let reason = failure.reason.as_ref().map(String::as_str).unwrap_or("");
                output::PrintLine::error(&format!("Finished task with error :{}", reason));
                return Err(Box::new(CommandError::Unknown));
            }

            match current_task.task.as_ref() {
                None => {
                    output::PrintLine::error("No task found");
                    return Err(Box::new(CommandError::Unknown));
                }
                Some(task) => {
                    let status = try!(
                        task.last_status
                            .as_ref()
                            .ok_or(Box::new(CommandError::Unknown))
                    );
                    if status == "STOPPED" {
                        if let Some(reason) = task.stopped_reason.as_ref() {
                            if reason != "Essential container in task exited" {
                                output::PrintLine::error(&format!(
                                    "The task stopped with reason: {}",
                                    reason
                                ));
                                return Err(Box::new(CommandError::Unknown));
                            }
                        }

                        let essential_container = try!(
                            task.containers
                                .as_ref()
                                .and_then(|c| c.first())
                                .ok_or(Box::new(CommandError::Unknown))
                        );

                        match essential_container.exit_code {
                            Some(0) => return Ok(true), // stopped task successfully!
                            Some(code) => {
                                output::PrintLine::error(&format!(
                                    "The container in the task exited with code: {}",
                                    code
                                ));
                                return Err(Box::new(CommandError::Unknown));
                            }
                            None => {
                                let reason = essential_container
                                    .reason
                                    .as_ref()
                                    .map(String::as_str)
                                    .unwrap_or("");
                                output::PrintLine::error(&format!(
                                    "Failed running task by some reason: {}",
                                    reason
                                ));
                                return Err(Box::new(CommandError::Unknown));
                            }
                        }
                    }
                }
            };

            // running task yet
            Ok(false)
        }

        let stopped = try!(check_stopped(running_task));
        if stopped {
            return Ok(());
        }

        let task_arn = &running_task
            .task
            .as_ref()
            .unwrap()
            .task_arn
            .as_ref()
            .unwrap();

        // TODO: Timeout
        loop {
            output::PrintLine::info("Waiting for the task to be stopped...");
            sleep(Duration::from_millis(2000));

            let current_task = try!(self.describe_task(&self.config.cluster, task_arn));
            let stopped = try!(check_stopped(&current_task));
            if stopped {
                return Ok(());
            }
        }

        // Ok(())
    }
}

impl<'c> EcsExecuter for Executer<'c> {
    fn ecs_client(&self) -> &EcsClient {
        &self.ecs_client
    }
}
