#!/bin/bash
COMMIT_HASH=$(git rev-parse --short HEAD)

echo "updating code.."
docker run --rm rust-lambda apply "terraform.$COMMIT_HASH.tfplan"
