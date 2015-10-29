/*extern crate argparse;
extern crate bincode;
extern crate rustc_serialize;

pub mod event;
pub mod message;
pub mod service;

use argparse::{ArgumentParser,Store};

use event::Event;
use service::omniscient::OmniscientService;

use std::net::{Ipv4Addr,SocketAddrV4};
use std::str::FromStr;

fn main() {
    let mut id: String = "foo".to_string();
    let mut token: u64 = 0 as u64;
    let mut listen_ip: String = "127.0.0.1".to_string();
    let mut listen_port: u16 = 0 as u16;
    let mut seed_ip: String = "127.0.0.1".to_string();
    let mut seed_port: u16 = 0 as u16;
    {   //solely to limit scope of parser variable
        let mut parser = ArgumentParser::new();
        parser.set_description("Start up a gossiping bearbones node");
        parser.refer(&mut id).add_option(&["-i", "--id"], Store, "ID of node").required();
        parser.refer(&mut token).add_option(&["-t", "--token"], Store, "Token of node").required();
        parser.refer(&mut listen_ip).add_option(&["-l", "--listen-ip"], Store, "Ip address to listen on").required();
        parser.refer(&mut listen_port).add_option(&["-p", "--listen-port"], Store, "Port to listen on").required();
        parser.refer(&mut seed_ip).add_option(&["-s", "--seed-ip"], Store, "Ip address for seed node");
        parser.refer(&mut seed_port).add_option(&["-e", "--seed-port"], Store, "Port for seed node");
        parser.parse_args_or_exit();
    }

    let listen_ip = match Ipv4Addr::from_str(&listen_ip[..]) {
        Ok(listen_ip) => listen_ip,
        Err(_) => panic!("Unable to parse ip '{}'", listen_ip),
    };

    let listen_addr = SocketAddrV4::new(listen_ip, listen_port);

    //parse seed ip address
    let mut seed_addr: Option<SocketAddrV4> = None;
    if seed_port != 0 {
        let seed_ip = match Ipv4Addr::from_str(&seed_ip[..]) {
            Ok(seed_ip) => seed_ip,
            Err(_) => panic!("Unable to parse ip {}", seed_ip),
        };

        seed_addr = Some(SocketAddrV4::new(seed_ip, seed_port));
    }

    //create new service handle
    let service_handle = OmniscientService::new(id, token, listen_addr, seed_addr);

    //start the service handle
    let (start_handle, rx) = service_handle.start();
    while let Ok(event) = rx.recv() {
        match event {
            Event::JoinMsgEvent(id, token, socket_addr) => {
                println!("recv JoinMsgEvent({}, {}, {})", id, token , socket_addr);
            },
            Event::LookupMsgEvent(token) => {
                println!("recv LookupMsgEvent({})", token);
            },
            Event::GenericMsgEvent(vec) => {
                println!("recv GenericMsgEvent() of length {}", vec.len());
            }
            _ => println!("not processing this event type"),
        }
    }
 
    start_handle.join().unwrap();
}*/

//TODO figure out a new way to parse out arguments other than argparse
fn main() {
    println!("Hello World!");
}
