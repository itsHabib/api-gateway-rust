FROM lambci/lambda:build-provided as bundler

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
RUN mkdir /bundles && zip -j /bundles/bundle.zip target/release/bootstrap

# Plan update
FROM hashicorp/terraform:light

ARG COMMIT_HASH
ARG AWS_SECRET_ACCESS_KEY
ARG AWS_ACCESS_KEY_ID
ARG AWS_DEFAULT_REGION

ENV AWS_ACCESS_KEY_ID=${AWS_ACCESS_KEY_ID}
ENV AWS_SECRET_ACCESS_KEY=${AWS_SECRET_ACCESS_KEY}
ENV AWS_DEFAULT_REGION=${AWS_DEFAULT_REGION}

COPY --from=bundler /bundles/bundle.zip /bundles/bundle.zip
COPY --from=bundler /app/terraform /terraform

# Create deploy plan
WORKDIR /terraform
RUN terraform init
RUN terraform plan \
-target=aws_lambda_function.users \
-out=terraform.${COMMIT_HASH}.tfplan \
-refresh=true \
-var="tag=${COMMIT_HASH}"

