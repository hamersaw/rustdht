extern crate capnp;

pub mod event;
pub mod service;
pub mod dht_msg_capnp {
    include!(concat!(env!("OUT_DIR"), "/dht_msg_capnp.rs"));
}
