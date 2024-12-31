use reqwest::StatusCode;
use serde_json::Value;

pub struct WeatherRepository;

impl WeatherRepository {
    pub fn new () -> Self {
        WeatherRepository
    }

    pub async fn fetch(&self, area_code: u32) -> Result<Value, String> {
        let url = format!(
            "https://www.jma.go.jp/bosai/forecast/data/forecast/{}.json",
            area_code
        );

        let response = reqwest::get(&url).await.map_err(|err| format!("HTTPリクエストエラー: {}", err))?;

        if response.status() != StatusCode::OK {
            let status_code = response.status();
            let response_text = response.text().await.unwrap_or_default();
            return Err(format!(
                "気象庁APIから200以外が返却されました。ステータスコード: {}, レスポンス内容: {}",
                status_code, response_text
            ));
        }

        let json_data: Value = response.json().await.map_err(|err| format!("JSON解析エラー: {}", err))?;
        Ok(json_data)
    }
}