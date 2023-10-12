mod ai;

use std::collections::HashMap;
use std::ops::Add;
use std::str::FromStr;
use ansi_term::Style;
use base64::encode;
use log::{debug, error};
use serde::{Deserialize, Serialize};
use teloxide::net::{Download, download_file};
use teloxide::payloads::GetFile;
use teloxide::prelude::*;
use teloxide::types::MessageEntityKind::Underline;
use teloxide::types::ParseMode;
use teloxide::types::ParseMode::{Html, MarkdownV2};
use teloxide::utils::html::underline;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use crate::ai::{AccuracyRequest, AccuracyResponse};

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

                error!("{}", base64_encoded);

                let sample_string = "That, isn't important for you to know.";

                let body = AccuracyRequest {
                    base64audio: "data:audio/ogg;;base64,".to_string() + &base64_encoded,
                    language: "en".to_string(),
                    title: sample_string.to_string(),
                };

                let client = reqwest::Client::new();
                let response: AccuracyResponse = client.post("http://localhost:3000/GetAccuracyFromRecordedAudio")
                    .json(&body)
                    .send()
                    .await?
                    .json()
                    .await?;

                fn highlight_zeros(text1: &str, text2: String) -> String {
                    text1
                        .chars()
                        .zip(text2.chars())
                        .map(|(c1, c2)| {
                            if c2 == '0' {
                                format!("<u>{c1}</u>")
                            } else {
                                c1.to_string()
                            }
                        })
                        .collect()
                }

                let highlight = highlight_zeros(sample_string, response.is_letter_correct_all_words);
                let finaltext = format!("{highlight}\n\
                Pronunciation accuracy: {}%\n\
                Say please again: \"{}\"", response.pronunciation_accuracy, sample_string);

                // Send a message with the result
                bot.send_message(msg.chat.id, finaltext).parse_mode(Html).await?

            }
            None => {
                bot.send_message(msg.chat.id, "Send a voice where you will say: \"That, isn't important for you to know.\"").await?
            }
        };
        Ok(())
    }).await;
}