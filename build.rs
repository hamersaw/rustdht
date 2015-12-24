extern crate capnpc;

fn main() {
    ::capnpc::compile(".", &["src/dht_msg.capnp"]).unwrap();
    
    //temporarily building echo_server messages example here
    ::capnpc::compile(".", &["examples/echo_server/message.capnp"]).unwrap();

    println!("Succesfully compiled capnproto files");
}
