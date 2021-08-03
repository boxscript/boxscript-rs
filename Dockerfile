FROM rust:alpine

WORKDIR /usr/src/bs

ADD Cargo.* /usr/src/bs
COPY ./src /usr/src/bs/src
RUN cargo build --release
RUN cargo install --path .

ENTRYPOINT ["/usr/src/bs/target/release/boxscript"]
