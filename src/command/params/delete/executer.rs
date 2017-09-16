use std::error;
use std::default::Default;

use rusoto_ssm;
use rusoto_ssm::Ssm;

use config;
use output;
use super::super::Executer as ParamsExecuter;

pub struct Executer<'c> {
    config: &'c config::command::ParamsConfig,
}

impl<'c> Executer<'c> {
    pub fn from_config(config: &'c config::command::ParamsConfig) -> Self {
        debug!("ParamsDeleteExecuter::new");

        Executer { config: config }
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

impl<'c> ParamsExecuter for Executer<'c> {
    fn config(&self) -> &config::command::ParamsConfig { &self.config }
}
