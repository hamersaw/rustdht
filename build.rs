extern crate capnpc;

fn main() {
    ::capnpc::compile(".", &["src/omniscient/omniscient_msg.capnp"]).unwrap();
    println!("Succesfully compiled capnproto files");
}
