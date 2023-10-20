use dotenv::dotenv;
use serde_json::json;
use std::env;
use teloxide::types::{
    InputFile, InputMediaPhoto, InputMediaVideo, MessageEntity, MessageEntityKind, ParseMode,
};
use teloxide::{prelude::*, types::InputMedia};
use url::Url;
use vidbot::{remove_file, Downloader, Instagram, Tweet, Variant};
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let bot_token = env::var("BOT_TOKEN").expect("You should set to bot_token to .env file!");
    log::info!("Starting bot...");

    let bot = Bot::new(bot_token);
    let handler = Update::filter_message().endpoint(message_handler);
    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

#[allow(unused_must_use)]
async fn message_handler(bot: Bot, msg: Message) -> HandlerResult {
    match msg.text() {
        Some(text) if Tweet::regex().is_match(text) => {
            let down_msg = bot.send_message(msg.chat.id, "Downloading...").await?;
            let caps = Tweet::regex().captures(text).unwrap();
            let tweet = Tweet::new(caps["id"].to_string());
            let post = tweet.fetch_data().await.unwrap();
            let mut send_media = vec![];
            if post.get("__typename").unwrap_or(&json!({"status":"err"}))
                == &json!({"status":"err"})
            {
                return Ok(());
            }
            if post["__typename"].as_str().unwrap() != "Tweet" {
                bot.send_message(msg.chat.id, "<b>This content is Protected or NSFW.</b>")
                    .reply_to_message_id(msg.id)
                    .parse_mode(ParseMode::Html)
                    .await?;
                bot.delete_message(msg.chat.id, down_msg.id).await?;
                return Ok(());
            }

            for media in post["legacy"]["extended_entities"]["media"]
                .as_array()
                .unwrap()
            {
                if media["type"].as_str().unwrap() == "video" {
                    let mut variants: Vec<Variant> = serde_json::from_value(
                        media
                            .get("video_info")
                            .unwrap()
                            .get("variants")
                            .unwrap()
                            .clone(),
                    )
                    .unwrap();
                    let mpeg = variants
                        .iter()
                        .position(|x| x.content_type == String::from("application/x-mpegURL"))
                        .unwrap();
                    variants.remove(mpeg);
                    variants.sort_by(|a, b| b.bitrate.unwrap().cmp(&a.bitrate.unwrap()));
                    println!("{:#?}", variants);
                    let vid = InputFile::url(Url::parse(variants[0].url.as_str()).unwrap());
                    let thumb = InputFile::url(
                        Url::parse(media["media_url_https"].as_str().unwrap()).unwrap(),
                    );
                    let input_media = InputMedia::Video(InputMediaVideo {
                        media: vid,
                        thumb: Some(thumb),
                        caption: Some(String::new()),
                        has_spoiler: false,
                        supports_streaming: Some(true),
                        parse_mode: Some(ParseMode::Html),
                        caption_entities: Some(vec![MessageEntity {
                            length: 0,
                            offset: 0,
                            kind: MessageEntityKind::Bold,
                        }]),
                        duration: Some(
                            media["video_info"]["duration_millis"].as_u64().unwrap_or(0) as u16,
                        ),
                        width: Some(media["original_info"]["width"].as_u64().unwrap_or(1000) as u16),
                        height: Some(
                            media["original_info"]["height"].as_u64().unwrap_or(1000) as u16
                        ),
                    });
                    send_media.push(input_media);
                } else {
                    let photo = InputFile::url(
                        Url::parse(media["media_url_https"].as_str().unwrap()).unwrap(),
                    );
                    let input_media = InputMedia::Photo(InputMediaPhoto {
                        media: photo,
                        caption: Some(String::new()),
                        has_spoiler: false,
                        parse_mode: Some(ParseMode::Html),
                        caption_entities: Some(vec![MessageEntity {
                            length: 0,
                            offset: 0,
                            kind: MessageEntityKind::Bold,
                        }]),
                    });
                    send_media.push(input_media);
                }
            }
            bot.send_media_group(msg.chat.id, send_media.into_iter())
                .await?;
            bot.delete_message(msg.chat.id, down_msg.id).await?;
        }
        Some(text) if Instagram::regex().is_match(text) => {
            let down_msg = bot.send_message(msg.chat.id, "Downloading...").await?;
            let caps = Instagram::regex().captures(text).unwrap();
            let post = Instagram::new(caps["id"].to_string())
                .fetch_data()
                .await
                .unwrap();
            let mut send_media = vec![];
            let post = &post["data"]["shortcode_media"];

            if post.get("edge_sidecar_to_children").is_some() {
                for node in post["edge_sidecar_to_children"]["edges"]
                    .as_array()
                    .unwrap()
                {
                    let node = &node["node"];
                    if node["is_video"].as_bool().unwrap() {
                        let filename = node["shortcode"].as_str().unwrap();
                        let down = Downloader::new(
                            filename,
                            // node["shortcode"].as_str().unwrap(),
                            node["video_url"].as_str().unwrap(),
                        );
                        down.download().await;
                        let vid = InputFile::file(down.filename);
                        let thumb = InputFile::url(
                            Url::parse(node["display_url"].as_str().unwrap()).unwrap(),
                        );

                        let input_media = InputMedia::Video(InputMediaVideo {
                            media: vid,
                            thumb: Some(thumb),
                            caption: Some(String::new()),
                            parse_mode: Some(ParseMode::Html),
                            caption_entities: Some(vec![MessageEntity {
                                length: 0,
                                offset: 0,
                                kind: MessageEntityKind::Bold,
                            }]),
                            width: Some(node["dimensions"]["width"].as_u64().unwrap_or(1000) as u16),
                            height: Some(
                                node["dimensions"]["height"].as_u64().unwrap_or(1000) as u16
                            ),
                            duration: Some(200),
                            supports_streaming: Some(true),
                            has_spoiler: false,
                        });
                        println!("{:#?}", input_media);
                        send_media.push(input_media);
                    } else {
                        let photo = InputFile::url(
                            Url::parse(node["display_url"].as_str().unwrap()).unwrap(),
                        );
                        let input_media = InputMedia::Photo(InputMediaPhoto {
                            media: photo,
                            caption: Some(String::from("")),
                            parse_mode: Some(ParseMode::Html),
                            caption_entities: Some(vec![MessageEntity::bold(1, 1)]),
                            has_spoiler: false,
                        });
                        send_media.push(input_media);
                    }
                }
            } else {
                if post["is_video"].as_bool().unwrap() {
                    let down = Downloader::new(
                        post["shortcode"].as_str().unwrap(),
                        post["video_url"].as_str().unwrap(),
                    );
                    down.download().await;

                    let vid = InputFile::file(down.filename);
                    let thumb = InputFile::url(
                        Url::parse(post["thumbnail_src"].as_str().unwrap()).unwrap(),
                    );
                    let med = InputMedia::Video(InputMediaVideo {
                        media: vid,
                        thumb: Some(thumb),
                        caption: Some(String::new()),
                        has_spoiler: false,
                        supports_streaming: Some(true),
                        parse_mode: Some(ParseMode::Html),
                        caption_entities: Some(vec![MessageEntity {
                            length: 0,
                            offset: 0,
                            kind: MessageEntityKind::Bold,
                        }]),
                        duration: Some(post["video_duration"].as_u64().unwrap_or(0) as u16),
                        width: Some(post["dimension"]["width"].as_u64().unwrap_or(1000) as u16),
                        height: Some(post["dimension"]["height"].as_u64().unwrap_or(1000) as u16),
                    });
                    send_media.push(med);
                } else {
                    let photo =
                        InputFile::url(Url::parse(post["display_url"].as_str().unwrap()).unwrap());
                    let input_media = InputMedia::Photo(InputMediaPhoto {
                        media: photo,
                        caption: Some(String::from("")),
                        parse_mode: Some(ParseMode::Html),
                        caption_entities: Some(vec![MessageEntity::bold(1, 1)]),
                        has_spoiler: false,
                    });
                    send_media.push(input_media);
                }
            }
            bot.send_media_group(msg.chat.id, send_media.into_iter())
                .await?;
            bot.delete_message(msg.chat.id, down_msg.id).await?;
        }
        Some(_) => {}
        None => {
            bot.send_message(msg.chat.id, format!("No matched data"))
                .await?;
        }
    }
    remove_file();
    Ok(())
}
