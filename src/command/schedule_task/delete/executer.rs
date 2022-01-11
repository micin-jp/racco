use std::error;

use rusoto_core::Region;
use rusoto_ecs::EcsClient;
use rusoto_events::EventBridgeClient;

use crate::command::cloudwatch_events::Executer as CloudwatchEventsExecuter;
use crate::command::ecs::Executer as EcsExecuter;
use crate::output;

pub struct Executer {
    ecs_client: EcsClient,
    events_client: EventBridgeClient,
}

impl Executer {
    pub fn new() -> Self {
        trace!("command::schedule_task::delete::Executer::new");

        let ecs_client = EcsClient::new(Region::ApNortheast1);
        let events_client = EventBridgeClient::new(Region::ApNortheast1);

        Executer {
            ecs_client: ecs_client,
            events_client: events_client,
        }
    }

    pub fn run(&self, rule_name: &str) -> Result<(), Box<dyn error::Error>> {
        trace!("command::schedule_task::delete::Executer::run");

        if r#try!(self.rule_exists(rule_name)) {
            r#try!(self.remove_targets(rule_name));
            r#try!(self.delete_rule(rule_name));
            output::PrintLine::success("Finished deleting the scheduled task");
        } else {
            output::PrintLine::success("The rule does not exists");
        }

        Ok(())
    }
}

impl EcsExecuter for Executer {
    fn ecs_client(&self) -> &EcsClient {
        &self.ecs_client
    }
}

impl CloudwatchEventsExecuter for Executer {
    fn events_client(&self) -> &EventBridgeClient {
        &self.events_client
    }
}
