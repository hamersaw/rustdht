@0xb3de5fbf2089d50b;

struct EchoServerMsg {
        msgType :union {
		clientEchoMsg : group {
			token @0 : UInt64;
			msg @1 :Text;
		}
                echoMsg @2 :Text;
	}
}
