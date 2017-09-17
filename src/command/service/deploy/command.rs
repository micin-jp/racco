use std::error;

use clap;
use config;

use super::executer::{Executer, ExecuterOptions};

pub struct Command<'c> {
    config: &'c config::command::Config,
    name: &'c str,
    no_wait: bool,
}

impl<'c> Command<'c> {
    pub fn from_args(config: &'c config::command::Config, args: &'c clap::ArgMatches<'c>) -> Self {
        trace!("command::service::deploy::Command::from_args");

        Command {
            config: config,
            name: args.value_of("NAME").unwrap(),
            no_wait: args.is_present("NO_WAIT"),
        }
    }

    pub fn new(config: &'c config::command::Config, name: &'c str, no_wait: bool) -> Self {
        trace!("command::service::deploy::Command::new");

        Command {
            config: config,
            name: name,
            no_wait: no_wait,
        }
    }

    pub fn run(&self) -> Result<(), Box<error::Error>> {
        trace!("command::service::deploy::Command::run");

        if let Some(deploy_config_group) = self.config.deploy.as_ref() {
            for deploy_config in deploy_config_group {
                if self.name != deploy_config.name {
                    continue;
                }

                let options = ExecuterOptions {
                    no_wait: self.no_wait,
                };
                let ecs_deploy_cmd = Executer::from_config(&deploy_config, &options);
                try!(ecs_deploy_cmd.run());
            }
        }

        Ok(())
    }
}
