use std::error;
use std::default::Default;

use clap;
use rusoto_ssm;
use rusoto_ssm::Ssm;

use config;
use output;

use super::params::ParamsExecuter;

pub struct ParamsDeleteCommand<'c> {
    config: &'c config::command::Config,
    name: &'c str,
}

impl<'c> ParamsDeleteCommand<'c> {
    pub fn from_args(config: &'c config::command::Config, args: &'c clap::ArgMatches<'c>) -> Self {
        debug!("ParamsDeleteCommand::from_args");

        ParamsDeleteCommand {
            config: config,
            name: args.value_of("NAME").unwrap(),
        }
    }

    pub fn new(config: &'c config::command::Config, name: &'c str) -> Self {
        debug!("ParamsDeleteCommand::new");

        ParamsDeleteCommand {
            config: config,
            name: name,
        }
    }

    pub fn run(&self) -> Result<(), Box<error::Error>> {
        debug!("ParamsDeleteCommand::run");

        if let Some(params_config) = self.config.params.as_ref() {
            let exec = ParamsDeleteExecuter::from_config(&params_config);

            try!(exec.run(self.name));
        }
        Ok(())
    }
}

pub struct ParamsDeleteExecuter<'c> {
    config: &'c config::command::ParamsConfig,
}

impl<'c> ParamsDeleteExecuter<'c> {
    pub fn from_config(config: &'c config::command::ParamsConfig) -> Self {
        debug!("ParamsDeleteExecuter::new");

        ParamsDeleteExecuter { config: config }
    }

    pub fn run(&self, name: &str) -> Result<(), Box<error::Error>> {
        debug!("ParamsDeleteExecuter::run");

        let req = rusoto_ssm::DeleteParameterRequest {
            name: self.name_with_path(name),
            ..Default::default()
        };

        let client = self.client();
        try!(client.delete_parameter(&req));

        output::PrintLine::success("Finished deleting the parameter");
        Ok(())
    }
}

impl<'c> ParamsExecuter for ParamsDeleteExecuter<'c> {
    fn config(&self) -> &config::command::ParamsConfig { &self.config }
}
