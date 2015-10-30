extern crate capnp;

pub mod event;
pub mod service;

pub mod message_capnp {
    include!(concat!(env!("OUT_DIR"), "/message_capnp.rs"));
}

#[test]
fn base() {

}
