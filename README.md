
# Racco

[![Build Status](https://travis-ci.org/micin-jp/racco.svg?branch=master)](https://travis-ci.org/micin-jp/racco)

AWS ECS Deployment toolbox.

## Install

```
brew tap micin-jp/racco
brew install racco
```

## Commands

### `deploy`

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

### Run

```racco deploy```

### `run-task`

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

### Run

```racco run-task [task-name]```

### `schedule`

WIP

### `params`

WIP


## Related projects

- https://github.com/eagletmt/hako
- https://github.com/silinternational/ecs-deploy
