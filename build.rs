extern crate capnpc;

fn main() {
    ::capnpc::compile(".", &["src/zero_hop/zero_hop_msg.capnp"]).unwrap();
    println!("Succesfully compiled capnproto files");
}
