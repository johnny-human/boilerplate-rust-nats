# boilerplate-rust-nats

This is a boilerplate that is set up to communicate with a [NATS](https://nats.io/) server. It handles multiple clients dockerized into separate images.

## Prerequisite

* Docker
* Rust

We need a NATS server to run the clients.
```
docker pull nats
```
And then we run it
```
docker run nats
```
More about NATS and Docker can be found here [docs.nats.io/nats-server/nats_docker](https://docs.nats.io/nats-server/nats_docker)

## Build project
There is no main application here, but we can build all binaries with:
```
cargo build --workspace
```

## Run project
First we need to build the Docker images by:
```
docker-compose build
```

Then we first run the client that is listening for messages from the NATS server:
```
docker run --network="host" [name of client image]
```
Then we do the same with the client that send message:
```
docker run --network="host" [name of hello image]
```

What to expect? No much in this boilerplate, but the client will print a `Vec<u8>` representing the string "Hello World" when the hello binary is run. 


