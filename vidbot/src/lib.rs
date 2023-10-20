use dotenv::dotenv;
use regex::Regex;
use reqwest::{
    self,
    header::{HeaderMap, AUTHORIZATION},
};
use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use std::{env, error::Error, fs};

pub struct Tweet {
    id: String,
}

impl Tweet {
    pub fn new(id: String) -> Self {
        Self { id }
    }
    fn bearer_token(&self) -> Result<String, Box<dyn Error>> {
        log::info!("Get BEARER TOKEN from env");
        dotenv().ok();
        let bearer_token =
            env::var("TW_BEARER_TOKEN").expect("You should set to TW_BEARER_TOKEN to .env file");
        Ok(bearer_token)
    }
    pub async fn fetch_data(&self) -> Result<Value, Box<dyn Error>> {
        log::info!("Fetching Tweets");
        let bearer_token = self.bearer_token().unwrap();
        // let url = format!("https://twitter.com/i/api/graphql/2ICDjqPd81tulZcYrtpTuQ/TweetResultByRestId?variables=%7B%22tweetId%22%3A%22${}%22%2C%22withCommunity%22%3Afalse%2C%22includePromotedContent%22%3Afalse%2C%22withVoice%22%3Afalse%7D&features=%7B%22creator_subscriptions_tweet_preview_api_enabled%22%3Atrue%2C%22tweetypie_unmention_optimization_enabled%22%3Atrue%2C%22responsive_web_edit_tweet_api_enabled%22%3Atrue%2C%22graphql_is_translatable_rweb_tweet_is_translatable_enabled%22%3Atrue%2C%22view_counts_everywhere_api_enabled%22%3Atrue%2C%22longform_notetweets_consumption_enabled%22%3Atrue%2C%22responsive_web_twitter_article_tweet_consumption_enabled%22%3Afalse%2C%22tweet_awards_web_tipping_enabled%22%3Afalse%2C%22freedom_of_speech_not_reach_fetch_enabled%22%3Atrue%2C%22standardized_nudges_misinfo%22%3Atrue%2C%22tweet_with_visibility_results_prefer_gql_limited_actions_policy_enabled%22%3Atrue%2C%22longform_notetweets_rich_text_read_enabled%22%3Atrue%2C%22longform_notetweets_inline_media_enabled%22%3Atrue%2C%22responsive_web_graphql_exclude_directive_enabled%22%3Atrue%2C%22verified_phone_label_enabled%22%3Afalse%2C%22responsive_web_media_download_video_enabled%22%3Afalse%2C%22responsive_web_graphql_skip_user_profile_image_extensions_enabled%22%3Afalse%2C%22responsive_web_graphql_timeline_navigation_enabled%22%3Atrue%2C%22responsive_web_enhance_cards_enabled%22%3Afalse%7D&fieldToggles=%7B%22withArticleRichContentState%22%3Afalse%7D",self.id);
        let url = format!("https://twitter.com/i/api/graphql/DJS3BdhUhcaEpZ7B7irJDg/TweetResultByRestId?variables=%7B%22tweetId%22%3A%22{}%22%2C%22withCommunity%22%3Afalse%2C%22includePromotedContent%22%3Afalse%2C%22withVoice%22%3Afalse%7D&features=%7B%22creator_subscriptions_tweet_preview_api_enabled%22%3Atrue%2C%22tweetypie_unmention_optimization_enabled%22%3Atrue%2C%22responsive_web_edit_tweet_api_enabled%22%3Atrue%2C%22graphql_is_translatable_rweb_tweet_is_translatable_enabled%22%3Atrue%2C%22view_counts_everywhere_api_enabled%22%3Atrue%2C%22longform_notetweets_consumption_enabled%22%3Atrue%2C%22responsive_web_twitter_article_tweet_consumption_enabled%22%3Afalse%2C%22tweet_awards_web_tipping_enabled%22%3Afalse%2C%22freedom_of_speech_not_reach_fetch_enabled%22%3Atrue%2C%22standardized_nudges_misinfo%22%3Atrue%2C%22tweet_with_visibility_results_prefer_gql_limited_actions_policy_enabled%22%3Atrue%2C%22longform_notetweets_rich_text_read_enabled%22%3Atrue%2C%22longform_notetweets_inline_media_enabled%22%3Atrue%2C%22responsive_web_graphql_exclude_directive_enabled%22%3Atrue%2C%22verified_phone_label_enabled%22%3Afalse%2C%22responsive_web_media_download_video_enabled%22%3Afalse%2C%22responsive_web_graphql_skip_user_profile_image_extensions_enabled%22%3Afalse%2C%22responsive_web_graphql_timeline_navigation_enabled%22%3Atrue%2C%22responsive_web_enhance_cards_enabled%22%3Afalse%7D", self.id);
        let guest_token = self.guest_token(&bearer_token).await?;
        let mut header = HeaderMap::new();
        header.insert("Content-type", "application/json".parse().unwrap());
        header.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.0.0 Safari/537.36".parse().unwrap());
        header.insert("x-twitter-active-user", "yes".parse().unwrap());
        header.insert("X-Guest-Token", guest_token.parse().unwrap());
        header.insert("referrer", "https://twitter.com".parse().unwrap());
        header.insert(
            AUTHORIZATION,
            format!("Bearer {}", bearer_token).parse().unwrap(),
        );
        let cl = reqwest::Client::new();
        let resp = cl.get(url).headers(header).send().await?;
        let mut json = resp.json::<Value>().await?;
        Ok(json["data"]["tweetResult"]["result"].take())
    }

