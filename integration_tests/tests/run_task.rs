extern crate racco;

use racco::command::{RunTaskCommand};
use racco::config;

#[test]
fn run_task() {

  let conf = config::command::Config::from_file("fixtures/configs/run_task.yml").unwrap();
  let cmd = RunTaskCommand::new(&conf, "racco-test-job");

  let res = cmd.run();
  assert!(res.is_ok());

  // TODO: assert container logs 
}
