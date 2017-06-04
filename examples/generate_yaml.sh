#!/bin/sh

set -eux


cat << YAML > racco.yml
deploy:
  - cluster: 'racco-cluster'
    service:
      name: 'racco-web'
      desired_count: 4
      deployment_configuration:
        maximum_percent: 200
        minimum_healthy_percent: 100
      role: '$(cd ./aws/terraform && terraform output cluster_service_role_arn)'
      load_balancers:
        - target_group_arn: '$(cd ./aws/terraform && terraform output alb_target_group_arn)'
          container_name: "nginx"
          container_port: 80
      task_definition:
        family: 'racco-web'
        task_role_arn: '$(cd ./aws/terraform && terraform output task_role_arn)'
        network_mode: bridge
        container_definitions:
          - name: nginx
            image: '$(cd ./aws/terraform && terraform output repository_url_nginx):latest'
            cpu: 20
            memory: 128
            port_mappings:
              - container_port: 80
            log_configuration:
              log_driver: awslogs
              options:
                awslogs-group: '$(cd ./aws/terraform && terraform output awslogs_group)'
                awslogs-region: 'ap-northeast-1'
                awslogs-stream-prefix: 'racco-web-nginx'
run_task:
  - cluster: 'racco-cluster'
    task_definition:
      family: 'racco-job'
      task_role_arn: '$(cd ./aws/terraform && terraform output task_role_arn)'
      network_mode: bridge
      container_definitions:
        - name: echo
          image: '$(cd ./aws/terraform && terraform output repository_url_echo):latest'
          cpu: 20
          memory: 128
          log_configuration:
            log_driver: awslogs
            options:
              awslogs-group: '$(cd ./aws/terraform && terraform output awslogs_group)'
              awslogs-region: 'ap-northeast-1'
              awslogs-stream-prefix: 'racco-job-echo'
params:
  path: 'ecs0'


YAML
