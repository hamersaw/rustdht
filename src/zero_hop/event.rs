use std::collections::BTreeMap;
use std::net::SocketAddrV4;

pub enum Event {
    LookupTableMsgEvent(BTreeMap<u64, SocketAddrV4>),
    RegisterTokenMsgEvent(u64, SocketAddrV4),
}
