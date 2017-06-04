#!/bin/sh
set -eux

AWS_ID=$(aws sts get-caller-identity |jq -r '.Account')
if [ -z "$AWS_ID" ]; then
	exit 1
fi

TAG="latest"

LOGIN=$(aws ecr get-login --region='ap-northeast-1' --no-include-email)
$LOGIN

NAME="racco/nginx"
REMOTE_NAME="${AWS_ID}.dkr.ecr.ap-northeast-1.amazonaws.com/${NAME}"
docker tag ${NAME}:${TAG} ${REMOTE_NAME}:${TAG}
docker push ${REMOTE_NAME}:${TAG}

