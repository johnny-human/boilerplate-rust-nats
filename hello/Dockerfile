FROM rustlang/rust:nightly as build

RUN USER=root cargo new --bin hello
WORKDIR /hello

COPY ./Cargo.lock ./Cargo.lock
COPY ./hello/Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./hello/src ./src

RUN rm ./target/release/deps/hello*
RUN cargo build --bin hello --release

FROM debian:jessie-slim as final

COPY --from=build /hello/target/release/hello .

CMD ["./hello"]