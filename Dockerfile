FROM rust:alpine

WORKDIR /usr/src/bs

RUN apk add llvm12

COPY Cargo.* /usr/src/bs
COPY ./src /usr/src/bs/src

RUN cargo build --release

WORKDIR /var/tmp

ENTRYPOINT ["/usr/src/bs/target/release/boxscript"]
