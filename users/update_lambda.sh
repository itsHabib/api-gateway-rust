#!/bin/bash

echo "updating code.."
aws lambda update-function-code --function-name $1 --zip-file fileb://rust.zip

