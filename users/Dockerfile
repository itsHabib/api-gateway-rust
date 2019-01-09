# TODO: File issue on clippy repo about compiler errors

FROM lambci/lambda:build-provided
ARG commit_hash 

# install rust - nightly
RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly -y
ENV PATH="$PATH:/root/.cargo/bin"
#RUN rustup self update
#RUN rustup component add clippy 
#RUN rustup component add clippy --toolchain=nightly \
#|| cargo install --git https://github.com/rust-lang/rust-clippy/ --force clippy

# setup directory
RUN mkdir /app
COPY . /app
WORKDIR /app

# build
#RUN cargo-clippy -- -D clippy:all
RUN cargo build --release

# bundle
RUN mkdir "/bundles-${commit_hash}"
RUN zip -j /bundles-${commit_hash}/rust.zip target/release/bootstrap

