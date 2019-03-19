use std::default::Default;
use std::error;

use rusoto_ecs;
use rusoto_ecs::{Ecs, EcsClient};

use command::error::CommandError;
use config;

pub struct TaskDescription {
    pub task: Option<rusoto_ecs::Task>,
    pub failure: Option<rusoto_ecs::Failure>,
}

pub trait Executer {
    fn ecs_client(&self) -> &EcsClient;

    fn describe_cluster(
        &self,
        name: &str,
    ) -> Result<Option<rusoto_ecs::Cluster>, Box<error::Error>> {
        trace!("command::ecs::Executer::describe_cluster");

        let req = rusoto_ecs::DescribeClustersRequest {
            clusters: Some(vec![name.to_owned()]),
            ..Default::default()
        };

        let res = try!(self.ecs_client().describe_clusters(req).sync());
        info!("Completed to describe clusters successfully");

        match res.clusters {
            Some(clusters) => {
                let actives = clusters
                    .iter()
                    .filter(|cluster| {
                        cluster.status.is_some() && cluster.status.as_ref().unwrap() == "ACTIVE"
                    })
                    .collect::<Vec<&rusoto_ecs::Cluster>>();
                Ok(actives.first().cloned().cloned())
            }
            _ => Err(Box::new(CommandError::Unknown)),
        }
    }

    fn describe_latest_task_definition(
        &self,
        family: &str,
    ) -> Result<Option<rusoto_ecs::TaskDefinition>, Box<error::Error>> {
        trace!("command::ecs::Executer::describe_latest_task_definition");

        let req = rusoto_ecs::DescribeTaskDefinitionRequest {
            task_definition: family.to_owned(),
            ..Default::default()
        };

        match self.ecs_client().describe_task_definition(req).sync() {
            Ok(res) => {
                info!("Completed to describe task_definition successfully");
                Ok(res.task_definition)
            }
            Err(rusoto_ecs::DescribeTaskDefinitionError::Client(_)) => {
                info!("Not found the task-definition: {}", family);
                Ok(None)
            }
            Err(e) => Err(Box::new(e)),
        }
    }

    fn register_task_definition(
        &self,
        task_definition_conf: &config::ecs::TaskDefinition,
    ) -> Result<rusoto_ecs::TaskDefinition, Box<error::Error>> {
        trace!("command::ecs::Executer::register_task_definition");
        let req = rusoto_ecs::RegisterTaskDefinitionRequest {
            family: task_definition_conf.family.to_owned(),
            task_role_arn: task_definition_conf.task_role_arn.to_owned(),
            network_mode: task_definition_conf.network_mode.to_owned(),
            volumes: task_definition_conf
                .volumes
                .as_ref()
                .map(|volumes| volumes.iter().map(|v| v.to_rusoto()).collect()),
            container_definitions: task_definition_conf
                .container_definitions
                .iter()
                .map(|cd| cd.to_rusoto())
                .collect(),
            execution_role_arn: task_definition_conf.execution_role_arn.to_owned(),
            requires_compatibilities: task_definition_conf.requires_compatibilities.to_owned(),
            cpu: task_definition_conf.cpu.to_owned(),
            memory: task_definition_conf.memory.to_owned(),
            ..Default::default()
        };

        let res = try!(self.ecs_client().register_task_definition(req).sync());
        info!("Completed to register task_definition successfully");

        res.task_definition.ok_or(Box::new(CommandError::Unknown))
    }

    fn create_service(
        &self,
        cluster: &str,
        service_conf: &config::ecs::Service,
        task_definition: &str,
    ) -> Result<rusoto_ecs::Service, Box<error::Error>> {
        trace!("command::ecs::Executer::create_service");

        let req = rusoto_ecs::CreateServiceRequest {
            cluster: Some(cluster.to_owned()),
            service_name: service_conf.name.to_owned(),
            desired_count: service_conf.desired_count,
            //deployment_configuration: service_conf.deployment_configuration.as_ref().map(|d| d.to_rusoto()),
            load_balancers: service_conf
                .load_balancers
                .as_ref()
                .map(|lbs| lbs.iter().map(|lb| lb.to_rusoto()).collect()),
            role: service_conf.role.to_owned(),
            launch_type: service_conf.launch_type.to_owned(),
            network_configuration: service_conf
                .network_configuration
                .as_ref()
                .map(|e| e.to_rusoto()),
            service_registries: service_conf
                .service_registries
                .as_ref()
                .map(|srs| srs.iter().map(|sr| sr.to_rusoto()).collect()),
            task_definition: task_definition.to_owned(),
            platform_version: service_conf.platform_version.to_owned(),
            ..Default::default()
        };

        let res = try!(self.ecs_client().create_service(req).sync());
        info!("Completed to create service successfully");

        res.service.ok_or(Box::new(CommandError::Unknown))
    }

