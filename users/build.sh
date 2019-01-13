#!/bin/bash

COMMIT_HASH=$(git rev-parse --short HEAD)
rm -f rust.zip

echo "bundling project.."

docker build -t rust-lambda:bundle --build-arg COMMIT_HASH=$commit_hash \
--build-arg AWS_DEFAULT_REGION=$AWS_DEFAULT_REGION \
--build-arg AWS_ACCESS_KEY_ID=$AWS_ACCESS_KEY_ID \
--build-arg AWS_SECRET_ACCESS_KEY=$AWS_SECRET_ACCESS_KEY .

echo "complete!"

