#!/bin/bash

rm -rf rust.zip
echo "zipping binary.."
zip -j rust.zip target/release/bootstrap

echo "updating code.."
aws lambda update-function-code --function-name $1 --zip-file fileb://rust.zip

