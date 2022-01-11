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
        trace!("command::params::get::Executer::from_config");

        Executer { config: config }
    }

    pub fn run(&self, name: &str) -> Result<(), Box<dyn error::Error>> {
        trace!("command::params::get::Executer::run");

        let name_with_path = self.name_with_path(name);
        let with_decription = self.config.secure.is_some();

        let req = rusoto_ssm::GetParameterRequest {
            name: name_with_path,
            with_decryption: Some(with_decription),
            ..Default::default()
        };

        let client = self.client();
        let res = r#try!(client.get_parameter(req).sync());

        if let Some(params) = res.parameter {
            self.print(&params);
        }

        Ok(())
    }

    fn print(&self, param: &rusoto_ssm::Parameter) {
        if let Some(val) = param.value.as_ref() {
            output::PrintLine::print(val);
        }
    }
}

impl<'c> ParamsExecuter for Executer<'c> {
    fn config(&self) -> &config::command::ParamsConfig {
        &self.config
    }
}
