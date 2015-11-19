extern crate capnp;

pub mod zero_hop;
pub mod zero_hop_msg_capnp {
    include!(concat!(env!("OUT_DIR"), "/zero_hop_msg_capnp.rs"));
}

#[test]
fn base() {

}
