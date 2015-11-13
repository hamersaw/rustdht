@0xf25f36c02ae1cd9d;

struct Message {
	msgType :union {
		addrMsg :group {
			socketAddr @0 :SocketAddr;
		}
		lookupMsg :group {
			token @1 :UInt64;
		}
		lookupTableMsg :group {
			entries @2 :List(LookupEntry);
		}
		registerTokenMsg :group {
			token @3 :UInt64;
			appAddr @4 :SocketAddr;
			serviceAddr @5 :SocketAddr;
			joinInd @6 :Bool;
		}
		resultMsg :group {
			success @7 :Bool;
			errMsg @8 :Text;
		}
	}
}

struct LookupEntry {
	token @0 :UInt64;
	ip @1 :Text;
	port @2 :UInt16;
}

struct SocketAddr {
	ip @0 :Text;
	port @1 :UInt16;
}
