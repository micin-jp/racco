#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduleRule {
  pub name: String,
  pub schedule_expression: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfiguration {
  pub awsvpc_configuration: Option<AwsVpcConfiguration>,
}
impl NetworkConfiguration {
  pub fn to_rusoto(&self) -> rusoto_events::NetworkConfiguration {
    rusoto_events::NetworkConfiguration {
      awsvpc_configuration: self.awsvpc_configuration.as_ref().map(|e| e.to_rusoto()),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwsVpcConfiguration {
  pub assign_public_ip: Option<String>,
  pub security_groups: Option<Vec<String>>,
  pub subnets: Vec<String>,
}
impl AwsVpcConfiguration {
  pub fn to_rusoto(&self) -> rusoto_events::AwsVpcConfiguration {
    rusoto_events::AwsVpcConfiguration {
      assign_public_ip: self.assign_public_ip.to_owned(),
      security_groups: self.security_groups.to_owned(),
      subnets: self.subnets.to_owned(),
    }
  }
}
