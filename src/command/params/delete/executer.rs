use std::default::Default;
use std::error;

use rusoto_ssm;
use rusoto_ssm::Ssm;

use super::super::Executer as ParamsExecuter;
use config;
use output;

pub struct Executer<'c> {
    config: &'c config::command::ParamsConfig,
}

impl<'c> Executer<'c> {
    pub fn from_config(config: &'c config::command::ParamsConfig) -> Self {
        trace!("command::params::delete::Executer::from_config");

        Executer { config: config }
    }

    pub fn run(&self, name: &str) -> Result<(), Box<dyn error::Error>> {
        trace!("command::params::delete::Executer::run");

        let req = rusoto_ssm::DeleteParameterRequest {
            name: self.name_with_path(name),
            ..Default::default()
        };

        let client = self.client();
        try!(client.delete_parameter(req).sync());

        output::PrintLine::success("Finished deleting the parameter");
        Ok(())
    }
}

impl<'c> ParamsExecuter for Executer<'c> {
    fn config(&self) -> &config::command::ParamsConfig {
        &self.config
    }
}
