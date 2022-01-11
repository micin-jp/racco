use std::error;

use clap;
use config;

use super::executer::{Executer, ExecuterOptions};

pub struct Command<'c> {
    config: &'c config::command::Config,
    name: Option<&'c str>,
    no_wait: bool,
    all: bool,
}

impl<'c> Command<'c> {
    pub fn from_args(config: &'c config::command::Config, args: &'c clap::ArgMatches<'c>) -> Self {
        trace!("command::service::deploy::Command::from_args");

        Command {
            config: config,
            name: args.value_of("NAME"),
            no_wait: args.is_present("NO_WAIT"),
            all: args.is_present("ALL"),
        }
    }

    pub fn new(
        config: &'c config::command::Config,
        name: Option<&'c str>,
        no_wait: bool,
        all: bool,
    ) -> Self {
        trace!("command::service::deploy::Command::new");

        Command {
            config: config,
            name: name,
            no_wait: no_wait,
            all: all,
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn error::Error>> {
        trace!("command::service::deploy::Command::run");

        if let Some(service_config_group) = self.config.service.as_ref() {
            for service_config in service_config_group {
                let mut runnable: bool = false;
                if let Some(name) = self.name {
                    if name == service_config.name {
                        runnable = true;
                    }
                }
                if self.all {
                    runnable = true;
                }
                if !runnable {
                    continue;
                }

                let options = ExecuterOptions {
                    no_wait: self.no_wait,
                };
                let ecs_deploy_cmd = Executer::from_config(&service_config, &options);
                r#try!(ecs_deploy_cmd.run());
            }
        }

        Ok(())
    }
}
