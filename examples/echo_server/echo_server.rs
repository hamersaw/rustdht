extern crate argparse;
use argparse::{ArgumentParser,Store};

extern crate capnp;

pub mod message_capnp {
    include!(concat!(env!("OUT_DIR"), "/message_capnp.rs"));
}
use message_capnp::echo_server_msg::msg_type::{ClientEchoMsg,EchoMsg};

extern crate rustdht;
use rustdht::event::Event;

use std::collections::BTreeMap;
use std::net::{Ipv4Addr,SocketAddrV4,TcpListener,TcpStream};
use std::str::FromStr;
use std::sync::{Arc,RwLock};
use std::thread;

fn main() {
    let mut token: u64 = 0;
    let mut app_ip: String = "127.0.0.1".to_string();
    let mut app_port: u16 = 0;
    let mut service_port: u16 = 0;
    let mut seed_ip: String = "127.0.0.1".to_string();
    let mut seed_port: u16 = 0;
    {    //solely to limit scope of parser variable
        let mut parser = ArgumentParser::new();
        parser.set_description("start up a echo server");
        parser.refer(&mut token).add_option(&["-t", "--token"], Store, "token of node").required();
        parser.refer(&mut app_ip).add_option(&["-i", "--listen-ip"], Store, "ip address for application and service to listen on").required();
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

    //start the p2p service
    let lookup_table = Arc::new(RwLock::new(BTreeMap::new()));
    let rx = rustdht::service::start(token, app_addr, service_addr, seed_addr, lookup_table.clone());

    //start listening on the application socket
    let listener = TcpListener::bind(app_addr).unwrap();
    let lookup_table = lookup_table.clone();
    let _ = thread::spawn(move || {
        for stream in listener.incoming() {
            let lookup_table = lookup_table.clone();
            thread::spawn(move || {
                let mut stream = stream.unwrap();

                let msg_reader = capnp::serialize::read_message(&mut stream, ::capnp::message::ReaderOptions::new()).unwrap();
                let msg = msg_reader.get_root::<message_capnp::echo_server_msg::Reader>().unwrap();

                //parse out message
                match msg.get_msg_type().which() {
                    Ok(ClientEchoMsg(client_echo_msg)) => {
                        //search lookup table to find address responsible for token
                        let lookup_table = lookup_table.read().unwrap();
                        let socket_addr = match  rustdht::service::lookup(&lookup_table, client_echo_msg.get_token()) {
                            Some(socket_addr) => socket_addr,
                            None => panic!("Unable to find address responsible for token"),
                        };

                        //create echo message
                        let mut msg_builder = capnp::message::Builder::new_default();
                        {
                            let msg = msg_builder.init_root::<message_capnp::echo_server_msg::Builder>();
                            msg.get_msg_type().set_echo_msg(client_echo_msg.get_msg().unwrap());
                        }

                        //send echo message
                        let mut stream = TcpStream::connect(socket_addr).unwrap();
                        capnp::serialize::write_message(&mut stream, &msg_builder).unwrap();
                    },
                    Ok(EchoMsg(echo_msg)) => {
                        println!("ECHO MSG: '{}'", echo_msg.unwrap());
                    },
                    Err(capnp::NotInSchema(e)) => panic!("Error capnp::NotInSchema: {}", e),
                }
            });
        }
    });

    //start event loop from the p2p service
    while let Ok(event) = rx.recv() {
        match event {
            Event::RemoveNodeEvent(token, socket_addr) => {
                println!("recv RemoveNodeEvent({}, {})", token, socket_addr);
            },
            Event::RegisterNodeEvent(token, socket_addr) => {
                println!("recv RegisterNodeEvent({}, {})", token, socket_addr);
            },
            _ => {},
        }
    }
}
