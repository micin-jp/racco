use std::error;

use clap;
use config;

use super::executer::Executer;

pub struct Command<'c> {
    config: &'c config::command::Config,
    name: Option<&'c str>,
}

impl<'c> Command<'c> {
    pub fn from_args(config: &'c config::command::Config, args: &'c clap::ArgMatches<'c>) -> Self {
        debug!("DeployCommand::from_args");

        Command {
            config: config,
            name: args.value_of("NAME"),
        }
    }

    pub fn new(config: &'c config::command::Config, name: Option<&'c str>) -> Self {
        debug!("DeployCommand::new");

        Command {
            config: config,
            name: name,
        }
    }

    pub fn run(&self) -> Result<(), Box<error::Error>> {
        debug!("DeployCommand::run");

        if let Some(deploy_config_group) = self.config.deploy.as_ref() {
            for deploy_config in deploy_config_group {
                if let Some(name) = self.name {
                    if name != deploy_config.name {
                        continue;
                    }
                }

                let ecs_deploy_cmd = Executer::from_config(&deploy_config);
                try!(ecs_deploy_cmd.run());
            }
        }

        Ok(())
    }
}
