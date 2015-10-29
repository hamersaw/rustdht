@0xf25f36c02ae1cd9d;

struct JoinMsg {
	id @0 :Text;
	token @1 :UInt64;
	ip @2 :Text;
	port @3 :UInt16;
}

struct ResultMsg {
	success @0 :Bool;
	errMsg @1 :Text;
}

struct Message {
	union {
		genericMsg :group {
			data @0 :Data;
		}
		joinMsg :group {
			id @1 :Text;
			token @2 :UInt64;
			ip @3 :Text;
			port @4 :UInt16;
		}
		lookupMsg :group {
			token @5 :UInt64;
		}
		registerTokenMsg :group {
			token @6 :UInt64;
			ip @7 :Text;
			port @8 :UInt16;
		}
		resultMsg :group {
			success @9 :Bool;
			errMsg @10 :Text;
		}
		
	}
}
