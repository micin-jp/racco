
use rusoto_ecs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Service {
    pub name: String,
    pub desired_count: i64,
    pub deployment_configuration: Option<DeploymentConfiguration>,
    pub load_balancers: Option<LoadBalancers>,
    pub task_definition: TaskDefinition,
    pub role: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskDefinition {
    pub family: String,
    pub container_definitions: ContainerDefinitions,
    pub task_role_arn: Option<String>,
    pub network_mode: Option<NetworkMode>,
    pub volumes: Option<Vec<Volume>>,
}

pub type NetworkMode = String;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Volume {
    pub host: Option<HostVolumeProperties>,
    pub name: Option<String>,
}
impl Volume {
    pub fn to_rusoto(&self) -> rusoto_ecs::Volume {
        rusoto_ecs::Volume {
            host: self.host.as_ref().map(|e| e.to_rusoto()),
            name: self.name.to_owned(),
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct HostVolumeProperties {
    pub source_path: Option<String>,
}

impl HostVolumeProperties {
    pub fn to_rusoto(&self) -> rusoto_ecs::HostVolumeProperties {
        rusoto_ecs::HostVolumeProperties {
            source_path: self.source_path.to_owned(),
        }
    }
}



// rusoto compatible structs

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentConfiguration {
    pub maximum_percent: Option<i64>,
    pub minimum_healthy_percent: Option<i64>,
}
impl DeploymentConfiguration {
    pub fn to_rusoto(&self) -> rusoto_ecs::DeploymentConfiguration {
        rusoto_ecs::DeploymentConfiguration {
            maximum_percent: self.maximum_percent,
            minimum_healthy_percent: self.minimum_healthy_percent,
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancer {
    pub container_name: Option<String>,
    pub container_port: Option<i64>,
    pub load_balancer_name: Option<String>,
    pub target_group_arn: Option<String>,
}

impl LoadBalancer {
    pub fn to_rusoto(&self) -> rusoto_ecs::LoadBalancer {
        rusoto_ecs::LoadBalancer {
            container_name: self.container_name.to_owned(),
            container_port: self.container_port,
            load_balancer_name: self.load_balancer_name.to_owned(),
            target_group_arn: self.target_group_arn.to_owned(),
        }
    }
}

pub type LoadBalancers = Vec<LoadBalancer>;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ContainerDefinition {
    pub command: Option<StringList>,
    pub cpu: Option<Integer>,
    pub disable_networking: Option<BoxedBoolean>,
    pub dns_search_domains: Option<StringList>,
    pub dns_servers: Option<StringList>,
    pub docker_labels: Option<DockerLabelsMap>,
    pub docker_security_options: Option<StringList>,
    pub entry_point: Option<StringList>,
    pub environment: Option<EnvironmentVariables>,
    pub essential: Option<BoxedBoolean>,
    pub extra_hosts: Option<HostEntryList>,
    pub hostname: Option<String>,
    pub image: Option<String>,
    pub links: Option<StringList>,
    pub log_configuration: Option<LogConfiguration>,
    pub memory: Option<BoxedInteger>,
    pub memory_reservation: Option<BoxedInteger>,
    pub mount_points: Option<MountPointList>,
    pub name: Option<String>,
    pub port_mappings: Option<PortMappingList>,
    pub privileged: Option<BoxedBoolean>,
    pub readonly_root_filesystem: Option<BoxedBoolean>,
    pub ulimits: Option<UlimitList>,
    pub user: Option<String>,
    pub volumes_from: Option<VolumeFromList>,
    pub working_directory: Option<String>,
}
impl ContainerDefinition {
    pub fn to_rusoto(&self) -> rusoto_ecs::ContainerDefinition {
        rusoto_ecs::ContainerDefinition {
            command: self.command.to_owned(),
            cpu: self.cpu,
            disable_networking: self.disable_networking,
            dns_search_domains: self.dns_search_domains.to_owned(),
            dns_servers: self.dns_servers.to_owned(),
            docker_labels: self.docker_labels.to_owned(),
            docker_security_options: self.docker_security_options.to_owned(),
            entry_point: self.entry_point.to_owned(),
            environment: self.environment
                .as_ref()
                .map(|e| e.iter().map(|e0| e0.to_rusoto()).collect()),
            essential: self.essential,
            extra_hosts: self.extra_hosts
                .as_ref()
                .map(|e| e.iter().map(|e0| e0.to_rusoto()).collect()),
            hostname: self.hostname.to_owned(),
            image: self.image.to_owned(),
            links: self.links.to_owned(),
            log_configuration: self.log_configuration.as_ref().map(|e| e.to_rusoto()),
            memory: self.memory,
            memory_reservation: self.memory_reservation,
            mount_points: self.mount_points
                .as_ref()
                .map(|e| e.iter().map(|e0| e0.to_rusoto()).collect()),
            name: self.name.to_owned(),
            port_mappings: self.port_mappings
                .as_ref()
                .map(|e| e.iter().map(|e0| e0.to_rusoto()).collect()),
            privileged: self.privileged,
            readonly_root_filesystem: self.readonly_root_filesystem,
            ulimits: self.ulimits
                .as_ref()
                .map(|e| e.iter().map(|e0| e0.to_rusoto()).collect()),
            user: self.user.to_owned(),
            volumes_from: self.volumes_from
                .as_ref()
                .map(|e| e.iter().map(|e0| e0.to_rusoto()).collect()),
            working_directory: self.working_directory.to_owned(),
        }
    }
}

pub type ContainerDefinitions = Vec<ContainerDefinition>;

pub type BoxedBoolean = bool;
pub type BoxedInteger = i64;
pub type Integer = i64;
pub type StringList = Vec<String>;

pub type DockerLabelsMap = ::std::collections::HashMap<String, String>;
pub type EnvironmentVariables = Vec<KeyValuePair>;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct KeyValuePair {
    pub name: Option<String>,
    pub value: Option<String>,
}
impl KeyValuePair {
    pub fn to_rusoto(&self) -> rusoto_ecs::KeyValuePair {
        rusoto_ecs::KeyValuePair {
            name: self.name.to_owned(),
            value: self.value.to_owned(),
        }
    }
}


#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct HostEntry {
    pub hostname: String,
    pub ip_address: String,
}
impl HostEntry {
    pub fn to_rusoto(&self) -> rusoto_ecs::HostEntry {
        rusoto_ecs::HostEntry {
            hostname: self.hostname.to_owned(),
            ip_address: self.ip_address.to_owned(),
        }
    }
}

pub type HostEntryList = Vec<HostEntry>;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct MountPoint {
    pub container_path: Option<String>,
    pub read_only: Option<BoxedBoolean>,
    pub source_volume: Option<String>,
}
impl MountPoint {
    pub fn to_rusoto(&self) -> rusoto_ecs::MountPoint {
        rusoto_ecs::MountPoint {
            container_path: self.container_path.to_owned(),
            read_only: self.read_only,
            source_volume: self.source_volume.to_owned(),
        }
    }
}

pub type MountPointList = Vec<MountPoint>;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PortMapping {
    pub container_port: Option<BoxedInteger>,
    pub host_port: Option<BoxedInteger>,
    pub protocol: Option<TransportProtocol>,
}
impl PortMapping {
    pub fn to_rusoto(&self) -> rusoto_ecs::PortMapping {
        rusoto_ecs::PortMapping {
            container_port: self.container_port,
            host_port: self.host_port,
            protocol: self.protocol.to_owned(),
        }
    }
}


pub type PortMappingList = Vec<PortMapping>;
pub type TransportProtocol = String;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Ulimit {
    pub hard_limit: Integer,
    pub name: UlimitName,
    pub soft_limit: Integer,
}
impl Ulimit {
    pub fn to_rusoto(&self) -> rusoto_ecs::Ulimit {
        rusoto_ecs::Ulimit {
            hard_limit: self.hard_limit,
            name: self.name.to_owned(),
            soft_limit: self.soft_limit,
        }
    }
}

pub type UlimitList = Vec<Ulimit>;
pub type UlimitName = String;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct VolumeFrom {
    pub read_only: Option<BoxedBoolean>,
    pub source_container: Option<String>,
}
impl VolumeFrom {
    pub fn to_rusoto(&self) -> rusoto_ecs::VolumeFrom {
        rusoto_ecs::VolumeFrom {
            read_only: self.read_only,
            source_container: self.source_container.to_owned(),
        }
    }
}

pub type VolumeFromList = Vec<VolumeFrom>;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct LogConfiguration {
    pub log_driver: LogDriver,
    pub options: Option<LogConfigurationOptionsMap>,
}
impl LogConfiguration {
    pub fn to_rusoto(&self) -> rusoto_ecs::LogConfiguration {
        rusoto_ecs::LogConfiguration {
            log_driver: self.log_driver.to_owned(),
            options: self.options.to_owned(),
        }
    }
}

pub type LogConfigurationOptionsMap = ::std::collections::HashMap<String, String>;
pub type LogDriver = String;
