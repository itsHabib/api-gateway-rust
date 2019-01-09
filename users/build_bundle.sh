#!/bin/bash

commit_hash=$(git rev-parse --short HEAD)
project_dir=$(pwd)
rm -f rust.zip

if [ "$1" = --keep-release ]; then
    echo "bundling project.."
    # temporarily rename dockerignore
    mv -f .dockerignore .dockerignore-$commit_hash
    docker build  -t rust-lambda:bundle --build-arg commit_hash=$commit_hash .
    mv -f .dockerignore-$commit_hash .dockerignore

    echo "retrieving bundle and release dir.."
    mkdir -p $project_dir/target/release # just in case
    # copy release dir and bundle
    id=$(docker run -d rust-lambda:bundle tail -f /dev/null)
    docker cp $id:/bundles-$commit_hash/rust.zip $project_dir
    docker cp $id:/app/target/release $project_dir/target/
    docker stop $id && docker rm $id

    echo "complete!"

else
    echo "bundling project from scratch.."
    docker build -t rust-lambda:bundle --build-arg commit_hash=$commit_hash .

    echo "retrieving bundle.."
    id=$(docker run -d rust-lambda:bundle tail -f /dev/null)
    docker cp $id:/bundles-$commit_hash/rust.zip $project_dir
    docker stop $id && docker rm $id

    echo "complete!"
fi

