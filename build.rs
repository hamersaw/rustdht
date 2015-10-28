extern crate capnpc;

fn main() {
    ::capnpc::compile("capnp", &["capnp/join.capnp"]).unwrap();
}
