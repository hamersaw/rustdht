use std::net::{Ipv4Addr,SocketAddrV4};
use std::str::FromStr;

pub trait Message {
    fn print(&self);
}

pub enum MessageType {
    ResultMsg = 0,
    JoinMsg = 1,
    LookupMsg = 2,
    AddrMsg = 3,
    RegisterTokenMsg = 4,
    GenericMsg = 255,
}

#[derive(RustcEncodable,RustcDecodable)]
pub struct ResultMsg {
    success: bool,
    error: Option<String>,
}

impl ResultMsg {
    pub fn new(success: bool, error: Option<String>) -> Self {
        ResultMsg {
            success: success,
            error: error,
        }
    }

    pub fn get_success(&self) -> bool {
        self.success.clone()
    }
}

impl Message for ResultMsg {
    fn print(&self) {
        println!("Result message success: {}", self.success);
        match self.error {
            Some(ref msg) => println!("Error messasge: {}", msg),
            None => {} ,
        }
    }
}

#[derive(RustcEncodable,RustcDecodable)]
pub struct JoinMsg {
    id: String,
    token: u64,
    ip: String,
    port: u16,
}

impl JoinMsg {
    pub fn new(id: String, token: u64, socket_addr: SocketAddrV4) -> Self {
        let ip: String = format!("{}", socket_addr.ip());

        JoinMsg {
            id: id,
            token: token,
            ip: ip,
            port: socket_addr.port(),
        }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_token(&self) -> u64 {
        self.token.clone()
    }

    pub fn get_socket_addr(&self) -> SocketAddrV4 {
        let ip_addr = Ipv4Addr::from_str(&self.ip[..]).unwrap();
        SocketAddrV4::new(ip_addr, self.port)
    }
}

impl Message for JoinMsg {
    fn print(&self) {
        println!("id: {}\ntoken: {}\nip: {}\nport: {}", self.id, self.token, self.ip, self.port);
    }
}

#[derive(RustcEncodable,RustcDecodable)]
pub struct LookupMsg {
    token: u64,
}

impl LookupMsg {
    pub fn new(token: u64) -> Self {
        LookupMsg { token: token }
    }

    pub fn get_token(&self) -> u64 {
        self.token.clone()
    }
}

impl Message for LookupMsg {
    fn print(&self) {
        println!("token: {}", self.token);
    }
}

#[derive(RustcEncodable,RustcDecodable)]
pub struct AddrMsg {
    ip: String,
    port: u16,
}

impl AddrMsg {
    pub fn new(socket_addr: SocketAddrV4) -> Self {
        let ip: String = format!("{}", socket_addr.ip());

        AddrMsg {
            ip: ip,
            port: socket_addr.port(),
        }
    }

    pub fn get_socket_addr(&self) -> SocketAddrV4 {
        let ip_addr = Ipv4Addr::from_str(&self.ip[..]).unwrap();
        SocketAddrV4::new(ip_addr, self.port)
    }
}

impl Message for AddrMsg {
    fn print(&self) {
        println!("ip: {}\nport: {}", self.ip, self.port);
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    extern crate bincode;
    use bincode::rustc_serialize::{decode,encode};

    use std::net::{Ipv4Addr,SocketAddrV4};
    use std::str::FromStr;

    #[test]
    fn serialize_result_msg() {
        let result_msg = ResultMsg::new(true, None);
        let encoded = encode(&result_msg, bincode::SizeLimit::Infinite).unwrap();
        let _: ResultMsg = decode(&encoded[..]).unwrap();
    }

    #[test]
    fn serialize_join_msg() {
        let ip = Ipv4Addr::from_str("127.0.0.1").unwrap();
        let socket_addr = SocketAddrV4::new(ip, 0 as u16);

        let join_msg = JoinMsg::new("test".to_string(), 0 as u64, socket_addr);
        let encoded = encode(&join_msg, bincode::SizeLimit::Infinite).unwrap();
        let _: JoinMsg = decode(&encoded[..]).unwrap();
    }

    #[test]
    fn serialize_lookup_msg() {
        let lookup_msg = LookupMsg::new(0 as u64);
        let encoded = encode(&lookup_msg, bincode::SizeLimit::Infinite).unwrap();
        let _: LookupMsg = decode(&encoded[..]).unwrap();
    }

    #[test]
    fn serialize_addr_msg() {
        let ip = Ipv4Addr::from_str("127.0.0.1").unwrap();
        let socket_addr = SocketAddrV4::new(ip, 0 as u16);

        let addr_msg = AddrMsg::new(socket_addr);
        let encoded = encode(&addr_msg, bincode::SizeLimit::Infinite).unwrap();
        let _: AddrMsg = decode(&encoded[..]).unwrap();
    }
}
