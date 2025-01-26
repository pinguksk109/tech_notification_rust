use reqwest::{self, StatusCode};
use serde_json::Value;
use std::error::Error;

pub struct ZennApiRepository;

impl ZennApiRepository {
    pub fn new() -> Self {
        ZennApiRepository
    }

    pub async fn get_items(&self, page: u32) -> Result<Value, Box<dyn Error>> {
        let url = format!("https://zenn.dev/api/articles?order=latest&page={}", page);

        let response = reqwest::get(&url).await.map_err(|e| {
            format!("Zenn APIのリクエストに失敗しました エラー内容: {}", e)
        })?;

        if response.status() != StatusCode::OK {
            let status_code = response.status();
            let response_text = response.text().await.unwrap_or_else(|_| "内容取得失敗".to_string());
            return Err(format!(
                "Zenn APIから200以外が返却されました。ステータスコード: {} レスポンス内容: {}",
                status_code, response_text
            )
            .into());
        }
        
        let json_response = response.json::<Value>().await.map_err(|e| {
            format!("レスポンスのJSONパースに失敗しました。エラー内容: {}", e)
        })?;

        Ok(json_response)
    }
}