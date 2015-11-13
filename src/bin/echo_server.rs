extern crate argparse;
use argparse::{ArgumentParser,Store};

extern crate rustp2p;
use rustp2p::omniscient::event::Event;
use rustp2p::omniscient::service::OmniscientService;

use std::net::{Ipv4Addr,SocketAddrV4};
use std::str::FromStr;

fn main() {
    println!("Hello Echo Server");
    let mut id = "World".to_string();
    let mut token: u64 = 0;
    let mut app_ip: String = "127.0.0.1".to_string();
    let mut app_port: u16 = 0;
    let mut service_port: u16 = 0;
    let mut seed_ip: String = "127.0.0.1".to_string();
    let mut seed_port: u16 = 0;
    {    //solely to limit scope of parser variable
        let mut parser = ArgumentParser::new();
        parser.set_description("start up a echo server");
        parser.refer(&mut id).add_option(&["-i", "--id"], Store, "id of node").required();
        parser.refer(&mut token).add_option(&["-t", "--token"], Store, "token of node").required();
        parser.refer(&mut app_ip).add_option(&["-l", "--listen-ip"], Store, "ip address for application and service to listen on").required();
        parser.refer(&mut app_port).add_option(&["-a", "--app-port"], Store, "port for application to listen on").required();
        parser.refer(&mut service_port).add_option(&["-p", "--service-port"], Store, "port for the p2p service listen on").required();
        parser.refer(&mut seed_ip).add_option(&["-s", "--seed-ip"], Store, "p2p service seed node ip address");
        parser.refer(&mut seed_port).add_option(&["-e", "--seed-port"], Store, "p2p service seed node port");
        parser.parse_args_or_exit();
    }

    //create application and service addresses
    let ip = Ipv4Addr::from_str(&app_ip[..]).unwrap();
    let app_addr = SocketAddrV4::new(ip, app_port);
    let service_addr = SocketAddrV4::new(ip, service_port);

    //create seed address
    let seed_addr = match seed_port {
        0 => None,
        _ => {
            let seed_ip = Ipv4Addr::from_str(&seed_ip[..]).unwrap();
            Some(SocketAddrV4::new(seed_ip, seed_port))
        }
    };

    //start the service
    let service = OmniscientService::new(id, token, app_addr, service_addr, seed_addr);
    let rx = service.start();

    //TODO start up a server socket at ip listen_addr and port app_port

    //start event loop on service
    while let Ok(event) = rx.recv() {
        match event {
            Event::PeerTableMsgEvent(peer_table) => {
                println!("recv PeerTableMsgEvent");
                for (token, socket_addr) in peer_table.iter() {
                    println!("{}: {}", token, socket_addr);
                }
            },
            Event::RegisterTokenMsgEvent(token, socket_addr) => {
                println!("recv RegisterTokenMsgEvent({}, {})", token, socket_addr);
            },
            _ => {},
        }
    }
}
