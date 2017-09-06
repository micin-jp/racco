
# Racco

[![Build Status](https://travis-ci.org/micin-jp/racco.svg?branch=master)](https://travis-ci.org/micin-jp/racco)

A deployment toolkit for AWS ECS. Racco runs deployment process by a configuration file user defined.

## Install

### macOS

```
brew tap micin-jp/racco
brew install racco
```

### Linux

From the [release page](https://github.com/micin-jp/racco/releases), download ZIP file. Unarchive it, and put the binary to somewhere you want.

Or, executing install script, you can install like below:

```
curl -sL https://raw.githubusercontent.com/micin-jp/racco/master/install.sh | sudo sh
```

The binary will be put under `/usr/local/bin`.

## Usage

Racco has 4 sub commands `deploy`, `run-task`, `schedule-task` and `params`. To execute the commands, a configuration file named `racco.yml` is needed.

While Racco deploys applications to ECS by manipulating AWS resources, some resources are required to be provisioned beforehand.
For example, to execute `deploy`, an ECS cluster required to be created. Specifying the cluster name, you can deploy ECS services on it. Creating and updating services are executed by Racco self.

See AWS documents for details of [ECS](http://docs.aws.amazon.com/AmazonECS/latest/developerguide/Welcome.html).

### Deploy

```
racco deploy [NAME]
```

This command updates ECS services. The section of `deploy` in the configuration file, has a service definition and a task definition to be run on the service.

Executing the command, a new task definition will be created, and update the service with its task definition. If there is no service, a new service will be created.

#### Required AWS Resources

- ECS cluster
- ELB (optional)
- IAM role for ECS Service (optional, required if you are using ELB)
- IAM role for ECS Task
- ECR repository for docker image (optional)

#### Example Configuration

```yml:racco.yml
deploy:
  - name: racco-web
    cluster: racco-cluster
    service:
      name: racco-web
      desired_count: 4
      deployment_configuration:
        maximum_percent: 200
        minimum_healthy_percent: 100
      role: 'arn:aws:iam::XXXXXXXXX:role/racco-nginx-role'
      load_balancers:
        - target_group_arn: 'arn:aws:elasticloadbalancing:ap-northeast-1:XXXXXXXX:targetgroup/racco-web/XXXXXXXXXXX'
          container_name: "nginx"
          container_port: 80
      task_definition:
        family: 'racco-web'
        task_role_arn: 'arn:aws:iam::XXXXXXXXXX:role/racco-web-task-role'
        network_mode: bridge
        container_definitions:
          - name: nginx
            image: 'XXXXXXXXXXX.dkr.ecr.ap-northeast-1.amazonaws.com/racco/nginx:latest'
            cpu: 20
            memory: 128
            port_mappings:
              - container_port: 80
            log_configuration:
              log_driver: awslogs
              options:
                awslogs-group: 'racco'
                awslogs-region: 'ap-northeast-1'
                awslogs-stream-prefix: 'racco-web-nginx'
```

### Run task

```
racco run-task [NAME]
```

This command executes a given task.

#### Required AWS Resources

- ECS cluster
- IAM role for ECS Task
- ECR repository for docker image (optional)

#### Example Configuration

```yml:racco.yml
run_task:
  - name: racco-job
    cluster: racco-cluster
    task_definition:
      family: racco-job
      task_role_arn: 'arn:aws:iam::XXXXXXXXX:role/racco-job-task-role'
      network_mode: bridge
      container_definitions:
        - name: echo
          image: 'XXXXXXXXXXX.dkr.ecr.ap-northeast-1.amazonaws.com/racco/echo:latest'
          cpu: 20
          memory: 128
          log_configuration:
            log_driver: awslogs
            options:
              awslogs-group: 'racco'
              awslogs-region: 'ap-northeast-1'
              awslogs-stream-prefix: 'racco-job-echo'
```

### Schedule task

This command sets scheduled tasks.

It creates a rule invoked at certain times in CloudWatch events. A task definition is triggered as a target of the rule.

```
racco schedule-task [NAME]
```

#### Required AWS Resources

- ECS cluster
- IAM role for ECS Task
- ECR repository for docker image (optional)
- IAM role to run ECS Task from CloudWatch Events

#### Example Configuration

```yml:racco.yml
schedule_task:
  - name: racco-schedule-job
    cluster: 'racco-cluster'
    rule:
      name: racco-schedule-job-rule
      schedule_expression: 'cron(0/5 * * * ? *)'
    rule_targets_role_arn: 'arn:aws:iam::XXXXXXXXXX:role/racco-events-target-role'
    task_definition:
      family: 'racco-schedule-job'
      task_role_arn: 'arn:aws:iam::XXXXXXXXXX:role/racco-task-role'
      network_mode: bridge
      container_definitions:
        - name: echo
          image: 'XXXXXXXXXX.dkr.ecr.ap-northeast-1.amazonaws.com/racco/echo:latest'
          cpu: 20
          memory: 128
```


### Params

```
racco params get [NAME]
racco params list
racco params put [NAME] [VALUE]
racco params delete [NAME]
racco params exec [PROGRAM] [ARGS]
```

`params` command manages parameters used at container runtime. Parameters are stored SSM Parameter Store. Using KMS, you can manage secrets.

Developers execute `params put`, and store parameters.

Using `params get` or `params exec`, you can get the stored parameters.
`params exec` expands the parameters in environment variables, and execute a given command.

#### Required AWS Resources

- KMS key (optional, if you use SecuredString)

#### Example Configuration

```yml:racco.yml
params:
  path: 'racco-params'
  secure:
    key: 'XXXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX'
```


## Related projects

- https://github.com/eagletmt/hako
- https://github.com/silinternational/ecs-deploy
