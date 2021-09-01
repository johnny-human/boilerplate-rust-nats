FROM rustlang/rust:nightly as build

RUN USER=root cargo new --bin client
WORKDIR /client

COPY ./Cargo.lock ./Cargo.lock
COPY ./client/Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./client/src ./src

RUN rm ./target/release/deps/client*
RUN cargo build --bin client --release

FROM debian:jessie-slim as final

COPY --from=build /client/target/release/client .

CMD ["./client"]