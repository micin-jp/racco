use std::collections::BTreeMap;
use std::fs::File;
use std::error;
use std::fmt;
use std::io::prelude::*;

use handlebars::Handlebars;
use serde_yaml;
use serde_json;

use super::ecs;
use super::cloudwatch_events;

#[derive(Debug)]
pub enum ConfigError {
    ParseError(serde_yaml::Error),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfigError::ParseError(ref yaml_err) => write!(f, "Parse Error: {}", yaml_err),
        }
    }
}

impl error::Error for ConfigError {
    fn description(&self) -> &str {
        match *self {
            ConfigError::ParseError(ref yaml_err) => yaml_err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            ConfigError::ParseError(ref yaml_err) => Some(yaml_err),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub service: Option<ServiceConfigGroup>,
    pub run_task: Option<RunTaskConfigGroup>,
    pub schedule_task: Option<ScheduleTaskConfigGroup>,
    pub params: Option<ParamsConfig>,
}

impl Config {
    pub fn from_file(
        file_name: &str,
        template_variables: Option<&BTreeMap<String, String>>,
        template_variable_file: Option<&str>,
    ) -> Result<Config, Box<error::Error>> {
        debug!("Config::from_file");

        let mut file = try!(File::open(file_name));
        let mut contents = String::new();

        try!(file.read_to_string(&mut contents));

        if let Some(tmpl_var_file) = template_variable_file {
            contents = try!(Config::apply_var_file(contents, tmpl_var_file));
        }

        if let Some(tmpl_vars) = template_variables {
            contents = try!(Config::apply_vars(contents, tmpl_vars));
        }

        debug!("Config::from_file - Yaml file: {}", contents);

        match serde_yaml::from_str::<Config>(&contents) {
            Ok(c) => {
                debug!(
                    "Config::from_file - Serialize reversely: {}",
                    serde_yaml::to_string(&c).unwrap()
                );
                Ok(c)
            }
            Err(e) => Err(Box::new(ConfigError::ParseError(e))),
        }
    }

    fn apply_vars(
        mut contents: String,
        template_variables: &BTreeMap<String, String>,
    ) -> Result<String, Box<error::Error>> {
        let handlebars = Handlebars::new();
        contents = try!(handlebars.template_render(&contents, template_variables));
        Ok(contents)
    }

    fn apply_var_file(
        mut contents: String,
        template_variable_file: &str,
    ) -> Result<String, Box<error::Error>> {
        let mut var_file = try!(File::open(template_variable_file));
        let mut var_contents = String::new();

        try!(var_file.read_to_string(&mut var_contents));

        let vars = try!(serde_yaml::from_str::<serde_json::Value>(&var_contents));
        let handlebars = Handlebars::new();
        contents = try!(handlebars.template_render(&contents, &vars));

        Ok(contents)
    }
}

pub type ServiceConfigGroup = Vec<ServiceConfig>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub name: String,
    pub cluster: String,
    pub service: ecs::Service,
}

pub type RunTaskConfigGroup = Vec<RunTaskConfig>;

#[derive(Debug, Serialize, Deserialize)]
pub struct RunTaskConfig {
    pub name: String,
    pub cluster: String,
    pub task_definition: ecs::TaskDefinition,
}

pub type ScheduleTaskConfigGroup = Vec<ScheduleTaskConfig>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduleTaskConfig {
    pub name: String,
    pub cluster: String,
    pub task_definition: ecs::TaskDefinition,
    pub rule: cloudwatch_events::ScheduleRule,
    pub rule_targets_role_arn: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParamsConfig {
    pub path: String,
    pub secure: Option<ParamsSecure>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParamsSecure {
    pub key: String,
}
