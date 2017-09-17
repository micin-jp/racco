extern crate racco;

use racco::command::run_task;
use racco::config;

#[test]
fn run_task() {

  let conf = config::command::Config::from_file("fixtures/configs/run_task.yml", None).unwrap();
  let cmd = run_task::Command::new(&conf, "racco-test-job", false);

  let res = cmd.run();
  assert!(res.is_ok());

  // TODO: assert container logs
}
