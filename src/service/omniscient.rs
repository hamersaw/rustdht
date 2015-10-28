extern crate bincode;
use bincode::rustc_serialize::{decode,encode};

extern crate capnp;
pub mod message_capnp {
    include!(concat!(env!("OUT_DIR"), "/message_capnp.rs"));
}

use message::{AddrMsg,Message,MessageType,JoinMsg,LookupMsg,ResultMsg};
use event::Event;

use std::collections::BTreeMap;
use std::io::{Read,Write};
use std::net::{SocketAddrV4,TcpListener,TcpStream};
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
 
                //create capnproto join message
                let mut join_message = capnp::message::Builder::new_default();
                {
                    let mut msg = join_message.init_root::<message_capnp::join_msg::Builder>();
                    msg.set_id(&self.id.clone()[..]);
                    msg.set_token(self.token.clone());

                    let ip: String = format!("{}", self.listen_addr.ip());
                    msg.set_ip(&ip[..]);
                    msg.set_port(self.listen_addr.port());
                }

                //send join message
                //capnp::serialize::write_message(&mut stream, &join_message);
                capnp::serialize::write_message(&mut ::std::io::stdout(), &join_message).unwrap();

                //TODO read capnproto result message
                let message_reader = capnp::serialize::read_message(&mut stream, ::capnp::message::ReaderOptions::new()).unwrap();
                let result_message = message_reader.get_root::<message_capnp::result_msg::Reader>();

                //create join message and serialize to vector
                let join_msg = JoinMsg::new(self.id.clone(), self.token.clone(), self.listen_addr.clone());
                let mut vec = encode(&join_msg, bincode::SizeLimit::Infinite).unwrap();

                //write message to stream
                vec.insert(0, MessageType::JoinMsg as u8);
                stream.write_all(&vec).unwrap();

                //read from stream into buf
                let mut buf = [0; 32];
                let bytes = stream.read(&mut buf).unwrap();

                //parse out message - buf[0] = msg_type
                if buf[0] != MessageType::ResultMsg as u8 {
                    panic!("Unexpected message type {} returned", buf[0]);
                }

                let result_msg: ResultMsg = decode(&buf[1..bytes]).unwrap();
                result_msg.print();
                //TODO handle failed result message
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
                    let mut stream = match stream {
                        Ok(stream) => stream,
                        Err(e) => panic!("{}", e),
                    };

                    //read from stream into buf
                    let mut buf = [0; 1024];
                    let bytes = stream.read(&mut buf).unwrap();

                    //parse out message - buf[0] = msg_type
                    match buf[0] {
                        x if x == MessageType::JoinMsg as u8 => {
                            let join_msg: JoinMsg = decode(&buf[1..bytes]).unwrap();
                            tx.send(Event::JoinMsgEvent(join_msg.get_id(), join_msg.get_token(), join_msg.get_socket_addr())).unwrap();

                            //add token and socket address to peer table
                            let mut peer_table = peer_table.write().unwrap();
                            let result_msg: ResultMsg = match add_token(&mut peer_table, join_msg.get_token(), join_msg.get_socket_addr()) {
                                Ok(token_added) => ResultMsg::new(token_added, None),
                                Err(e) => ResultMsg::new(false, Some(e)),
                            };

                            //serialize result message
                            let mut vec = encode(&result_msg, bincode::SizeLimit::Infinite).unwrap();
 
                            //write message to stream
                            vec.insert(0, MessageType::ResultMsg as u8);
                            stream.write_all(&vec).unwrap();

                            //if token was added send address back and forward join to every peer
                            if result_msg.get_success() {
                                //TODO send address message to joining node

                                for (peer_token, peer_socket_addr) in peer_table.iter() {
                                    if join_msg.get_token() == *peer_token {
                                        continue;
                                    }

                                    let mut stream = TcpStream::connect(peer_socket_addr).unwrap();
                
                                    //create join message and serialize to vector
                                    let mut vec = encode(&join_msg, bincode::SizeLimit::Infinite).unwrap();

                                    //write message to stream
                                    vec.insert(0, MessageType::JoinMsg as u8);
                                    stream.write(&vec).unwrap();

                                    //read from stream into buf
                                    let mut buf = [0; 32];
                                    let bytes = stream.read(&mut buf).unwrap();

                                    //parse out message - buf[0] = msg_type
                                    if buf[0] != MessageType::ResultMsg as u8 {
                                        panic!("Unexpected message type {} returned", buf[0]);
                                    }

                                    let result_msg: ResultMsg = decode(&buf[1..bytes]).unwrap();
                                    result_msg.print();
                                    //TODO handle a failure in the result message
                                }
                            }
                        },
                        x if x == MessageType::AddrMsg as u8 => {
                            //TODO process address message - add token
                        },
                        x if x == MessageType::LookupMsg as u8 => {
                            let lookup_msg: LookupMsg = decode(&buf[1..bytes]).unwrap();
                            tx.send(Event::LookupMsgEvent(lookup_msg.get_token())).unwrap();

                            //process the lookup
                            let peer_table = peer_table.read().unwrap();
                            let vec =  match lookup(&peer_table, lookup_msg.get_token()) {
                                Some(socket_addr) => {
                                    let addr_msg = AddrMsg::new(socket_addr);
                                    //let mut vec = addr_msg.to_vec();
                                    let mut vec = encode(&addr_msg, bincode::SizeLimit::Infinite).unwrap();
                                    vec.insert(0, MessageType::AddrMsg as u8);
                                    vec
                                },
                                None => {
                                    let result_msg = ResultMsg::new(false,Some("Unable to determine socket address for token".to_string()));
                                    let mut vec = encode(&result_msg, bincode::SizeLimit::Infinite).unwrap();
                                    vec.insert(0, MessageType::ResultMsg as u8);
                                    vec
                                },
                            };

                            //write message to stream
                            stream.write_all(&vec).unwrap();
                        },
                        x if x == MessageType::GenericMsg as u8 => {
                            tx.send(Event::GenericMsgEvent(buf[1..bytes].to_vec(), stream)).unwrap();
                        }
                        _ => panic!("Unrecognized message type: {}", buf[0]),
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
