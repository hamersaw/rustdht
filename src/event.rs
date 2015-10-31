use std::net::{SocketAddrV4,TcpStream};

pub enum Event {
    GenericMsgEvent(Vec<u8>, TcpStream),
    JoinMsgEvent(String, u64, String, u16),
    LookupMsgEvent(u64),
    RegisterTokenMsg(u64, SocketAddrV4),
}
