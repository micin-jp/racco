#!/usr/bin/env bash

set -e

DIR=$(cd $(dirname $BASH_SOURCE); pwd)
cd ${DIR}

echo "Setup test fixtures."


AWS_ID=$(aws sts get-caller-identity |jq -r '.Account')
if [ -z "$AWS_ID" ]; then
	exit 1
fi


## Terraform

cd "${DIR}/terraform"
#terraform plan
terraform apply

## Docker images

ECR_LOGIN=$(aws ecr get-login --no-include-email)
$ECR_LOGIN

DOCKER_IMAGES=("nginx" "echo")

for image in "${DOCKER_IMAGES[@]}"; do

	cd "${DIR}/docker/${image}"

	name="racco-test/${image}"
	remote_name="${AWS_ID}.dkr.ecr.ap-northeast-1.amazonaws.com/${name}"
	tag="latest"
	docker build -t ${remote_name} .
	docker push ${remote_name}:${tag}
done


cd "${DIR}/configs"
./generate.sh

