use asciicker_rs::y6::prelude::*;
use asciicker_rs::macro_rules_attribute::apply;
use asciicker_rs::callback;
use tokio::sync::Mutex;
use std::sync::Arc;
use lazy_static::lazy_static;
use reqwest::{
	Client,
	header::{
		HeaderMap,
		HeaderValue,
		CONTENT_TYPE
	}
};
use serde_json::json;

lazy_static! {
	static ref DISCORD_WEBHOOK: String = std::env::var("DISCORD_WEBHOOK").unwrap();
	static ref REQWEST_CLIENT: Client = {
		let mut headers = HeaderMap::default();
		headers.insert(
			CONTENT_TYPE,
			HeaderValue::from_str("application/json").expect("Failed to construct a header value")
		);
		let cb = Client::builder()
					.default_headers(headers)
					.build()
					.expect("Failed to build reqwest client");
		cb
	};
}

const ASCIICKER_SERVER: &'static str = "ws://asciicker.com/ws/y6/";

#[tokio::main]
async fn main() {
	let mut bot = Bot::new("logger", ASCIICKER_SERVER, true);
	bot.on_join(join_callback);
	bot.on_exit(exit_callback);
	bot.on_talk(talk_callback);
	let (threads, data) = match bot.run().await {
		Err(e) => panic!("Failed to run the bot: {:?}", e),
		Ok(stuff) => stuff,
	};
	data.0.lock().await.pose.position[2] = -300f32; // Bury the bot so real players don't see it.
	println!("{:?}", threads.0.thread.await);
}

#[apply(callback!)]
async fn join_callback(join_brc: JoinBroadcast, _bot: Arc<Mutex<Player>>, _world: Arc<Mutex<World>>, _message_sender: MessageSender) -> BotResult {
	let message = format!("{}[ID:{}] has joined the server.", join_brc.name.to_string_lossy(), join_brc.id);
	let body = json!({
		"content": message,
		"username": "Discord to asciicker bridge",
		"allowed_mentions": {
			"parse": []
		}
	});
	REQWEST_CLIENT.post(DISCORD_WEBHOOK.clone()).body(body.to_string()).send().await.unwrap();
	Ok(())
}

#[apply(callback!)]
async fn exit_callback(exit_brc: ExitBroadcast, _bot: Arc<Mutex<Player>>, world: Arc<Mutex<World>>, _message_sender: MessageSender) -> BotResult {
	let world = world.lock().await;
	let player = world.clients.iter().filter(|p|p.id == exit_brc.id).next().unwrap_or_else(||unreachable!());
	let message = format!("{}[ID:{}] has left the server.", player.nickname, player.id);
	let body = json!({
		"content": message,
		"username": "Discord to asciicker bridge",
		"allowed_mentions": {
			"parse": []
		}
	});
	REQWEST_CLIENT.post(DISCORD_WEBHOOK.clone()).body(body.to_string()).send().await.unwrap();
	Ok(())
}

#[apply(callback!)]
async fn talk_callback(talk_brc: TalkBroadcast, _bot: Arc<Mutex<Player>>, world: Arc<Mutex<World>>, _message_sender: MessageSender) -> BotResult {
	let world = world.lock().await;
	let player = world.clients.iter().filter(|p|p.id == talk_brc.id).next().unwrap_or_else(||unreachable!());
	let message = talk_brc.str.to_string_lossy();
	let name = format!("{}[ID:{}]", player.nickname, player.id);
	let body = json!({
		"content": message,
		"username": name,
		"allowed_mentions": {
			"parse": []
		}
	});
	REQWEST_CLIENT.post(DISCORD_WEBHOOK.clone()).body(body.to_string()).send().await.unwrap();
	Ok(())
}