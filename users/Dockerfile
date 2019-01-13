FROM lambci/lambda:build-provided

ARG COMMIT_HASH
ARG AWS_SECRET_ACCESS_KEY
ARG AWS_ACCESS_KEY_ID
ARG AWS_DEFAULT_REGION

# set aws credentials
ENV AWS_SECRET_ACCESS_KEY=${AWS_SECRET_ACCESS_KEY}
ENV AWS_ACCESS_KEY_ID=${AWS_ACCESS_KEY_ID}
ENV AWS_DEFAULT_REGION=${AWS_DEFAULT_REGION}

# install rust - nightly
RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly -y
ENV PATH="$PATH:/root/.cargo/bin"

# add clippy
RUN rustup self update
RUN rustup component add clippy --toolchain=nightly \
|| cargo install --git https://github.com/rust-lang/rust-clippy/ --force clippy

# setup directory
RUN mkdir /app
COPY . /app
WORKDIR /app

# build
RUN cargo clippy --all-targets --all-features -- -D warnings && cargo build --release

# bundle
RUN mkdir /bundles-${COMMIT_HASH} && zip -j /bundles-${COMMIT_HASH}/build-${COMMIT_HASH}.zip target/release/bootstrap

# Copy from builder and push to s3
RUN aws s3 cp /bundles-${COMMIT_HASH}/build-${COMMIT_HASH}.zip s3://lambda-rust-builds/build-${COMMIT_HASH}.zip
