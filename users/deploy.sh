#!/bin/bash
COMMIT_HASH=$(git rev-parse --short HEAD)

if [[ $1 == "--api" ]]; then
    deploy_type="api"

elif [[ $1 == "--state" ]]; then
    echo "deploying state storage"
    docker run --rm -v `pwd`/terraform/state-storage:/app \
    -e AWS_ACCESS_KEY_ID=$AWS_ACCESS_KEY_ID \
    -e AWS_SECRET_ACCESS_KEY=$AWS_SECRET_ACCESS_KEY \
    --entrypoint "/bin/sh" \
    hashicorp/terraform:light \
    -c "cd app && terraform init && terraform apply -auto-approve"
    exit $?
fi

echo "updating code.."
docker run --rm rust-lambda:${deploy_type:-latest} apply "terraform.$COMMIT_HASH.tfplan"
