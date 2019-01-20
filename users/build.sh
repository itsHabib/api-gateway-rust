#!/bin/bash

COMMIT_HASH=$(git rev-parse --short HEAD)

if [[ $1 == "--api" ]]
  then
    deploy_type="api"
    file_cmd="-f Dockerfile.api"
    echo "creating plan for api"
  else
    echo "creating plan for lambda"
fi

# assumes AWS_* environment variables are set
docker build ${file_cmd:-} -t rust-lambda:${deploy_type:-latest}  \
--build-arg COMMIT_HASH=$COMMIT_HASH \
--build-arg AWS_DEFAULT_REGION=$AWS_DEFAULT_REGION \
--build-arg AWS_ACCESS_KEY_ID=$AWS_ACCESS_KEY_ID \
--build-arg AWS_SECRET_ACCESS_KEY=$AWS_SECRET_ACCESS_KEY .

echo "complete!"

