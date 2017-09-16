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
        debug!("ParamsGetExecuter::new");

        Executer { config: config }
    }

    pub fn run(&self, name: &str) -> Result<(), Box<error::Error>> {
        debug!("ParamsGetExecuter::run");

        let name_with_path = self.name_with_path(name);
        let with_decription = self.config.secure.is_some();

        let req = rusoto_ssm::GetParameterRequest {
            name: name_with_path,
            with_decryption: Some(with_decription),
            ..Default::default()
        };

        let client = self.client();
        let res = try!(client.get_parameter(&req));

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
    fn config(&self) -> &config::command::ParamsConfig { &self.config }
}
