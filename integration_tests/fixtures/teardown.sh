#!/usr/bin/env bash

set -e

DIR=$(cd $(dirname $BASH_SOURCE); pwd)
cd ${DIR}

echo "Tear down test fixtures."

## Terraform

cd ./terraform
terraform destroy -force
