use std::error;

use hyper;
use rusoto_core::{default_tls_client, DefaultCredentialsProvider, Region};
use rusoto_ecs::EcsClient;
use rusoto_events::CloudWatchEventsClient;

use output;

use command::ecs::Executer as EcsExecuter;
use command::cloudwatch_events::Executer as CloudwatchEventsExecuter;

pub struct Executer {
    ecs_client: EcsClient<DefaultCredentialsProvider, hyper::client::Client>,
    events_client: CloudWatchEventsClient<DefaultCredentialsProvider, hyper::client::Client>,
}

impl Executer {
    pub fn new() -> Self {
        debug!("ScheduleTaskDeleteExecuter::new");

        let ecs_client = EcsClient::new(
            default_tls_client().unwrap(),
            DefaultCredentialsProvider::new().unwrap(),
            Region::ApNortheast1,
        );
        let events_client = CloudWatchEventsClient::new(
            default_tls_client().unwrap(),
            DefaultCredentialsProvider::new().unwrap(),
            Region::ApNortheast1,
        );

        Executer {
            ecs_client: ecs_client,
            events_client: events_client,
        }
    }

    pub fn run(&self, rule_name: &str) -> Result<(), Box<error::Error>> {
        debug!("ScheduleTaskDeleteExecuter::run");

        try!(self.delete_rule(rule_name));

        output::PrintLine::success("Finished deleting the scheduled task");
        Ok(())
    }
}

impl EcsExecuter for Executer {
    fn ecs_client(&self) -> &EcsClient<DefaultCredentialsProvider, hyper::client::Client> {
        &self.ecs_client
    }
}

impl CloudwatchEventsExecuter for Executer {
    fn events_client(
        &self,
    ) -> &CloudWatchEventsClient<DefaultCredentialsProvider, hyper::client::Client> {
        &self.events_client
    }
}
