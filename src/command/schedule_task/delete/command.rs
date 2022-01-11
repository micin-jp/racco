use std::error;

use clap;
use config;

use super::executer::Executer;

#[allow(dead_code)]
pub struct Command<'c> {
    config: &'c config::command::Config,
    name: Option<&'c str>,
    all: bool,
}

impl<'c> Command<'c> {
    pub fn from_args(config: &'c config::command::Config, args: &'c clap::ArgMatches<'c>) -> Self {
        trace!("command::schedule_task::delete::Command::from_args");

        Command {
            config: config,
            name: args.value_of("NAME"),
            all: args.is_present("ALL"),
        }
    }

    pub fn new(config: &'c config::command::Config, name: Option<&'c str>, all: bool) -> Self {
        trace!("command::schedule_task::delete::Command::new");

        Command {
            config: config,
            name: name,
            all: all,
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn error::Error>> {
        trace!("command::schedule_task::delete::Command::run");

        if let Some(name) = self.name {
            let schedule_del_exec = Executer::new();
            r#try!(schedule_del_exec.run(name));
        } else if self.all {
            if let Some(schedule_config_group) = self.config.schedule_task.as_ref() {
                for schedule_config in schedule_config_group {
                    let schedule_del_exec = Executer::new();
                    r#try!(schedule_del_exec.run(schedule_config.rule.name.as_str()));
                }
            }
        }

        Ok(())
    }
}
