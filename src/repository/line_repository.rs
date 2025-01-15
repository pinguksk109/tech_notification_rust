use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, AUTHORIZATION};
use serde_json::json;
use std::env;
use std::error::Error;

pub struct LineRepository {
    to: String,
    bearer_token: String,
}

impl LineRepository {
    pub fn new() -> Result<Self, Box<dyn Error + Send + Sync>> {
        dotenv().ok();

        let to = env::var("LINE_USER_ID")
        .map_err(|_| "環境変数 LINE_USER_ID が設定されていません")?;
        let bearer_token = env::var("LINE_BEARER_TOKEN")
            .map_err(|_| "環境変数 LINE_BEARER_TOKEN が設定されていません")?;
        Ok(Self { to, bearer_token })
    }

    pub async fn send_message(&self, message: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        let url = "https://api.line.me/v2/bot/message/push";

        let payload = json!({
            "to": self.to,
            "messages": [
                {
                    "type": "text",
                    "text": message
                }
            ]
        });

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("{}", self.bearer_token))?,
        );

        let client = reqwest::Client::new();
        println!("{:?}", headers);
        let response = client.post(url).headers(headers).json(&payload).send().await?;
    
        if !response.status().is_success() {
            let status = response.status();
            let response_text = response.text().await.unwrap_or_else(|_| "不明なエラー".to_string());
            return Err(format!(
                "Line APIから成功以外のステータスコードが返却されました。ステータスコード: {} レスポンス内容: {}",
                status, response_text
            )
            .into());
        }

        Ok(())
    }
}