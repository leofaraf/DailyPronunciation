use std::collections::HashMap;
use base64::encode;
use log::debug;
use teloxide::net::{Download, download_file};
use teloxide::payloads::GetFile;
use teloxide::prelude::*;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;


#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    debug!("Starting throw dice bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        match msg.voice() {
            Some(voice) => {
                let download_path = "audio.ogg"; // Путь для загрузки голосового сообщения
                let base64_path = "base64.txt";

                let file = bot.get_file(&voice.file.id).await?;
                // Open a file for writing and pass a mutable reference to it
                let mut dst = File::create(download_path).await?;

                // Download the voice message and write it to the file
                bot.download_file(&file.path, &mut dst).await?;

                // Read the downloaded file into bytes
                let voice_bytes = tokio::fs::read(download_path).await?;

                // Encode the voice message as Base64
                let base64_encoded = encode(&voice_bytes);


                let mut map = HashMap::new();
                map.insert("base64Audio", "");
                map.insert("language", "en");
                map.insert("title", "That, isn't important for you to know.");


                let client = reqwest::Client::new();
                let response = client.post("localhost:3000/GetAccuracyFromRecordedAudio")
                    .body("&map")
                    .send()
                    .await?
                    .text()
                    .await?;

                println!("{}", response);
                // Send a message with the result
                bot.send_message(msg.chat.id, "Voice message successfully encoded to Base64.").await?
            }
            None => {
                bot.send_message(msg.chat.id, "Send a voice, please!").await?
            }
        };
        Ok(())
    }).await;
}