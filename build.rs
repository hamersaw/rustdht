extern crate capnpc;

fn main() {
    ::capnpc::compile("capnproto", &["capnproto/message.capnp"]).unwrap();
    println!("Succesfully compiled capnproto files");
}
