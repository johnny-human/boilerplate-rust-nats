use nats::*;

fn main() {
    let nc = nats::connect("localhost:4222").unwrap();
    nc.publish("msg", "Hello World");
}