use std::error;
use std::process;

use crate::command::error::CommandError;
use crate::config;
use crate::output;

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

    pub async fn run(
        &self,
        program: &'c Program<'c>,
        args: &'c Arguments<'c>,
    ) -> Result<(), Box<dyn error::Error>> {
        trace!("command::params::exec::Executer::run");

        info!("exec: {} {}", program, args.join(" "));
        let params = self.params().await?;
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
        let mut child = cmd.spawn()?;
        let output = child.wait()?;

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
