#!/bin/bash


echo "updating code.."
aws lambda update-function-code --function-name $1 --s3-bucket lambda-rust-builds \
--s3-key build-$(git rev-parse --short HEAD)
