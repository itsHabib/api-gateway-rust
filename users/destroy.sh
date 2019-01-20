#!/bin/bash
COMMIT_HASH=$(git rev-parse --short HEAD)

if [[ $1 == "--state" ]]; then
    echo "destroying state storage"
    docker run --rm -v `pwd`/terraform/state-storage:/app \
    -e AWS_ACCESS_KEY_ID=$AWS_ACCESS_KEY_ID \
    -e AWS_SECRET_ACCESS_KEY=$AWS_SECRET_ACCESS_KEY \
    --entrypoint "/bin/sh" \
    hashicorp/terraform:light \
    -c "cd app && terraform init && terraform destroy  -auto-approve"
    exit $?
fi

echo "destroying lambda and api"
docker run --rm -v `pwd`/terraform/:/app \
-e AWS_ACCESS_KEY_ID=$AWS_ACCESS_KEY_ID \
-e AWS_SECRET_ACCESS_KEY=$AWS_SECRET_ACCESS_KEY \
--entrypoint "/bin/sh" \
hashicorp/terraform:light \
-c "cd app && terraform init && terraform destroy -var='tag=${COMMIT_HASH}' -auto-approve"

