@0xd25f36c02ae1cd9d;

struct Message {
	msgType :union {
		addrMsg :group {
			socketAddr @0:SocketAddr;
		}
		genericMsg :group {
			data @1 :Data;
		}
		joinMsg :group {
			id @2 :Text;
			token @3 :UInt64;
			socketAddr @4 :SocketAddr;
		}
		lookupMsg :group {
			token @5 :UInt64;
		}
		peerTableMsg :group {
			peers @6 :List(PeerAddr);
		}
		registerTokenMsg :group {
			token @7 :UInt64;
			socketAddr @8 :SocketAddr;
		}
		resultMsg :group {
			success @9 :Bool;
			errMsg @10 :Text;
		}
	}
}

struct PeerAddr {
	token @0 :UInt64;
	ip @1 :Text;
	port @2 :UInt16;
}

struct SocketAddr {
	ip @0 :Text;
	port @1 :UInt16;
}
