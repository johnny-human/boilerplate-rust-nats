fn main() {
    let nc = nats::connect("localhost:4222").unwrap();
    let sub = nc.subscribe("msg").unwrap();

    for msg in sub.messages() {
        println!("{:?}", msg.data.to_owned());
    }
}