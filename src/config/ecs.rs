use rusoto_ecs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Service {
    pub name: String,
    pub desired_count: Option<i64>,
    pub deployment_configuration: Option<DeploymentConfiguration>,
    pub load_balancers: Option<LoadBalancers>,
    pub task_definition: TaskDefinition,
    pub role: Option<String>,
    pub launch_type: Option<String>,
    pub network_configuration: Option<NetworkConfiguration>,
    pub service_registries: Option<Vec<ServiceRegistry>>,
    pub platform_version: Option<String>,
    pub enable_execute_command: Option<bool>,
    pub tags: Option<Tags>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskDefinition {
    pub family: String,
    pub container_definitions: ContainerDefinitions,
    pub task_role_arn: Option<String>,
    pub network_mode: Option<NetworkMode>,
    pub volumes: Option<Vec<Volume>>,
    pub requires_compatibilities: Option<Vec<String>>,
    pub execution_role_arn: Option<String>,
    pub cpu: Option<String>,
    pub memory: Option<String>,
    pub proxy_configuration: Option<ProxyConfiguration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfiguration {
    pub awsvpc_configuration: Option<AwsVpcConfiguration>,
}
impl NetworkConfiguration {
    pub fn to_rusoto(&self) -> rusoto_ecs::NetworkConfiguration {
        rusoto_ecs::NetworkConfiguration {
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
    pub fn to_rusoto(&self) -> rusoto_ecs::AwsVpcConfiguration {
        rusoto_ecs::AwsVpcConfiguration {
            assign_public_ip: self.assign_public_ip.to_owned(),
            security_groups: self.security_groups.to_owned(),
            subnets: self.subnets.to_owned(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRegistry {
    pub container_name: Option<String>,
    pub container_port: Option<i64>,
    pub port: Option<i64>,
    pub registry_arn: Option<String>,
}
impl ServiceRegistry {
    pub fn to_rusoto(&self) -> rusoto_ecs::ServiceRegistry {
        rusoto_ecs::ServiceRegistry {
            container_name: self.container_name.to_owned(),
            container_port: self.container_port,
            port: self.port,
            registry_arn: self.registry_arn.to_owned(),
        }
    }
}

pub type NetworkMode = String;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Volume {
    pub docker_volume_configuration: Option<DockerVolumeConfiguration>,
    pub efs_volume_configuration: Option<EFSVolumeConfiguration>,
    pub host: Option<HostVolumeProperties>,
    pub name: Option<String>,
}
impl Volume {
    pub fn to_rusoto(&self) -> rusoto_ecs::Volume {
        rusoto_ecs::Volume {
            docker_volume_configuration: self
                .docker_volume_configuration
                .as_ref()
                .map(|c| c.to_rusoto()),
            efs_volume_configuration: self
                .efs_volume_configuration
                .as_ref()
                .map(|c| c.to_rusoto()),
            fsx_windows_file_server_volume_configuration: None, // not supported
            host: self.host.as_ref().map(|e| e.to_rusoto()),
            name: self.name.to_owned(),
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct DockerVolumeConfiguration {
    pub autoprovision: Option<bool>,
    pub driver: Option<String>,
    pub driver_opts: Option<::std::collections::HashMap<String, String>>,
    pub labels: Option<::std::collections::HashMap<String, String>>,
    pub scope: Option<String>,
}
impl DockerVolumeConfiguration {
    pub fn to_rusoto(&self) -> rusoto_ecs::DockerVolumeConfiguration {
        rusoto_ecs::DockerVolumeConfiguration {
            autoprovision: self.autoprovision,
            driver: self.driver.to_owned(),
            driver_opts: self.driver_opts.to_owned(),
            labels: self.labels.to_owned(),
            scope: self.scope.to_owned(),
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct EFSVolumeConfiguration {
    pub authorization_config: Option<EFSAuthorizationConfig>,
    pub file_system_id: String,
    pub root_directory: Option<String>,
    pub transit_encryption: Option<String>,
    pub transit_encryption_port: Option<i64>,
}
impl EFSVolumeConfiguration {
    pub fn to_rusoto(&self) -> rusoto_ecs::EFSVolumeConfiguration {
        rusoto_ecs::EFSVolumeConfiguration {
            authorization_config: self.authorization_config.as_ref().map(|a| a.to_rusoto()),
            file_system_id: self.file_system_id.to_owned(),
            root_directory: self.root_directory.to_owned(),
            transit_encryption: self.transit_encryption.to_owned(),
            transit_encryption_port: self.transit_encryption_port.to_owned(),
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct EFSAuthorizationConfig {
    pub access_point_id: Option<String>,
    pub iam: Option<String>,
}
impl EFSAuthorizationConfig {
    pub fn to_rusoto(&self) -> rusoto_ecs::EFSAuthorizationConfig {
        rusoto_ecs::EFSAuthorizationConfig {
            access_point_id: self.access_point_id.to_owned(),
            iam: self.iam.to_owned(),
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

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ProxyConfiguration {
    pub container_name: String,
    pub properties: Option<Vec<KeyValuePair>>,
    pub type_: Option<String>,
}
impl ProxyConfiguration {
    pub fn to_rusoto(&self) -> rusoto_ecs::ProxyConfiguration {
        rusoto_ecs::ProxyConfiguration {
            container_name: self.container_name.to_owned(),
            properties: self
                .properties
                .as_ref()
                .map(|p| p.iter().map(|p0| p0.to_rusoto()).collect()),
            type_: self.type_.to_owned(),
        }
    }
}

// rusoto compatible structs

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentConfiguration {
    pub maximum_percent: Option<i64>,
    pub minimum_healthy_percent: Option<i64>,
    pub deployment_circuit_breaker: Option<DeploymentCircuitBreaker>,
}
impl DeploymentConfiguration {
    pub fn to_rusoto(&self) -> rusoto_ecs::DeploymentConfiguration {
        rusoto_ecs::DeploymentConfiguration {
            maximum_percent: self.maximum_percent,
            minimum_healthy_percent: self.minimum_healthy_percent,
            deployment_circuit_breaker: self
                .deployment_circuit_breaker
                .as_ref()
                .map(|d| d.to_rusoto()),
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentCircuitBreaker {
    pub enable: bool,
    pub rollback: bool,
}
impl DeploymentCircuitBreaker {
    pub fn to_rusoto(&self) -> rusoto_ecs::DeploymentCircuitBreaker {
        rusoto_ecs::DeploymentCircuitBreaker {
            enable: self.enable,
            rollback: self.rollback,
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
    pub environment_files: Option<Vec<EnvironmentFile>>,
    pub essential: Option<BoxedBoolean>,
    pub extra_hosts: Option<HostEntryList>,
    pub hostname: Option<String>,
    pub image: Option<String>,
    pub interactive: Option<bool>,
    pub links: Option<StringList>,
    pub log_configuration: Option<LogConfiguration>,
    pub memory: Option<BoxedInteger>,
    pub memory_reservation: Option<BoxedInteger>,
    pub mount_points: Option<MountPointList>,
    pub name: Option<String>,
    pub port_mappings: Option<PortMappingList>,
    pub privileged: Option<BoxedBoolean>,
    pub pseudo_terminal: Option<bool>,
    pub readonly_root_filesystem: Option<BoxedBoolean>,
    pub repository_credentials: Option<RepositoryCredentials>,
    pub resource_requirements: Option<Vec<ResourceRequirement>>,
    pub secrets: Option<Vec<Secret>>,
    pub system_controls: Option<Vec<SystemControl>>,
    pub ulimits: Option<UlimitList>,
    pub user: Option<String>,
    pub volumes_from: Option<VolumeFromList>,
    pub working_directory: Option<String>,
    pub health_check: Option<HealthCheck>,
    pub linux_parameters: Option<LinuxParameters>,
    pub depends_on: Option<Vec<ContainerDependency>>,
    pub firelens_configuration: Option<FirelensConfiguration>,
    pub start_timeout: Option<i64>,
    pub stop_timeout: Option<i64>,
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
            environment: self
                .environment
                .as_ref()
                .map(|e| e.iter().map(|e0| e0.to_rusoto()).collect()),
            environment_files: self
                .environment_files
                .as_ref()
                .map(|e| e.iter().map(|e0| e0.to_rusoto()).collect()),
            essential: self.essential,
            extra_hosts: self
                .extra_hosts
                .as_ref()
                .map(|e| e.iter().map(|e0| e0.to_rusoto()).collect()),
            hostname: self.hostname.to_owned(),
            image: self.image.to_owned(),
            interactive: self.interactive,
            links: self.links.to_owned(),
            log_configuration: self.log_configuration.as_ref().map(|e| e.to_rusoto()),
            memory: self.memory,
            memory_reservation: self.memory_reservation,
            mount_points: self
                .mount_points
                .as_ref()
                .map(|e| e.iter().map(|e0| e0.to_rusoto()).collect()),
            name: self.name.to_owned(),
            port_mappings: self
                .port_mappings
                .as_ref()
                .map(|e| e.iter().map(|e0| e0.to_rusoto()).collect()),
            privileged: self.privileged,
            pseudo_terminal: self.pseudo_terminal,
            readonly_root_filesystem: self.readonly_root_filesystem,
            repository_credentials: self.repository_credentials.as_ref().map(|e| e.to_rusoto()),
            resource_requirements: self
                .resource_requirements
                .as_ref()
                .map(|e| e.iter().map(|e0| e0.to_rusoto()).collect()),
            secrets: self
                .secrets
                .as_ref()
                .map(|e| e.iter().map(|e0| e0.to_rusoto()).collect()),
            system_controls: self
                .system_controls
                .as_ref()
                .map(|e| e.iter().map(|e0| e0.to_rusoto()).collect()),
            ulimits: self
                .ulimits
                .as_ref()
                .map(|e| e.iter().map(|e0| e0.to_rusoto()).collect()),
            user: self.user.to_owned(),
            volumes_from: self
                .volumes_from
                .as_ref()
                .map(|e| e.iter().map(|e0| e0.to_rusoto()).collect()),
            working_directory: self.working_directory.to_owned(),
            health_check: self.health_check.as_ref().map(|e| e.to_rusoto()),
            linux_parameters: self.linux_parameters.as_ref().map(|e| e.to_rusoto()),
            depends_on: self
                .depends_on
                .as_ref()
                .map(|e| e.iter().map(|e0| e0.to_rusoto()).collect()),
            firelens_configuration: self.firelens_configuration.as_ref().map(|e| e.to_rusoto()),
            start_timeout: self.start_timeout,
            stop_timeout: self.stop_timeout,
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ContainerDependency {
    pub condition: String,
    pub container_name: String,
}
impl ContainerDependency {
    pub fn to_rusoto(&self) -> rusoto_ecs::ContainerDependency {
        rusoto_ecs::ContainerDependency {
            condition: self.condition.to_owned(),
            container_name: self.container_name.to_owned(),
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct FirelensConfiguration {
    pub options: Option<::std::collections::HashMap<String, String>>,
    pub type_: String,
}
impl FirelensConfiguration {
    pub fn to_rusoto(&self) -> rusoto_ecs::FirelensConfiguration {
        rusoto_ecs::FirelensConfiguration {
            options: self.options.to_owned(),
            type_: self.type_.to_owned(),
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryCredentials {
    pub credentials_parameter: String,
}
impl RepositoryCredentials {
    pub fn to_rusoto(&self) -> rusoto_ecs::RepositoryCredentials {
        rusoto_ecs::RepositoryCredentials {
            credentials_parameter: self.credentials_parameter.to_owned(),
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequirement {
    pub type_: String,
    pub value: String,
}
impl ResourceRequirement {
    pub fn to_rusoto(&self) -> rusoto_ecs::ResourceRequirement {
        rusoto_ecs::ResourceRequirement {
            type_: self.type_.to_owned(),
            value: self.value.to_owned(),
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Secret {
    pub name: String,
    pub value_from: String,
}
impl Secret {
    pub fn to_rusoto(&self) -> rusoto_ecs::Secret {
        rusoto_ecs::Secret {
            name: self.name.to_owned(),
            value_from: self.value_from.to_owned(),
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentFile {
    pub type_: String,
    pub value: String,
}
impl EnvironmentFile {
    pub fn to_rusoto(&self) -> rusoto_ecs::EnvironmentFile {
        rusoto_ecs::EnvironmentFile {
            type_: self.type_.to_owned(),
            value: self.value.to_owned(),
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct SystemControl {
    pub namespace: Option<String>,
    pub value: Option<String>,
}
impl SystemControl {
    pub fn to_rusoto(&self) -> rusoto_ecs::SystemControl {
        rusoto_ecs::SystemControl {
            namespace: self.namespace.to_owned(),
            value: self.value.to_owned(),
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
    pub secret_options: Option<Vec<Secret>>,
}
impl LogConfiguration {
    pub fn to_rusoto(&self) -> rusoto_ecs::LogConfiguration {
        rusoto_ecs::LogConfiguration {
            log_driver: self.log_driver.to_owned(),
            options: self.options.to_owned(),
            secret_options: self
                .secret_options
                .as_ref()
                .map(|e| e.iter().map(|e0| e0.to_rusoto()).collect()),
        }
    }
}

pub type LogConfigurationOptionsMap = ::std::collections::HashMap<String, String>;
pub type LogDriver = String;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    pub command: Vec<String>,
    pub interval: Option<i64>,
    pub retries: Option<i64>,
    pub start_period: Option<i64>,
    pub timeout: Option<i64>,
}
impl HealthCheck {
    pub fn to_rusoto(&self) -> rusoto_ecs::HealthCheck {
        rusoto_ecs::HealthCheck {
            command: self.command.to_owned(),
            interval: self.interval,
            retries: self.retries,
            start_period: self.start_period,
            timeout: self.timeout,
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct LinuxParameters {
    pub capabilities: Option<KernelCapabilities>,
    pub devices: Option<Vec<Device>>,
    pub init_process_enabled: Option<bool>,
    pub shared_memory_size: Option<i64>,
    pub tmpfs: Option<Vec<Tmpfs>>,
    pub max_swap: Option<i64>,
    pub swappiness: Option<i64>,
}
impl LinuxParameters {
    pub fn to_rusoto(&self) -> rusoto_ecs::LinuxParameters {
        rusoto_ecs::LinuxParameters {
            capabilities: self.capabilities.as_ref().map(|e| e.to_rusoto()),
            devices: self
                .devices
                .as_ref()
                .map(|e| e.iter().map(|e0| e0.to_rusoto()).collect()),
            init_process_enabled: self.init_process_enabled,
            shared_memory_size: self.shared_memory_size,
            tmpfs: self
                .tmpfs
                .as_ref()
                .map(|e| e.iter().map(|e0| e0.to_rusoto()).collect()),
            max_swap: self.max_swap,
            swappiness: self.swappiness,
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct KernelCapabilities {
    pub add: Option<Vec<String>>,
    pub drop: Option<Vec<String>>,
}
impl KernelCapabilities {
    pub fn to_rusoto(&self) -> rusoto_ecs::KernelCapabilities {
        rusoto_ecs::KernelCapabilities {
            add: self.add.to_owned(),
            drop: self.drop.to_owned(),
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub container_path: Option<String>,
    pub host_path: String,
    pub permissions: Option<Vec<String>>,
}
impl Device {
    pub fn to_rusoto(&self) -> rusoto_ecs::Device {
        rusoto_ecs::Device {
            container_path: self.container_path.to_owned(),
            host_path: self.host_path.to_owned(),
            permissions: self.permissions.to_owned(),
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Tmpfs {
    pub container_path: String,
    pub mount_options: Option<Vec<String>>,
    pub size: i64,
}
impl Tmpfs {
    pub fn to_rusoto(&self) -> rusoto_ecs::Tmpfs {
        rusoto_ecs::Tmpfs {
            container_path: self.container_path.to_owned(),
            mount_options: self.mount_options.to_owned(),
            size: self.size,
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub key: Option<String>,
    pub value: Option<String>,
}

impl Tag {
    pub fn to_rusoto(&self) -> rusoto_ecs::Tag {
        rusoto_ecs::Tag {
            key: self.key.to_owned(),
            value: self.value.to_owned(),
        }
    }
}

pub type Tags = Vec<Tag>;
