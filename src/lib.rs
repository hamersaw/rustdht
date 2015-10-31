extern crate capnp;

pub mod omniscient;
pub mod omniscient_msg_capnp {
    include!(concat!(env!("OUT_DIR"), "/omniscient_msg_capnp.rs"));
}

#[test]
fn base() {

}
