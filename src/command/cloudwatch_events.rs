use std::error;
use std::default::Default;

use hyper;
use rusoto_core::{DefaultCredentialsProvider};
use rusoto_events;
use rusoto_events::{ CloudWatchEvents, CloudWatchEventsClient };

use config;


pub trait CloudWatchEventsExecuter {

  fn events_client(&self) -> &CloudWatchEventsClient<DefaultCredentialsProvider, hyper::client::Client>;

  fn delete_rule(&self, rule_name: &str) -> Result<(), Box<error::Error>> {
    debug!("CloudWatchEventsExecuter::delete_rule");

    let req = rusoto_events::DeleteRuleRequest {
      name: rule_name.to_owned()
    };

    try!(self.events_client().delete_rule(&req));
    info!("Completed to delete-rule successfully");

    Ok(())
  }

  fn put_rule(&self, rule_conf: &config::cloudwatch_events::ScheduleRule) -> Result<(), Box<error::Error>> {
    debug!("CloudWatchEventsExecuter::put_rule");

    let req = rusoto_events::PutRuleRequest {
      name: rule_conf.name.to_owned(),
      schedule_expression: Some(rule_conf.schedule_expression.to_owned()),
      ..Default::default()
    };

    try!(self.events_client().put_rule(&req));
    info!("Completed to put-rule successfully");

    Ok(())
  }

  fn put_ecs_task_target(&self, rule_conf: &config::cloudwatch_events::ScheduleRule, rule_targets_role_arn: Option<&str>, cluster_arn: &str, task_definition_arn: &str) -> Result<(), Box<error::Error>> {
    debug!("CloudWatchEventsExecuter::put_ecs_task_target");

    let targets = vec![
      rusoto_events::Target {
        id: self.auto_generated_target_id(rule_conf),
        arn: cluster_arn.to_owned(),
        ecs_parameters: Some(rusoto_events::EcsParameters {
          task_count: Some(1),
          task_definition_arn: task_definition_arn.to_owned()
        }),
        role_arn: rule_targets_role_arn.map(str::to_string),
        ..Default::default()
      }
    ];

    let req = rusoto_events::PutTargetsRequest {
      rule: rule_conf.name.to_owned(),
      targets: targets,
      ..Default::default()
    };

    try!(self.events_client().put_targets(&req));
    info!("Completed to put-targets successfully");

    Ok(())
  }

  fn auto_generated_target_id(&self, rule_conf: &config::cloudwatch_events::ScheduleRule) -> String {
    format!("{}_target_1", rule_conf.name)
  }

}