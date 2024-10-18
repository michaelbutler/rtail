FROM rust:1.81 AS builder

# This Dockerfile is really just for compiling and then running the tests
# For this to work, the bats modules must be checked out as such:
#    git clone https://github.com/bats-core/bats-core.git test/bats && \
#    git clone https://github.com/bats-core/bats-support.git test/test_helper/bats-support && \
#    git clone https://github.com/bats-core/bats-assert.git test/test_helper/bats-assert

WORKDIR /code

COPY . .

RUN cargo build --target-dir /code/targetdocker --release && \
    strip targetdocker/release/rtail && \
    cp targetdocker/release/rtail /usr/bin/

ENTRYPOINT ["/code/test/bats/bin/bats"]
