use hyper;
use rusoto_core::{default_tls_client, DefaultCredentialsProvider, Region};
use rusoto_ssm::SsmClient;
use config;

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

}
