use std::error;
use std::process;

use command::error::CommandError;
use config;
use output;

use super::super::Executer as ParamsExecuter;
use super::{Arguments, Program};

pub struct Executer<'c> {
    config: &'c config::command::ParamsConfig,
}

impl<'c> Executer<'c> {
    pub fn from_config(config: &'c config::command::ParamsConfig) -> Self {
        trace!("command::params::exec::Executer::from_config");

        Executer { config: config }
    }

    pub fn run(
        &self,
        program: &'c Program<'c>,
        args: &'c Arguments<'c>,
    ) -> Result<(), Box<error::Error>> {
        trace!("command::params::exec::Executer::run");

        info!("exec: {} {}", program, args.join(" "));
        let params = try!(self.params());
        let mut cmd = process::Command::new(program);

        cmd.args(args);

        for param in params.iter() {
            if let (Some(name_with_path), Some(value)) = (param.name.as_ref(), param.value.as_ref())
            {
                if let Ok(name) = self.strip_path(name_with_path) {
                    cmd.env(name, value);
                }
            }
        }

        // TODO: Handle signals
        let mut child = try!(cmd.spawn());
        let output = try!(child.wait());

        if output.success() {
            Ok(())
        } else {
            output::PrintLine::error(&format!(
                "Command exit with status code: {}",
                output.code().unwrap_or(0)
            ));
            Err(Box::new(CommandError::Unknown))
        }
    }
}

impl<'c> ParamsExecuter for Executer<'c> {
    fn config(&self) -> &config::command::ParamsConfig {
        &self.config
    }
}
