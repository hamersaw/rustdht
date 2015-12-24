#rustdht

##Overview
This library is a generic distributed hash table framework written in rust.

##Running Echo Server Example
####Server
```bash
./echo_server -i coeus -t 0 -i 127.0.0.1 -a 15605 -p 15705
./echo_server -i crius -t 1537228672809129301 -i 127.0.0.1 -a 15606 -p 15706 -s 127.0.0.1 -e 15705
./echo_server -i cronus -t 3074457345618258602 -i 127.0.0.1 -a 15607 -p 15707 -s 127.0.0.1 -e 15705
./echo_server -i hyperion -t 4611686018427387903 -i 127.0.0.1 -a 15608 -p 15708 -s 127.0.0.1 -e 15705
./echo_server -i lapetus -t 6148914691236517204 -i 127.0.0.1 -a 15609 -p 15709 -s 127.0.0.1 -e 15705
./echo_server -i mnemosyne -t 7686143364045646505 -i 127.0.0.1 -a 15610 -p 15710 -s 127.0.0.1 -e 15705
./echo_server -i oceanus -t 9223372036854775806 -i 127.0.0.1 -a 15611 -p 15711 -s 127.0.0.1 -e 15705
./echo_server -i phoebe -t 10760600709663905107 -i 127.0.0.1 -a 15612 -p 15712 -s 127.0.0.1 -e 15705
./echo_server -i rhea -t 12297829382473034408 -i 127.0.0.1 -a 15613 -p 15713 -s 127.0.0.1 -e 15705
./echo_server -i tethys -t 13835058055282163709 -i 127.0.0.1 -a 15614 -p 15714 -s 127.0.0.1 -e 15705
./echo_server -i theia -t 15372286728091293010 -i 127.0.0.1 -a 15615 -p 15715 -s 127.0.0.1 -e 15705
./echo_server -i themis -t 16909515400900422311 -i 127.0.0.1 -a 15616 -p 15716 -s 127.0.0.1 -e 15705
```

####Client
```bash
./echo_client -i 127.0.0.1 -p 15605
```

##
- fuzzydb (https://github.com/hamersaw/fuzzydb)

##TODO
