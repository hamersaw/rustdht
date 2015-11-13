rustp2p
=======

Overview
--------
This library is a generic p2p framework written in rust.

Examples
--------
cqdb - https://github.com/hamersaw/cqdb
note: cqdb not yet implemented

```bash
./echo_server -i coeus -t 0 -l 127.0.0.1 -a 15605 -p 15705
./echo_server -i crius -t 1537228672809129301 -l 127.0.0.1 -a 15606 -p 15706 -s 127.0.0.1 -e 15705
./echo_server -i cronus -t 3074457345618258602 -l 127.0.0.1 -a 15607 -p 15707 -s 127.0.0.1 -e 15705
./echo_server -i hyperion -t 4611686018427387903 -l 127.0.0.1 -a 15608 -p 15708 -s 127.0.0.1 -e 15705
./echo_server -i lapetus -t 6148914691236517204 -l 127.0.0.1 -a 15609 -p 15709 -s 127.0.0.1 -e 15705
./echo_server -i mnemosyne -t 7686143364045646505 -l 127.0.0.1 -a 15610 -p 15710 -s 127.0.0.1 -e 15705
./echo_server -i oceanus -t 9223372036854775806 -l 127.0.0.1 -a 15611 -p 15711 -s 127.0.0.1 -e 15705
./echo_server -i phoebe -t 10760600709663905107 -l 127.0.0.1 -a 15612 -p 15712 -s 127.0.0.1 -e 15705
./echo_server -i rhea -t 12297829382473034408 -l 127.0.0.1 -a 15613 -p 15713 -s 127.0.0.1 -e 15705
./echo_server -i tethys -t 13835058055282163709 -l 127.0.0.1 -a 15614 -p 15714 -s 127.0.0.1 -e 15705
./echo_server -i theia -t 15372286728091293010 -l 127.0.0.1 -a 15615 -p 15715 -s 127.0.0.1 -e 15705
./echo_server -i themis -t 16909515400900422311 -l 127.0.0.1 -a 15616 -p 15716 -s 127.0.0.1 -e 15705
```

TODO
----
- large changes are necessary to get a new framework
	provide a struct that has method "start()" that starts up the service
	then use lookup, to transfer messages
- key concept is a different port for rustp2p communications and application message passing
	allows us to start a server socket on the client side and use it separate from the rustp2p framework

* gossiping and chord service
