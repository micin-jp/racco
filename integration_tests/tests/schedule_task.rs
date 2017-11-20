extern crate racco;

use racco::command::schedule_task;
use racco::config;

#[test]
fn schedule_task_put() {
  let conf =
    config::command::Config::from_file("fixtures/configs/schedule_task.yml", None).unwrap();
  let cmd = schedule_task::put::Command::new(&conf, Some("racco-test-schedule-job"), false);

  let res = cmd.run();
  assert!(res.is_ok());

  // TODO: assert container logs
}

#[test]
fn schedule_task_delete() {
  let conf =
    config::command::Config::from_file("fixtures/configs/schedule_task.yml", None).unwrap();
  let cmd = schedule_task::delete::Command::new(&conf, "racco-test-schedule-job");

  let res = cmd.run();
  assert!(res.is_ok());

  // TODO: assert container logs
}
