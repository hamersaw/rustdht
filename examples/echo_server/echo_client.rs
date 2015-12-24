extern crate argparse;
use argparse::{ArgumentParser,Store};

extern crate capnp;

pub mod message_capnp {
    include!(concat!(env!("OUT_DIR"), "/message_capnp.rs"));
}

use std::io;
use std::io::prelude::*; //needed for flushing stdout
use std::hash::{Hash,Hasher,SipHasher};
use std::net::{Ipv4Addr,SocketAddrV4,TcpStream};
use std::str::FromStr;

fn main() {
    let mut host_ip: String = "127.0.0.1".to_string();
    let mut host_port: u16 = 0;
    {    //solely to limit scope of parser variable
        let mut parser = ArgumentParser::new();
        parser.set_description("Start up an echo server client");
        parser.refer(&mut host_ip).add_option(&["-i", "--host-ip"], Store, "Ip address of the host to connect to").required();
        parser.refer(&mut host_port).add_option(&["-p", "--host-port"], Store, "Port of the host to connect to").required();
        parser.parse_args_or_exit();
    }
   
    //parse host address
    let host_ip = Ipv4Addr::from_str(&host_ip[..]).unwrap();
    let host_addr = SocketAddrV4::new(host_ip, host_port);

    //read user input
    let stdin = io::stdin();
    let mut line = String::new();
    loop {
        print!("Enter input: ");
        std::io::stdout().flush().ok(); //future versions of rust will fix this need

        line.clear();
        stdin.read_line(&mut line).ok();
        let echo_msg = line.trim();

        //compute hash of line
        let mut hasher = SipHasher::new();
        echo_msg.hash(&mut hasher);
        let token = hasher.finish();

        //create insert entity message
        let mut msg_builder = capnp::message::Builder::new_default();
        {
            let msg = msg_builder.init_root::<message_capnp::echo_server_msg::Builder>();
            let mut client_echo_msg = msg.get_msg_type().init_client_echo_msg();
            client_echo_msg.set_token(token);
            client_echo_msg.set_msg(echo_msg);
        }

        //send insert entity message
        let mut stream = TcpStream::connect(host_addr).unwrap();
        capnp::serialize::write_message(&mut stream, &msg_builder).unwrap();
    }
}
