use std::error;

use clap;

use crate::config;

use super::executer::{Executer, ExecuterOptions};

pub struct Command<'c> {
    config: &'c config::command::Config,
    name: &'c str,
    no_wait: bool,
}

impl<'c> Command<'c> {
    pub fn from_args(config: &'c config::command::Config, args: &'c clap::ArgMatches<'c>) -> Self {
        trace!("command::run_task::Command::from_args");

        Command {
            config: config,
            name: args.value_of("NAME").unwrap(),
            no_wait: args.is_present("NO_WAIT"),
        }
    }

    pub fn new(config: &'c config::command::Config, name: &'c str, no_wait: bool) -> Self {
        trace!("command::run_task::Command::new");

        Command {
            config: config,
            name: name,
            no_wait: no_wait,
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn error::Error>> {
        trace!("command::run_task::Command::run");

        if let Some(run_task_config_group) = self.config.run_task.as_ref() {
            for run_task_config in run_task_config_group {
                if run_task_config.name != self.name {
                    continue;
                }

                let options = ExecuterOptions {
                    no_wait: self.no_wait,
                };
                let ecs_run_task_cmd = Executer::from_config(&run_task_config, &options);
                r#try!(ecs_run_task_cmd.run());
            }
        }

        Ok(())
    }
}
