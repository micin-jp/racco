extern crate racco;

use racco::command::{ScheduleTaskPutCommand, ScheduleTaskDeleteCommand};
use racco::config;

#[test]
fn schedule_task_put() {

  let conf = config::command::Config::from_file("fixtures/configs/schedule_task.yml").unwrap();
  let cmd = ScheduleTaskPutCommand::new(&conf, "racco-test-schedule-job");

  let res = cmd.run();
  assert!(res.is_ok());

  // TODO: assert container logs 
}

#[test]
fn schedule_task_delete() {

  let conf = config::command::Config::from_file("fixtures/configs/schedule_task.yml").unwrap();
  let cmd = ScheduleTaskDeleteCommand::new(&conf, "racco-test-schedule-job");

  let res = cmd.run();
  assert!(res.is_ok());

  // TODO: assert container logs 
}