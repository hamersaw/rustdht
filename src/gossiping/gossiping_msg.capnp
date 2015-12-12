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
		probeMsg :group {
			timestamp @3 :UInt64;
		}
		registerTokenMsg :group {
			token @4 :UInt64;
			appAddr @5 :SocketAddr;
			serviceAddr @6 :SocketAddr;
			joinInd @7 :Bool;
		}
		resultMsg :group {
			success @8 :Bool;
			errMsg @9 :Text;
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
