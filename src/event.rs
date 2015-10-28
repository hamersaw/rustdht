use std::net::SocketAddrV4;

pub enum Event {
    JoinMsgEvent(String, u64, SocketAddrV4),
    LookupMsgEvent(u64),
    AddrMsgEvent(SocketAddrV4),
    GenericMsgEvent(Vec<u8>),
}
