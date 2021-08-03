FROM rust:alpine

WORKDIR /usr/src/bs

# uncomment this when llvm12 is available
# RUN apk add llvm12

# besides, the code doesn't even use llvm yet

COPY Cargo.* /usr/src/bs
COPY ./src /usr/src/bs/src

RUN cargo build --release

ENTRYPOINT ["/usr/src/bs/target/release/boxscript"]
