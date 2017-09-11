use std::error;

use clap;
use rusoto_ssm;

use config;

use std::io::stdout;
use std::io::Write;
use tabwriter::TabWriter;


use super::params::ParamsExecuter;

pub struct ParamsListCommand<'c> {
    config: &'c config::command::Config,
}

impl<'c> ParamsListCommand<'c> {
    pub fn from_args(config: &'c config::command::Config, args: &'c clap::ArgMatches<'c>) -> Self {
        debug!("ParamsListCommand::from_args");

        ParamsListCommand { config: config }
    }

    pub fn new(config: &'c config::command::Config) -> Self {
        debug!("ParamsListCommand::new");

        ParamsListCommand { config: config }
    }

    pub fn run(&self) -> Result<(), Box<error::Error>> {
        debug!("ParamsListCommand::run");
        if let Some(params_config) = self.config.params.as_ref() {

            let exec = ParamsListExecuter::from_config(params_config);
            try!(exec.run());
        }
        Ok(())
    }
}

pub struct ParamsListExecuter<'c> {
    config: &'c config::command::ParamsConfig,
}

impl<'c> ParamsListExecuter<'c> {
    pub fn from_config(config: &'c config::command::ParamsConfig) -> Self {
        debug!("ParamsListExecuter::new");

        ParamsListExecuter { config: config }
    }

    pub fn run(&self) -> Result<(), Box<error::Error>> {
        debug!("ParamsListExecuter::run");

        let maybe_params = try!(self.params());

        if let Some(params) = maybe_params {
            try!(self.print(&params));
        }

        Ok(())
    }

    fn print(&self, params: &Vec<rusoto_ssm::Parameter>) -> Result<(), Box<error::Error>> {
        let mut tw = TabWriter::new(stdout());

        for p in params.iter() {
            if let (Some(name_with_path), Some(value)) = (p.name.as_ref(), p.value.as_ref()) {
                let name = try!(self.strip_path(name_with_path));
                try!(write!(&mut tw, "{}\t{}\n", name, value));
            }
        }

        try!(tw.flush());
        Ok(())
    }
}

impl<'c> ParamsExecuter for ParamsListExecuter<'c> {
    fn config(&self) -> &config::command::ParamsConfig { &self.config }
}
