use std::error;

use rusoto_core::Region;
use rusoto_ecs::EcsClient;
use rusoto_events::EventBridgeClient;

use config;
use output;

use command::cloudwatch_events::Executer as CloudwatchEventsExecuter;
use command::ecs::Executer as EcsExecuter;
use command::error::CommandError;

pub struct Executer<'c> {
    ecs_client: EcsClient,
    events_client: EventBridgeClient,
    config: &'c config::command::ScheduleTaskConfig,
}

impl<'c> Executer<'c> {
    pub fn from_config(config: &'c config::command::ScheduleTaskConfig) -> Self {
        trace!("command::schedule_task::put::Executer::from_config");

        let ecs_client = EcsClient::new(Region::ApNortheast1);
        let events_client = EventBridgeClient::new(Region::ApNortheast1);

        Executer {
            ecs_client: ecs_client,
            events_client: events_client,
            config: config,
        }
    }

    pub fn run(&self) -> Result<(), Box<error::Error>> {
        trace!("command::schedule_task::put::Executer::run");

        let maybe_ecs_cluster = try!(self.describe_cluster(&self.config.cluster));
        let ecs_cluster = try!(maybe_ecs_cluster.ok_or(Box::new(CommandError::Unknown)));
        let ecs_cluster_arn = try!(ecs_cluster
            .cluster_arn
            .as_ref()
            .ok_or(Box::new(CommandError::Unknown,)));

        let task_definition = try!(self.register_task_definition(&self.config.task_definition));
        let task_definition_arn = try!(task_definition
            .task_definition_arn
            .as_ref()
            .ok_or(Box::new(CommandError::Unknown,),));

        let role_arn = self
            .config
            .rule_targets_role_arn
            .as_ref()
            .map(String::as_str);

        try!(self.put_rule(&self.config.rule));
        try!(self.put_ecs_task_target(
            role_arn,
            ecs_cluster_arn,
            task_definition_arn,
            &self.config,
        ));

        output::PrintLine::success("Finished putting the scheduled task");
        Ok(())
    }
}

impl<'c> EcsExecuter for Executer<'c> {
    fn ecs_client(&self) -> &EcsClient {
        &self.ecs_client
    }
}

impl<'c> CloudwatchEventsExecuter for Executer<'c> {
    fn events_client(&self) -> &EventBridgeClient {
        &self.events_client
    }
}
