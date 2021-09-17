#![feature(async_closure)]

use std::collections::HashMap;

use tokio_tungstenite::{*, tungstenite::*};
use futures_util::{SinkExt, StreamExt};
use futures_timer::Delay;

mod ak;

unsafe fn anything_to_bytes<'a, T: Sized>(to_pack: &'a T) -> &'a [u8] {
    std::slice::from_raw_parts((to_pack as *const T) as *const u8, std::mem::size_of::<T>())
}

#[allow(dead_code)]
unsafe fn bytes_to_anything<'a, T>(bytes: &'a [u8]) -> &'a T {
    assert_eq!(bytes.len(), std::mem::size_of::<T>());
    let ptr: *const u8 = bytes.as_ptr();
    assert_eq!(ptr.align_offset(std::mem::align_of::<T>()), 0);

    ptr.cast::<T>().as_ref().unwrap()
}

#[repr(C)]
#[derive(Debug)]
struct Binary<T: Sized> {
    inner: T
}

impl<'a, T> Binary<T> {
    pub(crate) fn new(t: T) -> Self {
        Self {inner: t}
    }

    pub(crate) fn bytes(&'a self) -> &'a [u8] {
        unsafe {anything_to_bytes(&self.inner)}
    }

    #[allow(dead_code)]
    pub(crate) fn get_ref(&'a self) -> &'a T {
        &self.inner
    }

    #[allow(dead_code)]
    pub(crate) fn get_mut(&'a mut self) -> &'a mut T {
        &mut self.inner
    }
}

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    loop {
        match bot_main().await {
            Ok(()) => {
                return Ok(());
            },
            Err(e) => {
                println!("Connection has died, reason: {}", e);
                println!("Restarting...");
            }
        }
    }
}

async fn bot_main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut players = HashMap::<u16, ak::JoinBroadcast>::new();

    let mut ws = connect_async("ws://asciicker.com/ws/y6/").await?.0;

    let mut name = [b'\0'; 31];
    name[0] = b'B';
    name[1] = b'o';
    name[2] = b't';

    let join_req = Binary::new(ak::JoinRequest::new(name));

    ws.send(Message::Binary(join_req.bytes().to_vec())).await?;

    let data = match ws.next().await.unwrap().unwrap() {
        Message::Binary(d) => d,
        _ => panic!("Received data of an unsupported format")
    };

    let join_rsp = ak::JoinResponse::new(data.as_slice());

    println!("Bot has joined the server and received an id of {}", join_rsp.id);

    let (mut ws_s, mut ws_r) = ws.split(); 

    tokio::spawn(async move {
        loop {
            Delay::new(std::time::Duration::from_millis(10)).await;
            let pos_req = Binary::new(ak::PoseRequest::new(0, 0,0, [0.0, 0.0, 0.0], 0.0, 0));
            let pos_req_bytes = pos_req.bytes();
            ws_s.send(Message::Binary(pos_req_bytes[..pos_req_bytes.len()-2].to_vec())).await.expect("Failed to send a pose request");
        }
    });

    let mut headers = reqwest::header::HeaderMap::default();
    headers.insert(reqwest::header::CONTENT_TYPE, reqwest::header::HeaderValue::from_str("application/json").unwrap());                            
    let client = reqwest::Client::builder().default_headers(headers).build().expect("Failed to build http client");

    loop {
        while let Some(d) = ws_r.next().await {
            match d {
                Ok(tmp_data) => {
                    match tmp_data {
                        Message::Binary(more_data) => match more_data[0] {
                            ak::TalkBroadcast::TOKEN => {
                                let the_data = ak::TalkBroadcast::new(more_data);
                                let mut what: String = std::str::from_utf8(the_data.string().as_slice()).unwrap_or("Invalid UTF-8").escape_default().collect();                                
                                if std::env::var_os("CODEBLOCK").is_some() {
                                    what = "```".to_owned() + &what + &"```";
                                }
                                what = what.replace("\\'", "'");
                                let mut who: String = players[&the_data.id()].name().to_str().unwrap_or("Invalid UTF-8").to_owned().escape_default().collect();
                                who = who.replace("\\'", "'");
                                let body = format!("{{\"content\": \"{}\", \"username\": \"{}[id:{}]\", \"allowed_mentions\": {{\"parse\": []}}}}", what, who, the_data.id());
                                println!("Sending data to a webhook: {}", body);
                                client.post(std::env::var("DISCORD_WEBHOOK").unwrap())
                                .body(body)
                                .send().await.unwrap();
                            },  
                            ak::JoinBroadcast::TOKEN => {
                                let the_data = ak::JoinBroadcast::new(more_data);
                                let mut who: String = the_data.name().to_str().unwrap_or("Invalid UTF-8").to_owned().escape_default().collect();
                                who = who.replace("\\'", "'");
                                let body = format!("{{\"content\": \"New user {}[id:{}]\", \"username\": \"Information\", \"allowed_mentions\": {{\"parse\": []}}}}", who, the_data.id());
                                println!("Sending data to a webhook: {}", body);
                                client.post(std::env::var("DISCORD_WEBHOOK").unwrap())
                                .body(body)
                                .send().await.unwrap();
                                players.insert(the_data.id(), the_data);
                            },
                            ak::ExitBroadcast::TOKEN => {
                                let the_data = ak::ExitBroadcast::new(more_data);
                                let mut who: String = players[&the_data.id()].name().to_str().unwrap_or("Invalid UTF-8").to_owned().escape_default().collect();
                                who = who.replace("\\'", "'");
                                let body = format!("{{\"content\": \"{}[id:{}] has left\", \"username\": \"Information\", \"allowed_mentions\": {{\"parse\": []}}}}", who, the_data.id());
                                println!("Sending data to a webhook: {}", body);
                                client.post(std::env::var("DISCORD_WEBHOOK").unwrap())
                                .body(body)
                                .send().await.unwrap();
                                players.remove(&the_data.id());
                            },
                            _ => continue
                        },
                        _ => continue
                    }
                },
                Err(_) => continue
            }
        }
    }

    #[allow(unreachable_code)]
    Ok(())
}
