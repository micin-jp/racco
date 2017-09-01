
# Racco

[![Build Status](https://travis-ci.org/micin-jp/racco.svg?branch=master)](https://travis-ci.org/micin-jp/racco)

AWS ECS Deployment toolbox.

## Install

### macOS

```
brew tap micin-jp/racco
brew install racco
```

### Linux

From the [release page](https://github.com/micin-jp/racco/releases), download ZIP file. Unarchive it, and put the binary to somewhere you want.

Or, you can install by running the install script like below (the binary is put under `/usr/local/bin`):

```
curl -sL https://raw.githubusercontent.com/micin-jp/racco/master/install.sh | sudo sh
```

## Commands

### `deploy`

```racco deploy [NAME]```

#### Setting up AWS Resources

AWS resources below are required to be provisioned before deployment:

- ECS cluster
- ELB (optional)
- IAM role for ECS Service (optional, required if you are using ELB)
- IAM role for ECS Task
- ECR repository for docker image (optional)

#### Example Configuration

Edit your `racco.yml`.

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

### `run-task`

```racco run-task [NAME]```

#### Setting up AWS Resources

AWS resources below are required to be provisioned before running task:

- ECS cluster
- IAM role for ECS Task
- ECR repository for docker image (optional)

#### Example Configuration

Edit your `racco.yml`.

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

### `schedule-task`

```
racco schedule-task put [NAME]
racco schedule-task delete [NAME]
```

#### Setting up AWS Resources

AWS resources below are required to be provisioned:

- ECS cluster
- IAM role for ECS Task
- ECR repository for docker image (optional)
- IAM role to run ECS Task from CloudWatch Events

#### Example Configuration

Edit your `racco.yml`.

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




### `params`

```
racco params get [NAMES]
racco params put [NAME] [VALUE]
racco params delete [NAME]
```

#### Setting up AWS Resources

AWS resources below are required to be provisioned:

- KMS key (optional, if you use SecuredString)

#### Example Configuration

Edit your `racco.yml`.

```yml:racco.yml
params:
  path: 'racco-params'
  secure:
    key: 'XXXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX'
```



## Related projects

- https://github.com/eagletmt/hako
- https://github.com/silinternational/ecs-deploy
