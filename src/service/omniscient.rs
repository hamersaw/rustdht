extern crate capnp;
use message_capnp;
use message_capnp::message::msg_type::{GenericMsg,JoinMsg,LookupMsg,RegisterTokenMsg,ResultMsg};

use event::Event;

use std::collections::BTreeMap;
use std::io::{Read,Write};
use std::net::{Ipv4Addr,SocketAddrV4,TcpListener,TcpStream};
use std::str::FromStr;
use std::sync::{Arc,RwLock};
use std::sync::mpsc::{channel,Receiver};
use std::thread;
use std::thread::JoinHandle;

pub struct OmniscientService {
    id: String,
    token: u64,
    listen_addr: SocketAddrV4,
    seed_addr: Option<SocketAddrV4>,
    peer_table: Arc<RwLock<BTreeMap<u64,SocketAddrV4>>>,
}

impl OmniscientService {
    pub fn new(id: String, token: u64, listen_addr: SocketAddrV4, seed_addr: Option<SocketAddrV4>) -> Self {
        OmniscientService {
            id: id,
            token: token,
            listen_addr: listen_addr,
            seed_addr: seed_addr,
            peer_table: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }

    pub fn start(&self) -> (JoinHandle<()>, Receiver<Event>) {
        //send join message to seed_addr
        match self.seed_addr {
            Some(seed_addr) => {
                let mut stream = TcpStream::connect(seed_addr).unwrap();

                //create join message
                let mut msg_builder = capnp::message::Builder::new_default();
                {
                    let msg = msg_builder.init_root::<message_capnp::message::Builder>();
                    let mut join_msg = msg.get_msg_type().init_join_msg();
                    join_msg.set_id(&self.id.clone()[..]);
                    join_msg.set_token(self.token.clone());
                    let mut msg_socket_addr = join_msg.get_socket_addr().unwrap();
                    msg_socket_addr.set_ip(&self.listen_addr.ip().to_string()[..]);
                    msg_socket_addr.set_port(self.listen_addr.port());
                }

                //send join message
                capnp::serialize::write_message(&mut stream, &msg_builder).unwrap();
                
                //read result message
                let msg_reader = capnp::serialize::read_message(&mut stream, ::capnp::message::ReaderOptions::new()).unwrap();
                let msg = msg_reader.get_root::<message_capnp::message::Reader>().unwrap();
                match msg.get_msg_type().which() {
                    Ok(ResultMsg(result_msg)) => {
                        println!("recv result msg with success: {}", result_msg.get_success());
                    },
                    Ok(_) => panic!("Unknown message type"),
                    Err(capnp::NotInSchema(e)) => panic!("Error capnp::NotInSchema: {}", e),
                };
            },
            None => {},
        }

        //create listener
        let listener = match TcpListener::bind(self.listen_addr) {
            Ok(listener) => listener,
            Err(e) => panic!("{}", e),
        };

        //print socket addr
        match listener.local_addr() {
            Ok(local_addr) => println!("Server {} listening at {}", self.id, local_addr),
            Err(e) => panic!("{}", e),
        };

        //create event channel
        let (tx, rx) = channel::<Event>();

        //start listening
        let peer_table = self.peer_table.clone();
        let handle = thread::spawn(move || {
            for stream in listener.incoming() {
                let peer_table = peer_table.clone();
                let tx = tx.clone();

                thread::spawn(move || {
                    let mut stream = stream.unwrap();

                    //read capnproto result message
                    let msg_reader = capnp::serialize::read_message(&mut stream, ::capnp::message::ReaderOptions::new()).unwrap();
                    let msg = msg_reader.get_root::<message_capnp::message::Reader>().unwrap();

                    //parse out message
                    match msg.get_msg_type().which() {
                        Ok(GenericMsg(generic_msg)) => {
                            tx.send(Event::GenericMsgEvent(generic_msg.get_data().unwrap().to_vec(), stream)).unwrap();
                        },
                        Ok(JoinMsg(join_msg)) => {
                            let msg_socket_addr = join_msg.get_socket_addr().unwrap();
                            tx.send(Event::JoinMsgEvent(join_msg.get_id().unwrap().to_string(), join_msg.get_token(), msg_socket_addr.get_ip().unwrap().to_string(), msg_socket_addr.get_port())).unwrap();

                            //create socket address
                            let msg_socket_addr = join_msg.get_socket_addr().unwrap();
                            let ip_addr = Ipv4Addr::from_str(msg_socket_addr.get_ip().unwrap()).unwrap();
                            let socket_addr = SocketAddrV4::new(ip_addr, msg_socket_addr.get_port());

                            //create result message
                            let mut msg_builder = capnp::message::Builder::new_default();
                            let mut peer_table = peer_table.write().unwrap();
                            {
                                let msg = msg_builder.init_root::<message_capnp::message::Builder>();
                                let mut result_msg = msg.get_msg_type().init_result_msg();

                                //add token and socket address to peer table
                                match add_token(&mut peer_table, join_msg.get_token(), socket_addr) {
                                    Ok(token_added) => {
                                        result_msg.set_success(token_added);
                                        result_msg.set_err_msg("");
                                    },
                                    Err(e) => {
                                        result_msg.set_success(false);
                                        result_msg.set_err_msg(&format!("{}", e)[..]);
                                    }
                                };
                            }

                            //send result message
                            capnp::serialize::write_message(&mut stream, &msg_builder).unwrap();

                            //TODO send peer table to joining node
                            
                            //create register token message
                            let mut msg_builder = capnp::message::Builder::new_default();
                            {
                                let msg = msg_builder.init_root::<message_capnp::message::Builder>();
                                let mut register_token_msg = msg.get_msg_type().init_register_token_msg();
                                register_token_msg.set_token(join_msg.get_token());
                                register_token_msg.set_socket_addr(join_msg.get_socket_addr().unwrap()).unwrap();
                            }

                            //send register token message to all peers
                            for (peer_token, peer_socket_addr) in peer_table.iter() {
                                if join_msg.get_token() == *peer_token {
                                    continue;
                                }

                                let mut stream = TcpStream::connect(peer_socket_addr).unwrap();
                                capnp::serialize::write_message(&mut stream, &msg_builder).unwrap();

                                //read result message
                                let msg_reader = capnp::serialize::read_message(&mut stream, ::capnp::message::ReaderOptions::new()).unwrap();
                                let msg = msg_reader.get_root::<message_capnp::message::Reader>().unwrap();
                                match msg.get_msg_type().which() {
                                    Ok(ResultMsg(result_msg)) => {
                                        println!("recv result msg with success: {}", result_msg.get_success());
                                    },
                                    Ok(_) => panic!("Unknown message type"),
                                    Err(capnp::NotInSchema(e)) => panic!("Error capnp::NotInSchema: {}", e),
                                };
                            }
                        },
                        Ok(LookupMsg(lookup_msg)) => {
                            tx.send(Event::LookupMsgEvent(lookup_msg.get_token())).unwrap();

                            //create result message
                            let mut msg_builder = capnp::message::Builder::new_default();
                            {
                                let msg = msg_builder.init_root::<message_capnp::message::Builder>();

                                //lookup token in peer table and create return message
                                let peer_table = peer_table.read().unwrap();
                                match lookup(&peer_table, lookup_msg.get_token()) {
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
                        Ok(RegisterTokenMsg(register_token_msg)) => {
                            //TODO send event to tx

                            let mut msg_builder = capnp::message::Builder::new_default();
                            {
                                let msg = msg_builder.init_root::<message_capnp::message::Builder>();
                                let mut result_msg = msg.get_msg_type().init_result_msg();

                                let msg_socket_addr = register_token_msg.get_socket_addr().unwrap();
                                let ip_addr = Ipv4Addr::from_str(&msg_socket_addr.get_ip().unwrap()[..]).unwrap();
                                let socket_addr = SocketAddrV4::new(ip_addr, msg_socket_addr.get_port());

                                //add token and socket address to peer table
                                let mut peer_table = peer_table.write().unwrap();
                                match add_token(&mut peer_table, register_token_msg.get_token(), socket_addr) {
                                    Ok(token_added) => {
                                        result_msg.set_success(token_added);
                                        result_msg.set_err_msg("");
                                    },
                                    Err(e) => {
                                        result_msg.set_success(false);
                                        result_msg.set_err_msg(&format!("{}", e)[..]);
                                    }
                                };
                            }

                            //send result message
                            capnp::serialize::write_message(&mut stream, &msg_builder).unwrap();
                        },
                        Ok(_) => panic!("Unknown message type"),
                        Err(capnp::NotInSchema(e)) => panic!("Error capnp::NotInSchema: {}", e),
                    };
                });
            }
        });

        (handle, rx)
    }

    pub fn lookup(&self, token: u64) -> Option<SocketAddrV4> {
        let peer_table = self.peer_table.clone();
        let peer_table = peer_table.read().unwrap();

        lookup(&peer_table, token)
    }

    pub fn add_token(&mut self, token: u64, socket_addr: SocketAddrV4) -> Result<bool,String> {
        let peer_table = self.peer_table.clone();
        let mut peer_table = peer_table.write().unwrap();

        add_token(&mut peer_table, token, socket_addr)
    }
    
    pub fn send_msg(&self, token: u64, msg: Vec<u8>) -> Result<(), String> {
        unimplemented!();
    }

    pub fn broadcast_msg(&self, msg: Vec<u8>) -> Result<(), String> {
        unimplemented!();
    }

    pub fn print(&self) {
        println!("ID:{}\ntoken:{}", self.id, self.token);
        
        let peer_table = self.peer_table.clone();
        let peer_table = peer_table.read().unwrap();

        println!("----TOKEN TABLE----");
        for (peer_token, peer_socket_addr) in peer_table.iter() {
            println!("\t{}: {}", peer_token, peer_socket_addr);
        }
    }
}

fn lookup(peer_table: &BTreeMap<u64,SocketAddrV4>, token: u64) -> Option<SocketAddrV4> {
    //get first (smallest) token from the peer table
    let mut iter = peer_table.iter();
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

fn add_token(peer_table: &mut BTreeMap<u64,SocketAddrV4>, token: u64, socket_addr: SocketAddrV4) -> Result<bool,String> {
    let token_added = match peer_table.contains_key(&token) {
        false => {
            peer_table.insert(token, socket_addr);
            true
        },
        true => false,
    };

    Ok(token_added)
}
