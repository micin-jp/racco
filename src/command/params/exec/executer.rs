use std::error;
use std::process;

use config;

use super::{Program, Arguments};
use super::super::Executer as ParamsExecuter;

pub struct Executer<'c> {
    config: &'c config::command::ParamsConfig,
}

impl<'c> Executer<'c> {
    pub fn from_config(config: &'c config::command::ParamsConfig) -> Self {
        debug!("ParamsGetExecuter::new");

        Executer { config: config }
    }

    pub fn run(
        &self,
        program: &'c Program<'c>,
        args: &'c Arguments<'c>,
    ) -> Result<(), Box<error::Error>> {
        debug!("ParamsExecExecuter::run");

        info!("exec: {} {}", program, args.join(" "));
        let maybe_params = try!(self.params());
        let mut cmd = process::Command::new(program);

        cmd.args(args);

        if let Some(params) = maybe_params {
            for param in params.iter() {
                if let (Some(name_with_path), Some(value)) =
                    (param.name.as_ref(), param.value.as_ref())
                {
                    if let Ok(name) = self.strip_path(name_with_path) {
                        cmd.env(name, value);
                    }
                }
            }
        }

        // TODO: Handle signals
        let mut child = try!(cmd.spawn());
        let _output = try!(child.wait());

        Ok(())
    }
}

impl<'c> ParamsExecuter for Executer<'c> {
    fn config(&self) -> &config::command::ParamsConfig { &self.config }
}
