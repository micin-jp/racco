extern crate racco;

use racco::command::deploy;
use racco::config;

#[test]
fn deploy_service() {

  let conf =  config::command::Config::from_file("fixtures/configs/deploy_service.yml", None).unwrap();
  let cmd = deploy::Command::new(&conf, Some("racco-test-web"));

  // first time 
  let res1 = cmd.run();
  assert!(res1.is_ok());

  // second time
  let res2 = cmd.run();
  assert!(res2.is_ok());

  // TODO: assert http response from nginx container
}

