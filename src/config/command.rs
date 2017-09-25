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
        file: &str,
        template_variable_map: Option<&BTreeMap<String, String>>,
        template_variable_file: Option<&str>,
    ) -> Result<Config, Box<error::Error>> {
        debug!("Config::from_file");

        let contents = try!(Self::load_file(&file));
        let tmpl_vars = try!(Self::load_template_variables(
            template_variable_map,
            template_variable_file
        ));

        Self::new(&contents, &tmpl_vars)
    }

    fn new(contents: &str, tmpl_vars: &serde_json::Value) -> Result<Config, Box<error::Error>> {
        let rendered_contents = try!(Self::apply_template_vars(contents, tmpl_vars));
        debug!("Config::from_file - Yaml file: {}", rendered_contents);

        match serde_yaml::from_str::<Config>(&rendered_contents) {
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

    fn load_file(file: &str) -> Result<String, Box<error::Error>> {
        let mut f = try!(File::open(file));
        let mut contents = String::new();

        try!(f.read_to_string(&mut contents));

        Ok(contents)
    }

    fn load_template_variables(
        template_variable_map: Option<&BTreeMap<String, String>>,
        template_variable_file: Option<&str>,
    ) -> Result<serde_json::Value, Box<error::Error>> {

        let mut vars = match template_variable_file {
            Some(tmpl_var_file) => {
                let mut var_file = try!(File::open(tmpl_var_file));
                let mut var_contents = String::new();

                try!(var_file.read_to_string(&mut var_contents));

                try!(serde_yaml::from_str::<serde_json::Value>(&var_contents))
            }
            None => json!({}),
        };

        if let Some(tmpl_vars) = template_variable_map {
            for (k, v) in tmpl_vars {
                let jv: serde_json::Value = v.as_str().into();
                vars[k] = jv;
            }
        }

        Ok(vars)
    }

    fn apply_template_vars(
        contents: &str,
        vars: &serde_json::Value,
    ) -> Result<String, Box<error::Error>> {
        let handlebars = Handlebars::new();
        let rendered = try!(handlebars.template_render(contents, vars));
        Ok(rendered)
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


#[test]
fn test_apply_template_vars() {
    let tmpl = "foo: {{ bar }}";
    let vars = json!({"bar": "baz"});
    let ret = Config::apply_template_vars(tmpl, &vars);
    assert!(match ret {
        Ok(rendered) => match rendered.as_ref() {
            "foo: baz" => true,
            _ => false
        },
        _ => false
    });
    //assert_eq!(ret.unwrap(), String::from("foo: baz"));
}

#[test]
fn test_service_config() {
    let tmpl = r"service:
  - name: test
    cluster: test-cluster
    service:
      name: test
      desired_count: 1
      task_definition:
        family: test
        container_definitions:
          - name: test
            image: 'test.dkr.com/racco/test:latest'
";
    let vars = json!({});

    let ret = Config::new(tmpl, &vars);
    assert!(match ret {
        Ok(config) => match config.service {
            Some(service_group) => service_group.len() == 1,
            _ => false
        },
        _ => false
    });
}

#[test]
fn test_run_task_config() {
    let tmpl = r"run_task:
  - name: test
    cluster: test-cluster
    task_definition:
      family: test
      container_definitions:
        - name: test
          image: 'test.dkr.com/racco/test:latest'
";
    let vars = json!({});

    let ret = Config::new(tmpl, &vars);
    assert!(match ret {
        Ok(config) => match config.run_task {
            Some(run_task_group) => run_task_group.len() == 1,
            _ => false
        },
        _ => false
    });
}

#[test]
fn test_schedule_task_config() {
    let tmpl = r"schedule_task:
  - name: test
    cluster: test-cluster
    task_definition:
      family: test
      container_definitions:
        - name: test
          image: 'test.dkr.com/racco/test:latest'
    rule:
      name: test-schedule-rule
      schedule_expression: 'cron(0/5 * * * ? *)'
";
    let vars = json!({});

    let ret = Config::new(tmpl, &vars);
    assert!(match ret {
        Ok(config) => match config.schedule_task {
            Some(schedule_task_group) => schedule_task_group.len() == 1,
            _ => false
        },
        _ => false
    });
}

#[test]
fn test_params_config() {
    let tmpl = r"params:
  path: test0/test1
  secure:
    key: 'xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx'
";
    let vars = json!({});

    let ret = Config::new(tmpl, &vars);
    assert!(match ret {
        Ok(config) => match config.params {
            Some(_params) => true,
            _ => false
        },
        _ => false
    });
}
