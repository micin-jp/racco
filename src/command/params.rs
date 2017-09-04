use std::error;
use hyper;
use rusoto_core::{default_tls_client, DefaultCredentialsProvider, Region};
use rusoto_ssm::SsmClient;
use config;

use super::error::CommandError;

pub trait ParamsExecuter {

  fn client(&self) -> SsmClient<DefaultCredentialsProvider, hyper::client::Client> {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    return SsmClient::new(default_tls_client().unwrap(), credentials, Region::ApNortheast1);
  }

  fn config(&self) -> &config::command::ParamsConfig;

  fn name_with_path(&self, name: &str) -> String {
    let mut path = self.path();
    path.push_str(name);
    path
  }

  fn path(&self) -> String {
    let mut path = self.config().path.to_owned();
    if !path.ends_with("/") {
      path.push_str("/");
    }
    if !path.starts_with("/") {
      path = format!("/{}", path);
    }
    path
  }

  fn strip_path<'a>(&self, name: &'a str) -> Result<&'a str, Box<error::Error>> {
    let path = self.path();
    if name.starts_with(&path) {
      return Ok(name.trim_left_matches(&path))
    } else {
      Err(Box::new(CommandError::Unknown))
    }
  }


}
