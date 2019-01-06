#!/bin/bash

# TODO: look into building the zip inside the container

echo "building image.."
docker build -t rust-lambda .

if [ "$1" = --scratch ]; then
    echo "building project from scratch"
    # don't run container with a volume mounted
    id=$(docker run -d rust-lambda /bin/bash -c "cargo build --release && tail -f /dev/null")
    until [ -z  "$(docker exec $id ps -e | egrep 'rust|cargo' | awk '{ print $0 }')" ]
    do
        sleep 1;
    done
    echo "build complete"

    # copy bootstrap from container into host
    mkdir -p target/release # just in case
    docker cp $id:/app/target/release/bootstrap target/release/bootstrap
    docker stop $id && docker rm $id
    exit 0
fi

echo "building with target mounted"
docker run --rm -v `pwd`/target:/app/target rust-lambda cargo build --release

