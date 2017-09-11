use std::error;
use std::default::Default;

use clap;
use rusoto_ssm;
use rusoto_ssm::Ssm;

use config;
use output;

use super::params::ParamsExecuter;

pub struct ParamsPutCommand<'c> {
    config: &'c config::command::Config,
    name: &'c str,
    value: &'c str,
}

impl<'c> ParamsPutCommand<'c> {
    pub fn from_args(config: &'c config::command::Config, args: &'c clap::ArgMatches<'c>) -> Self {
        debug!("ParamsPutCommand::from_args");
        ParamsPutCommand {
            config: config,
            name: args.value_of("NAME").unwrap(),
            value: args.value_of("VALUE").unwrap(),
        }
    }

    pub fn new(config: &'c config::command::Config, name: &'c str, value: &'c str) -> Self {
        debug!("ParamsPutCommand::new");
        ParamsPutCommand {
            config: config,
            name: name,
            value: value,
        }
    }

    pub fn run(&self) -> Result<(), Box<error::Error>> {
        debug!("ParamsPutCommand::run");
        if let Some(params_config) = self.config.params.as_ref() {

            let exec = ParamsPutExecuter::from_config(&params_config);

            try!(exec.run(self.name, self.value));
        }
        Ok(())
    }
}

pub struct ParamsPutExecuter<'c> {
    config: &'c config::command::ParamsConfig,
}

impl<'c> ParamsPutExecuter<'c> {
    pub fn from_config(config: &'c config::command::ParamsConfig) -> Self {
        debug!("ParamsPutExecuter::new");

        ParamsPutExecuter { config: config }
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

impl<'c> ParamsExecuter for ParamsPutExecuter<'c> {
    fn config(&self) -> &config::command::ParamsConfig { &self.config }
}
