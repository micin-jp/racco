use std::error;

use clap;
use config;

use super::executer::Executer;

pub struct Command<'c> {
    config: &'c config::command::Config,
    name: &'c str,
}

impl<'c> Command<'c> {
    pub fn from_args(config: &'c config::command::Config, args: &'c clap::ArgMatches<'c>) -> Self {
        debug!("RunTaskCommand::from_args");

        Command {
            config: config,
            name: args.value_of("NAME").unwrap(),
        }
    }

    pub fn new(config: &'c config::command::Config, name: &'c str) -> Self {
        debug!("RunTaskCommand::new");

        Command {
            config: config,
            name: name,
        }
    }

    pub fn run(&self) -> Result<(), Box<error::Error>> {
        debug!("RunTaskCommand::run");

        if let Some(run_task_config_group) = self.config.run_task.as_ref() {
            for run_task_config in run_task_config_group {
                if run_task_config.name != self.name {
                    continue;
                }

                let ecs_run_task_cmd = Executer::from_config(&run_task_config);
                try!(ecs_run_task_cmd.run());
            }
        }

        Ok(())
    }
}
