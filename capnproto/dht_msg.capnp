@0xff59a08fd3e241c3;

struct Message {
	msgType :union {
		addrMsg @0 :SocketAddr;
		heartbeatMsg @1: Void;
		lookupMsg @2 :UInt64;
		lookupTableMsg @3 :List(LookupEntry);
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