    async fn guest_token(&self, token: &String) -> Result<String, Box<dyn Error>> {
        let mut header = HeaderMap::new();
        header.insert(AUTHORIZATION, format!("Bearer {}", token).parse().unwrap());
        let cl = reqwest::Client::new();
        println!("guest token");

        let resp = cl
            .post("https://api.twitter.com/1.1/guest/activate.json")
            .headers(header)
            .send()
            .await?;
        let json = resp.json::<Token>().await?;
        Ok(json.guest_token)
    }
    pub fn regex() -> Regex {
        Regex::new(r"((https?)://)?(www\.)?(twitter|x)\.com/(\w+)/status/(?<id>\d+)").unwrap()
    }
}

pub struct Instagram {
    id: String,
}

impl Instagram {
    pub fn new(id: String) -> Self {
        Self { id }
    }

    pub async fn fetch_data(&self) -> Result<Value, Box<dyn Error>> {
        let url = format!(
            r##"https:\/\/www.instagram.com/graphql/query/?query_hash=cf28bf5eb45d62d4dc8e77cdb99d750d&variables={}"shortcode":"{}"{}"##,
            "{", self.id, "}"
        );
        let resp = reqwest::get(url).await?;
        let json = resp.json::<Value>().await?;
        Ok(json)
    }
    pub fn regex() -> Regex {
        Regex::new(r"((?:https?://)?(?:www\.)?instagram\.com/(?:p|reel)/(?<id>[^/?#&]+))").unwrap()
    }
}

pub struct Downloader {
    pub filename: String,
    url: String,
}

impl Downloader {
    pub fn new(filename: &str, url: &str) -> Self {
        Self {
            filename: format!("videos/{}.mp4", filename),
            url: url.to_string(),
        }
    }

    pub async fn download(&self) -> Result<(), Box<dyn Error>> {
        // let file = File::create(self.url.as_str()).unwrap();
        let bytes = reqwest::get(self.url.as_str()).await?.bytes().await?;
        fs::write(self.filename.as_str(), bytes).unwrap();
        Ok(())
    }
}
pub fn remove_file() -> Result<(), Box<dyn Error>> {
    fs::remove_dir_all("videos").and_then(|_| fs::create_dir("videos"))?;
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct Token {
    guest_token: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtendedEntitiesVideo {
    pub media: Vec<Medum>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtendedEntitiesPhoto {
    pub media: Vec<MedumPhoto>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Medum {
    #[serde(rename = "display_url")]
    pub display_url: String,
    #[serde(rename = "expanded_url")]
    pub expanded_url: String,
    #[serde(rename = "id_str")]
    pub id_str: String,
    pub indices: Vec<i64>,
    #[serde(rename = "media_key")]
    pub media_key: String,
    #[serde(rename = "media_url_https")]
    pub media_url_https: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub url: String,
    #[serde(rename = "additional_media_info")]
    pub additional_media_info: AdditionalMediaInfo,
    pub media_stats: MediaStats,
    #[serde(rename = "ext_media_availability")]
    pub ext_media_availability: ExtMediaAvailability,
    pub features: Features,
    pub sizes: Sizes,
    #[serde(rename = "original_info")]
    pub original_info: OriginalInfo,
    #[serde(rename = "video_info")]
    pub video_info: VideoInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedumPhoto {
    #[serde(rename = "display_url")]
    pub display_url: String,
    #[serde(rename = "expanded_url")]
    pub expanded_url: String,
    #[serde(rename = "id_str")]
    pub id_str: String,
    pub indices: Vec<i64>,
    #[serde(rename = "media_key")]
    pub media_key: String,
    #[serde(rename = "media_url_https")]
    pub media_url_https: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub url: String,
    #[serde(rename = "ext_media_availability")]
    pub ext_media_availability: ExtMediaAvailability,
    pub features: Features,
    pub sizes: Sizes,
    #[serde(rename = "original_info")]
    pub original_info: OriginalInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalMediaInfo {
    pub monetizable: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaStats {
    pub view_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtMediaAvailability {
    pub status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Features {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sizes {
    pub large: Large,
    pub medium: Medium,
    pub small: Small,
    pub thumb: Thumb,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Large {
    pub h: i64,
    pub w: i64,
    pub resize: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Medium {
    pub h: i64,
    pub w: i64,
    pub resize: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Small {
    pub h: i64,
    pub w: i64,
    pub resize: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumb {
    pub h: i64,
    pub w: i64,
    pub resize: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OriginalInfo {
    pub height: i64,
    pub width: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoInfo {
    #[serde(rename = "aspect_ratio")]
    pub aspect_ratio: Vec<i64>,
    #[serde(rename = "duration_millis")]
    pub duration_millis: i64,
    pub variants: Vec<Variant>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Variant {
    pub bitrate: Option<i64>,
    #[serde(rename = "content_type")]
    pub content_type: String,
    pub url: String,
}