    fn describe_service(
        &self,
        cluster: &str,
        service_conf: &config::ecs::Service,
    ) -> Result<Option<rusoto_ecs::Service>, Box<error::Error>> {
        trace!("command::ecs::Executer::describe_service");

        let req = rusoto_ecs::DescribeServicesRequest {
            cluster: Some(cluster.to_owned()),
            services: vec![service_conf.name.to_owned()],
            ..Default::default()
        };

        let res = try!(self.ecs_client().describe_services(req).sync());
        info!("Completed to describe services successfully");

        match res.services {
            Some(services) => {
                let actives = services
                    .iter()
                    .filter(|service| {
                        service.status.is_some() && service.status.as_ref().unwrap() == "ACTIVE"
                    })
                    .collect::<Vec<&rusoto_ecs::Service>>();
                Ok(actives.first().cloned().cloned())
            }
            _ => Err(Box::new(CommandError::Unknown)),
        }
    }

    fn update_service(
        &self,
        cluster: &str,
        service_conf: &config::ecs::Service,
        task_definition: &rusoto_ecs::TaskDefinition,
    ) -> Result<rusoto_ecs::Service, Box<error::Error>> {
        trace!("command::ecs::Executer::update_service");

        if task_definition.task_definition_arn.is_none() {
            return Err(Box::new(CommandError::Unknown));
        }

        let req = rusoto_ecs::UpdateServiceRequest {
            service: service_conf.name.to_owned(),
            cluster: Some(cluster.to_owned()),
            desired_count: service_conf.desired_count,
            deployment_configuration: service_conf
                .deployment_configuration
                .as_ref()
                .map(|d| d.to_rusoto()),
            network_configuration: service_conf
                .network_configuration
                .as_ref()
                .map(|e| e.to_rusoto()),
            task_definition: task_definition.task_definition_arn.to_owned(),
            platform_version: service_conf.platform_version.to_owned(),
            ..Default::default()
        };

        let res = try!(self.ecs_client().update_service(req).sync());
        info!("Completed to update service successfully");

        let service = res.service.map(|s| s.to_owned());
        service.ok_or(Box::new(CommandError::Unknown))
    }

    fn describe_task(
        &self,
        cluster: &str,
        task_arn: &str,
    ) -> Result<TaskDescription, Box<error::Error>> {
        let req = rusoto_ecs::DescribeTasksRequest {
            cluster: Some(cluster.to_owned()),
            tasks: vec![task_arn.to_owned()],
            ..Default::default()
        };

        let result = try!(self.ecs_client().describe_tasks(req).sync());
        debug!("{:?}", result);

        let failure = result
            .failures
            .as_ref()
            .and_then(|failures| failures.first())
            .map(|f| f.to_owned());
        match result.tasks.as_ref().and_then(|tasks| tasks.first()) {
            Some(task) => Ok(TaskDescription {
                task: Some(task.to_owned()),
                failure: failure,
            }),
            None => Err(Box::new(CommandError::Unknown)),
        }
    }

    fn run_task(
        &self,
        cluster: &str,
        task_definition_arn: &str,
        launch_type: Option<&str>,
        network_configuration: Option<&config::ecs::NetworkConfiguration>,
        platform_version: Option<&str>,
    ) -> Result<TaskDescription, Box<error::Error>> {
        let req = rusoto_ecs::RunTaskRequest {
            cluster: Some(cluster.to_owned()),
            task_definition: task_definition_arn.to_owned(),
            launch_type: launch_type.map(str::to_string),
            network_configuration: network_configuration.map(|d| d.to_rusoto()),
            platform_version: platform_version.map(str::to_string),
            ..Default::default()
        };

        let result = try!(self.ecs_client().run_task(req).sync());
        info!("Completed to run task successfully");

        debug!("{:?}", result);

        let failure = result
            .failures
            .as_ref()
            .and_then(|failures| failures.first())
            .map(|f| f.to_owned());
        match result.tasks.as_ref().and_then(|tasks| tasks.first()) {
            Some(task) => Ok(TaskDescription {
                task: Some(task.to_owned()),
                failure: failure,
            }),
            None => Err(Box::new(CommandError::Unknown)),
        }
    }

    fn detect_task_definition_changes(
        &self,
        task_definition_conf: &config::ecs::TaskDefinition,
        current_task_definitions: &rusoto_ecs::TaskDefinition,
    ) -> bool {
        if current_task_definitions.family.is_none()
            || current_task_definitions.family.as_ref().unwrap().as_str()
                != task_definition_conf.family.as_str()
        {
            return true;
        }
        // if current_task_definitions.task_role_arn.is_none() || current_task_definitions.task_role_arn.as_ref().unwrap().as_str() != task_definition_conf.task_role_arn.as_str() {
        //   return true
        // }
        // if current_task_definitions.network_mode.is_none() || current_task_definitions.network_mode.as_ref().unwrap().as_str() != task_definition_conf.network_mode.as_str() {
        //   return true
        // }

        // TODO: detect difference between config of container definition and current one
        true

        //false
    }
}
