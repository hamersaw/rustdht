extern crate capnp;
use omniscient_msg_capnp;
use omniscient_msg_capnp::message::msg_type::{LookupMsg,LookupTableMsg,RegisterTokenMsg};

use omniscient::event::Event;

use std::collections::BTreeMap;
use std::io::{Read,Write};
use std::net::{Ipv4Addr,SocketAddrV4,TcpListener,TcpStream};
use std::str::FromStr;
use std::sync::{Arc,RwLock};
use std::sync::mpsc::{channel,Receiver};
use std::thread;

pub fn start(_: String, token: u64, app_addr: SocketAddrV4, service_addr: SocketAddrV4,  seed_addr: Option<SocketAddrV4>, lookup_table: Arc<RwLock<BTreeMap<u64,SocketAddrV4>>>) -> Receiver<Event> {
    //create variables needed and clone
    let service_addr_table = Arc::new(RwLock::new(BTreeMap::new()));
    let (tx, rx) = channel::<Event>();

    //clone variables
    let lookup_table = lookup_table.clone();
    let service_addr_table = service_addr_table.clone();

    //start listening
    let listener = TcpListener::bind(service_addr).unwrap();
    let _ = thread::spawn(move || {
        for stream in listener.incoming() {
            let lookup_table = lookup_table.clone();
            let service_addr_table = service_addr_table.clone();
            let tx = tx.clone();

            thread::spawn(move || {
                let mut stream = stream.unwrap();

                //read capnproto message
                let msg_reader = capnp::serialize::read_message(&mut stream, ::capnp::message::ReaderOptions::new()).unwrap();
                let msg = msg_reader.get_root::<omniscient_msg_capnp::message::Reader>().unwrap();

                //parse out message
                match msg.get_msg_type().which() {
                    Ok(LookupMsg(lookup_msg)) => {
                        tx.send(Event::LookupMsgEvent(lookup_msg.get_token())).unwrap();

                        //create result message
                        let mut msg_builder = capnp::message::Builder::new_default();
                        {
                            let msg = msg_builder.init_root::<omniscient_msg_capnp::message::Builder>();

                            //lookup token in peer table and create return message
                            let lookup_table = lookup_table.read().unwrap();
                            match lookup(&lookup_table, lookup_msg.get_token()) {
                                Some(socket_addr) => {
                                    let addr_msg = msg.get_msg_type().init_addr_msg();
                                    let mut msg_socket_addr = addr_msg.get_socket_addr().unwrap();
                                    msg_socket_addr.set_ip(&socket_addr.ip().to_string()[..]);
                                    msg_socket_addr.set_port(socket_addr.port());
                                },
                                None => {
                                    let mut result_msg = msg.get_msg_type().init_result_msg();
                                    result_msg.set_success(true);
                                    result_msg.set_err_msg("");
                                },
                            };
                        }

                        //send result message
                        capnp::serialize::write_message(&mut stream, &msg_builder).unwrap();
                    },
                    Ok(LookupTableMsg(lookup_table_msg)) => {
                        let mut map: BTreeMap<u64,SocketAddrV4> = BTreeMap::new();
                        for lookup_entry in lookup_table_msg.get_entries().unwrap().iter() {
                            let ip_addr = Ipv4Addr::from_str(&lookup_entry.get_ip().unwrap()[..]).unwrap();
                            let socket_addr = SocketAddrV4::new(ip_addr, lookup_entry.get_port());
                            
                            //add token and socket address to peer table
                            let mut lookup_table = lookup_table.write().unwrap();
                            let _ = add_token(&mut lookup_table, lookup_entry.get_token(), socket_addr);

                            map.insert(lookup_entry.get_token(), socket_addr);
                        }

                        //send event
                        tx.send(Event::LookupTableMsgEvent(map)).unwrap();
                    },
                    Ok(RegisterTokenMsg(register_token_msg)) => {
                        //add entry to lookup table
                        {
                            let msg_app_addr = register_token_msg.get_app_addr().unwrap();
                            let ip_addr = Ipv4Addr::from_str(&msg_app_addr.get_ip().unwrap()[..]).unwrap();
                            let socket_addr = SocketAddrV4::new(ip_addr, msg_app_addr.get_port());
                            tx.send(Event::RegisterTokenMsgEvent(register_token_msg.get_token(), socket_addr.clone())).unwrap();

                            //add token and socket address to peer table
                            let mut lookup_table = lookup_table.write().unwrap();
                            let _ = add_token(&mut lookup_table, register_token_msg.get_token(), socket_addr);
                        }

                        //parse out service socket address
                        let msg_service_addr = register_token_msg.get_service_addr().unwrap();
                        let ip_addr = Ipv4Addr::from_str(&msg_service_addr.get_ip().unwrap()[..]).unwrap();
                        let socket_addr = SocketAddrV4::new(ip_addr, msg_service_addr.get_port());

                        if register_token_msg.get_join_ind() {
                            //create peer table message
                            let mut msg_builder = capnp::message::Builder::new_default();
                            {
                                let msg = msg_builder.init_root::<omniscient_msg_capnp::message::Builder>();
                                let lookup_table_msg = msg.get_msg_type().init_lookup_table_msg();

                                let lookup_table = lookup_table.read().unwrap();
                                let mut lookup_entries = lookup_table_msg.init_entries(lookup_table.len() as u32);

                                let mut index = 0;
                                for (token, socket_addr) in lookup_table.iter() {
                                    if register_token_msg.get_token() == *token {
                                        continue;
                                    }

                                    let mut lookup_entry = lookup_entries.borrow().get(index); 
                                    lookup_entry.set_token(*token);
                                    lookup_entry.set_ip(&socket_addr.ip().to_string()[..]);
                                    lookup_entry.set_port(socket_addr.port());

                                    index += 1;
                                }

                                //add yourto the peer table
                                let mut lookup_entry = lookup_entries.borrow().get(index);
                                lookup_entry.set_token(token);
                                lookup_entry.set_ip(&app_addr.ip().to_string()[..]);
                                lookup_entry.set_port(app_addr.port());
                            }

                            //send peer table message to joining node
                            let mut stream = TcpStream::connect(socket_addr).unwrap();
                            capnp::serialize::write_message(&mut stream, &msg_builder).unwrap();

                            //create register token message
                            let mut msg_builder = capnp::message::Builder::new_default();
                            {
                                let msg = msg_builder.init_root::<omniscient_msg_capnp::message::Builder>();
                                let mut rt_msg = msg.get_msg_type().init_register_token_msg();
                                rt_msg.set_token(register_token_msg.get_token());
                                rt_msg.set_app_addr(register_token_msg.get_app_addr().unwrap()).unwrap();
                                rt_msg.set_service_addr(register_token_msg.get_service_addr().unwrap()).unwrap();
                            }

                            //send register token message to all peers
                            let service_addr_table = service_addr_table.read().unwrap();
                            for (peer_token, peer_socket_addr) in service_addr_table.iter() {
                                if register_token_msg.get_token() == *peer_token {
                                    continue;
                                }

                                let mut stream = TcpStream::connect(peer_socket_addr).unwrap();
                                capnp::serialize::write_message(&mut stream, &msg_builder).unwrap();
                            }
                        }

                        //add service socket address to service address table
                        let mut service_addr_table = service_addr_table.write().unwrap();
                        let _ = add_token(&mut service_addr_table, register_token_msg.get_token(), socket_addr);
                    },
                    Ok(_) => panic!("Unknown message type"),
                    Err(capnp::NotInSchema(e)) => panic!("Error capnp::NotInSchema: {}", e),
                };
            });
        }
    });

    //send join message to seed_addr
    match seed_addr {
        Some(seed_addr) => {
            let mut stream = TcpStream::connect(seed_addr).unwrap();

            //create join message
            let mut msg_builder = capnp::message::Builder::new_default();
            {
                let msg = msg_builder.init_root::<omniscient_msg_capnp::message::Builder>();
                let mut register_token_msg = msg.get_msg_type().init_register_token_msg();
                register_token_msg.set_token(token.clone());
                register_token_msg.set_join_ind(true);

                {
                    let mut msg_app_addr = register_token_msg.borrow().get_app_addr().unwrap();
                    msg_app_addr.set_ip(&app_addr.ip().to_string()[..]);
                    msg_app_addr.set_port(app_addr.port());
                }

                {
                    let mut msg_service_addr = register_token_msg.borrow().get_service_addr().unwrap();
                    msg_service_addr.set_ip(&service_addr.ip().to_string()[..]);
                    msg_service_addr.set_port(service_addr.port());
                }
            }

            //send join message
            capnp::serialize::write_message(&mut stream, &msg_builder).unwrap();
        },
        None => {},
    }

    rx
}

pub fn lookup(lookup_table: &BTreeMap<u64,SocketAddrV4>, token: u64) -> Option<SocketAddrV4> {
    //get first (smallest) token from the peer table
    let mut iter = lookup_table.iter();
    let first_tuple = match iter.next() {
        Some(current_tuple) => current_tuple,
        None => return None,
    };

    //if search token is smaller than first
    if token < *first_tuple.0 {
        return Some(*first_tuple.1);
    };

    //search in between every set of concurrent tokens
    let mut last_token = *first_tuple.0;
    for (current_token, socket_addr) in iter {
        if last_token < token && *current_token >= token {
            return Some(*socket_addr);
        }

        last_token = *current_token;
    }

    Some(*first_tuple.1)
}

pub fn add_token(lookup_table: &mut BTreeMap<u64,SocketAddrV4>, token: u64, socket_addr: SocketAddrV4) -> Result<bool,String> {
    let token_added = match lookup_table.contains_key(&token) {
        false => {
            lookup_table.insert(token, socket_addr);
            true
        },
        true => false,
    };

    Ok(token_added)
}
