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
        debug!("ParamsPutExecuter::new");

        Executer { config: config }
    }

    pub fn run(&self, name: &str, value: &str) -> Result<(), Box<error::Error>> {
        debug!("ParamsPutExecuter::run");

        let (type_, key_id) = if let Some(secure) = self.config.secure.as_ref() {
            (String::from("SecureString"), Some(secure.key.to_owned()))
        } else {
            (String::from("String"), None)
        };

        let req = rusoto_ssm::PutParameterRequest {
            name: self.name_with_path(name),
            value: value.to_owned(),
            type_: type_,
            key_id: key_id,
            overwrite: Some(true),
            ..Default::default()
        };

        let client = self.client();
        try!(client.put_parameter(&req));

        output::PrintLine::success("Finished put the parameter");
        Ok(())
    }
}

impl<'c> ParamsExecuter for Executer<'c> {
    fn config(&self) -> &config::command::ParamsConfig { &self.config }
}
