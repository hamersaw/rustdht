use std::collections::BTreeMap;
use std::io::Write;
use std::net::{TcpListener};
use std::net::{Ipv4Addr,SocketAddrV4};
use std::str::FromStr;
use std::sync::{Arc,RwLock};
use std::thread;

#[derive(Clone)]
pub struct ChordNode {
    id: String,
    token: u64,
    listen_addr: SocketAddrV4,
    pred: Option<(u64, SocketAddrV4)>,
    succ: Option<(u64, SocketAddrV4)>,
    finger_table: Arc<RwLock<BTreeMap<u64,SocketAddrV4>>>,
}

impl ChordNode {
    pub fn new(id: String, token: u64, listen_ip: &str, listen_port: u16) -> Self {
        //parse ip address
        let listen_ip = match Ipv4Addr::from_str(listen_ip) {
            Ok(listen_ip) => listen_ip,
            Err(_) => panic!("Unable to parse ip '{}'", listen_ip),
        };

        //TODO search for pred and succ

        ChordNode {
            id: id,
            token: token,
            listen_addr: SocketAddrV4::new(listen_ip, listen_port),
            pred: None,
            succ: None,
            finger_table: Arc::new(RwLock::new(BTreeMap::new()))
        }
    }

    pub fn start(&self) {
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

        for stream in listener.incoming() {
            let self_clone = self.clone();

            thread::spawn(move || {
                println!("recieved connection from peer");
                let mut stream = stream.unwrap();
                stream.write(b"Hello World\n\t").unwrap();

                //TODO actually send and receive messages
                let _ = self_clone.lookup(0 as u64);
            });
        }
    }

    pub fn lookup(&self, token: u64) -> Option<SocketAddrV4> {
        //if the value is between pred and this node return this ip
        match self.pred {
            Some((pred_token, _)) => {
                if token > pred_token && token <= self.token {
                    return Some(self.listen_addr);
                }
            },
            None => {},
        }

        //if the value is between this and the successor value return sucessor ip
        match self.succ {
            Some((succ_token, socket_addr)) => {
                if token > self.token && token <= succ_token {
                    return Some(socket_addr);
                }
            },
            None => {},
        }

        //get finger table read lock
        let finger_table = self.finger_table.clone();
        let finger_table = match finger_table.read() {
            Ok(finger_table) => finger_table,
            Err(e) => panic!("{}", e),
        };
        
        //get first (smallest) token from the finger table
        let mut iter = finger_table.iter();
        let first_tuple = match iter.next() {
            Some(current_tuple) => current_tuple,
            None => return None,
        };

        //if search token is smaller than first
        if token <  *first_tuple.0 {
            return Some(*first_tuple.1);
        };

        //search for betweens
        let mut last_token = *first_tuple.0;
        for (current_token, socket_addr) in iter {
            if last_token < token && *current_token >= token {
                return Some(*socket_addr);
            }

            last_token = *current_token;
        }

        Some(*first_tuple.1)
    }

    /*pub fn send(&self, msg: &GenericMsg) -> Result<(),String> {
        //TODO actually need to route through the cluster
        msg.execute()
    }*/

    pub fn add_token(&mut self, token: u64, socket_addr: SocketAddrV4) -> Result<(),String> {
        let finger_table = self.finger_table.clone();
        let mut finger_table = match finger_table.write() {
            Ok(finger_table) => finger_table,
            Err(e) => panic!("{}", e),
        };

        finger_table.insert(token, socket_addr);

        Ok(())
    }

    pub fn print(&self) {
        println!("ID:{}\ntoken:{}", self.id, self.token);
        
        let finger_table = self.finger_table.clone();
        let finger_table = match finger_table.read() {
            Ok(finger_table) => finger_table,
            Err(e) => panic!("{}", e),
        };

        println!("----FINGER TABLE----");
        for (key, value) in finger_table.iter() {
            println!("\t{}: {}", key, value);
        }
    }
}
