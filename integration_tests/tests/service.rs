extern crate racco;

use racco::command::service;
use racco::config;

#[test]
fn service_deploy() {

  let conf = config::command::Config::from_file("fixtures/configs/service_deploy.yml", None).unwrap();
  let cmd = service::deploy::Command::new(&conf, "racco-test-web", false);

  // first time 
  let res1 = cmd.run();
  assert!(res1.is_ok());

  // second time
  let res2 = cmd.run();
  assert!(res2.is_ok());

  // TODO: assert http response from nginx container
}

