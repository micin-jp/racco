use std::error;
use std::io::stdout;
use std::io::Write;
use tabwriter::TabWriter;

use rusoto_ssm;

use super::super::Executer as ParamsExecuter;
use crate::config;

pub struct Executer<'c> {
    config: &'c config::command::ParamsConfig,
}

impl<'c> Executer<'c> {
    pub fn from_config(config: &'c config::command::ParamsConfig) -> Self {
        trace!("command::params::list::Executer::from_config");

        Executer { config: config }
    }

    pub fn run(&self) -> Result<(), Box<dyn error::Error>> {
        trace!("command::params::list::Executer::run");

        let params = self.params()?;
        self.print(&params)?;

        Ok(())
    }

    fn print(&self, params: &Vec<rusoto_ssm::Parameter>) -> Result<(), Box<dyn error::Error>> {
        let mut tw = TabWriter::new(stdout());

        for p in params.iter() {
            if let (Some(name_with_path), Some(value)) = (p.name.as_ref(), p.value.as_ref()) {
                let name = self.strip_path(name_with_path)?;
                write!(&mut tw, "{}\t{}\n", name, value)?;
            }
        }

        tw.flush()?;
        Ok(())
    }
}

impl<'c> ParamsExecuter for Executer<'c> {
    fn config(&self) -> &config::command::ParamsConfig {
        &self.config
    }
}
