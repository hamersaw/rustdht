use std::net::{SocketAddrV4,TcpStream};

pub enum Event {
    JoinMsgEvent(String, u64, String, u16),
    LookupMsgEvent(u64),
    AddrMsgEvent(SocketAddrV4),
    GenericMsgEvent(Vec<u8>, TcpStream),
}
