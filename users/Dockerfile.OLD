#FROM lambci/lambda:build-provided

## install rust - nightly
#RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain j#ENV PATH="$PATH:/root/.cargo/bin"
#RUN cargo build --release

FROM lambci/lambda:build-provided

# install rust - nightly
RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly -y
ENV PATH="$PATH:/root/.cargo/bin"

# setup directory
RUN mkdir /app
COPY . /app
WORKDIR /app


