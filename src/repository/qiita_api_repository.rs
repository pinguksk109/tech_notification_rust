use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde_json::Value;
use std::error::Error;

pub struct QiitaApiRepository {
    client: Client,
}

impl QiitaApiRepository {
    pub fn new ()  -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn get_items(&self, page: u32) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!(
            "https://qiita.com/api/v2/items? page={}&per_page=100",
            page
        );

        //テスト用URL
        // let url = "http://localhost:3000/items";

        let response = self.client.get(&url).send()?;

        match response.status() {
            StatusCode::OK => {
                let json: Value = response.json()?;
                Ok(json)
            }
            StatusCode::FORBIDDEN => Err(format!(
                "Qiita APIから403(レートリミット)が返却されました。レスポンス内容: {}",
                response.text()?
            )
            .into()),
            _ => Err(format!(
                "Qiita APIから200以外が返却されました。ステータスコード: {} レスポンス内容: {}",
                response.status(),
                response.text()?
            )
            .into()),
        }
    }
}