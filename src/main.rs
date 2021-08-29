use std::{io::{Read, Write}, net::*};

const UPGRADE_REQ: &str = "GET /ws/y6/ HTTP/1.1\r\n\
Host: asciicker.com\r\n\
User-Agent: native-asciicker-linux\r\n\
Accept: */*\r\n\
Accept-Language: en-US,en;q=0.5\r\n\
Sec-WebSocket-Version: 13\r\n\
Sec-WebSocket-Key: btsPdKGunHdaTPnSSDlfow==\r\n\
Pragma: no-cache\r\n\
Cache-Control: no-cache\r\n\
Upgrade: WebSocket\r\n\
Connection: Upgrade\r\n\r\n";

fn read_till_double_crlf(socket: &mut TcpStream, buf: &mut Vec<u8>) {
    let mut mem: (bool, bool, bool) = (false, false, false);
    loop {
        let mut new_buf = [0; 1];
        socket.read_exact(&mut new_buf).unwrap();
        buf.push(new_buf[0]);
        match std::str::from_utf8(&new_buf).unwrap() {
            "\r" => {
                match mem {
                    (_, true, _) => mem = (false, false, true),
                    (_, _, _) => mem = (true, false, false),
                }
            },
            "\n" => {
                match mem {
                    (true, _, _) => mem = (false, true, false),
                    (_, _, true) => mem = (true, true, true),
                    (_, _, _) => mem = (false, false, false)
                }
            },
            _ => mem = (false, false, false)
        }
        if mem == (true, true, true) {
            break
        }
    }
}

struct STRUCT_REQ_JOIN {
    token: u8,
    name: [char; 31]
}

fn main() {
    let mut socket = TcpStream::connect("asciicker.com:80").expect("Failed to create a TCP socket");

    socket.write(UPGRADE_REQ.as_bytes()).expect("Failed to upgrade the socket to a websocket");
    
    let mut buf = vec![];

    read_till_double_crlf(&mut socket, &mut buf);

    println!("{}", std::str::from_utf8(buf.as_slice()).unwrap());

    let mut name = ['\0'; 31];
    name[0] = 'B';
    name[1] = 'o';
    name[2] = 't';

    let join_req = STRUCT_REQ_JOIN {
        token: 'J' as u8,
        name: name
    };

    socket.shutdown(Shutdown::Both).expect("Failed to shutdown the socket");
}