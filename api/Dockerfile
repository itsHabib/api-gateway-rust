FROM lambci/lambda:build-provided as bundler

ARG AWS_SECRET_ACCESS_KEY
ARG AWS_ACCESS_KEY_ID
ARG AWS_DEFAULT_REGION
ARG PROJECT_FOLDERS

# set aws credentials
ENV AWS_SECRET_ACCESS_KEY=${AWS_SECRET_ACCESS_KEY}
ENV AWS_ACCESS_KEY_ID=${AWS_ACCESS_KEY_ID}
ENV AWS_DEFAULT_REGION=${AWS_DEFAULT_REGION}

#RUN yum install glibc -y
#binutils openssl libssl-dev wget autoconf automake libtool curl make g++ unzip cmake  -y


# install rust - nightly
RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly-2019-02-22 -y
ENV PATH="$PATH:/root/.cargo/bin"

# add clippy
#RUN rustup self update
#RUN rustup component add clippy --toolchain=nightly \
#|| cargo install --git https://github.com/rust-lang/rust-clippy/ --force clippy

# setup directory
RUN mkdir /app
COPY . /app
WORKDIR /app

# build crane
RUN cd crane && cargo build --release && cd -

# install crane
#RUN cargo install --git https://github.com/itsHabib/crane --branch develop

# build
#RUN cargo clippy --all-targets --all-features -- -D warnings && cargo build --release

RUN mkdir /bundles
# bundle
RUN ./crane/target/release/crane build ---project-folders ${PROJECT_FOLDERS} -w .

# Plan update
FROM hashicorp/terraform:light

ARG COMMIT_HASH
ARG AWS_SECRET_ACCESS_KEY
ARG AWS_ACCESS_KEY_ID
ARG AWS_DEFAULT_REGION

ENV AWS_ACCESS_KEY_ID=${AWS_ACCESS_KEY_ID}
ENV AWS_SECRET_ACCESS_KEY=${AWS_SECRET_ACCESS_KEY}
ENV AWS_DEFAULT_REGION=${AWS_DEFAULT_REGION}

COPY --from=bundler /bundles /bundles
COPY --from=bundler /app/terraform /terraform
# only for development
COPY --from=bundler /app/ /app

# Create deploy plan
WORKDIR /terraform
RUN terraform init
RUN terraform plan \
-refresh=true \
-out=terraform.tfplan

