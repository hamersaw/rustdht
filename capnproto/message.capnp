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

struct TestMsg {
	msg @0 :Text;
}
