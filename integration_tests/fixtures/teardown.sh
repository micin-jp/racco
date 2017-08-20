#!/usr/bin/env bash

set -e

DIR=$(cd $(dirname $BASH_SOURCE); pwd)
cd ${DIR}

echo "Tear down test fixtures."


## Delete services to prevent race condition of destroying the cluster
aws ecs update-service --cluster=racco-test-cluster --service=racco-test-web --desired-count 0 || true
aws ecs delete-service --cluster=racco-test-cluster --service=racco-test-web || true

## Terraform

cd ./terraform
terraform destroy -force
