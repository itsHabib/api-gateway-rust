#!/bin/bash
COMMIT_HASH=$(git rev-parse --short HEAD)

if [[ $1 == "--api" ]]
  then
    deploy_type="api"
fi

echo "updating code.."
docker run --rm rust-lambda:${deploy_type:-latest} apply "terraform.$COMMIT_HASH.tfplan"
