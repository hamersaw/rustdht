extern crate bincode;
extern crate capnp;
extern crate rustc_serialize;

pub mod event;
pub mod message;
pub mod service;

pub mod message_capnp {
    include!(concat!(env!("OUT_DIR"), "/message_capnp.rs"));
}

#[test]
fn base() {

}
