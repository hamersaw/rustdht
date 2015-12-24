use std::collections::BTreeMap;
use std::net::SocketAddrV4;

pub enum Event {
    LookupTableMsgEvent(BTreeMap<u64, SocketAddrV4>),
    RemoveNodeEvent(u64, SocketAddrV4),
    RegisterNodeEvent(u64, SocketAddrV4),
}
