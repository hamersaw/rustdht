extern crate capnp;

pub mod zero_hop;
pub mod zero_hop_msg_capnp {
    include!(concat!(env!("OUT_DIR"), "/zero_hop_msg_capnp.rs"));
}

pub mod gossiping;
pub mod gossiping_msg_capnp {
    include!(concat!(env!("OUT_DIR"), "/gossiping_msg_capnp.rs"));
}
