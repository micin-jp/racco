use async_trait::async_trait;
use std::default::Default;
use std::error;

use rusoto_core::RusotoError;
use rusoto_events;
use rusoto_events::{EventBridge, EventBridgeClient};

use crate::config;

#[async_trait]
pub trait Executer {
    fn events_client(&self) -> &EventBridgeClient;

    async fn rule_exists(&self, rule_name: &str) -> Result<bool, Box<dyn error::Error>> {
        trace!("command::cloudwatch_events::Executer::rule_exists");

        let req = rusoto_events::DescribeRuleRequest {
            name: rule_name.to_owned(),
            ..Default::default()
        };

        match self.events_client().describe_rule(req).await {
            Ok(res) => Ok(res.arn.is_some()),
            Err(RusotoError::Service(rusoto_events::DescribeRuleError::ResourceNotFound(_))) => {
                Ok(false)
            }
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn delete_rule(&self, rule_name: &str) -> Result<(), Box<dyn error::Error>> {
        trace!("command::cloudwatch_events::Executer::delete_rule");

        let req = rusoto_events::DeleteRuleRequest {
            name: rule_name.to_owned(),
            ..Default::default()
        };

        self.events_client().delete_rule(req).await?;
        info!("Completed to delete-rule successfully");

        Ok(())
    }

    async fn put_rule(
        &self,
        rule_conf: &config::cloudwatch_events::ScheduleRule,
    ) -> Result<(), Box<dyn error::Error>> {
        trace!("command::cloudwatch_events::Executer::put_rule");

        let req = rusoto_events::PutRuleRequest {
            name: rule_conf.name.to_owned(),
            schedule_expression: Some(rule_conf.schedule_expression.to_owned()),
            ..Default::default()
        };

        self.events_client().put_rule(req).await?;
        info!("Completed to put-rule successfully");

        Ok(())
    }

    async fn put_ecs_task_target(
        &self,
        rule_targets_role_arn: Option<&str>,
        cluster_arn: &str,
        task_definition_arn: &str,
        config: &config::command::ScheduleTaskConfig,
    ) -> Result<(), Box<dyn error::Error>> {
        trace!("command::cloudwatch_events::Executer::put_ecs_task_target");

        let targets = vec![rusoto_events::Target {
            id: self.auto_generated_target_id(&config.rule),
            arn: cluster_arn.to_owned(),
            ecs_parameters: Some(rusoto_events::EcsParameters {
                task_count: Some(1),
                task_definition_arn: task_definition_arn.to_owned(),
                launch_type: config.launch_type.to_owned(),
                platform_version: config.platform_version.to_owned(),
                network_configuration: config.network_configuration.as_ref().map(|d| d.to_rusoto()),
                ..Default::default()
            }),
            role_arn: rule_targets_role_arn.map(str::to_string),
            ..Default::default()
        }];

        let req = rusoto_events::PutTargetsRequest {
            rule: config.rule.name.to_owned(),
            targets: targets,
            ..Default::default()
        };

        self.events_client().put_targets(req).await?;
        info!("Completed to put-targets successfully");

        Ok(())
    }

    async fn remove_targets(&self, rule_name: &str) -> Result<(), Box<dyn error::Error>> {
        trace!("command::cloudwatch_events::Executer::remove_targets");

        let req = rusoto_events::ListTargetsByRuleRequest {
            rule: rule_name.to_owned(),
            ..Default::default()
        };

        let res = self.events_client().list_targets_by_rule(req).await?;
        if let Some(targets) = res.targets {
            let req = rusoto_events::RemoveTargetsRequest {
                rule: rule_name.to_owned(),
                ids: targets.iter().map(|t| t.id.to_owned()).collect(),
                ..Default::default()
            };
            self.events_client().remove_targets(req).await?;
        }

        Ok(())
    }

    fn auto_generated_target_id(
        &self,
        rule_conf: &config::cloudwatch_events::ScheduleRule,
    ) -> String {
        format!("{}_target_1", rule_conf.name)
    }
}
