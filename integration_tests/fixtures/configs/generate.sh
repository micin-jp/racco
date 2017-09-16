#!/usr/bin/env bash

set -e

DIR=$(cd $(dirname $BASH_SOURCE); pwd)
cd ${DIR}


# export variables


cd "${DIR}/../terraform"

export AWS_REGION="$(terraform output aws_region)"
export SERVICE_ROLE_ARN="$(terraform output cluster_service_role_arn)"
export LB_TARGET_GROUP_ARN="$(terraform output alb_target_group_arn)"
export TASK_ROLE_ARN="$(terraform output task_role_arn)"
export NGINX_IMAGE="$(terraform output repository_url_nginx):latest"
export ECHO_IMAGE="$(terraform output repository_url_echo):latest"
export AWSLOGS_GROUP="$(terraform output awslogs_group)"
export EVENTS_TARGET_ROLE_ARN="$(terraform output events_target_role_arn)"


# generate templates

cd "${DIR}"

TEMPLATES=("service_deploy.yml" "run_task.yml" "schedule_task.yml")

for tmpl in "${TEMPLATES[@]}"; do
  cat "$tmpl.template" | envsubst > $tmpl
done


