#!/usr/bin/env bash

set -e

DIR=$(cd $(dirname $BASH_SOURCE); pwd)
cd ${DIR}


# export variables


cd "${DIR}/../terraform"

export SERVICE_ROLE_ARN="$(terraform output cluster_service_role_arn)"
export LB_TARGET_GROUP_ARN="$(terraform output alb_target_group_arn)"
export TASK_ROLE_ARN="$(terraform output task_role_arn)"
export NGINX_IMAGE="$(terraform output repository_url_nginx):latest"

# generate templates

cd "${DIR}"

TEMPLATES=("deploy_service.yml")

for tmpl in "${TEMPLATES[@]}"; do
  cat "$tmpl.template" | envsubst > $tmpl
done


