use tungstenite::*;

fn anything_to_bytes<T: Sized>(to_pack: &T) -> &'static [u8] {
    unsafe {std::slice::from_raw_parts((to_pack as *const T) as *const u8, std::mem::size_of::<T>())}
}

fn bytes_to_anything<'a, T>(bytes: &'a [u8]) -> &'a T {
    assert_eq!(bytes.len(), std::mem::size_of::<T>());
    let ptr: *const u8 = bytes.as_ptr();
    assert_eq!(ptr.align_offset(std::mem::align_of::<T>()), 0);

    unsafe {ptr.cast::<T>().as_ref().unwrap()}
}

#[repr(C)]
struct STRUCT_REQ_JOIN {
    token: u8,
    name: [u8; 31]
}

#[repr(C)]
struct STRUCT_RSP_JOIN {
    token: u8,
    maxcli: u8,
    id: u16
}

fn main() {
    let mut ws = connect("ws://localhost:8080/ws/y6/").expect("Failed to open a websocket").0;

    let mut name = ['\0' as u8; 31];
    name[0] = 'B' as u8;
    name[1] = 'o' as u8;
    name[2] = 't' as u8;

    let join_req = STRUCT_REQ_JOIN {
        token: 'J' as u8,
        name: name
    };
    
    let join_req_bytes = anything_to_bytes(&join_req);

    println!("Can write: {}, Can read: {}", ws.can_write(), ws.can_read());

    ws.write_message(Message::Binary(join_req_bytes.to_vec())).expect("Failed to send join request");

    let msg = ws.read_message().expect("Failed to read a message from the websocket");

    let rsp: &STRUCT_RSP_JOIN;

    let tmp_data: Vec<u8>;

    match msg {
        Message::Binary(data) => {
            tmp_data = data;
            rsp = bytes_to_anything::<STRUCT_RSP_JOIN>(&tmp_data);
        },
        _ => panic!("Expected binary data")
    }

    println!("Response:\ntoken: {}\nmax clients: {}\nid: {}", rsp.token as char, rsp.maxcli, rsp.id);

    loop {
        println!("Trying to receive a message");

        let msg = ws.read_message().expect("Failed to get a message in a loop");

        println!("Got a message: {:?}", msg);
    }
}