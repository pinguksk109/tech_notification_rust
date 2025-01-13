use serde::{Deserialize, Serialize};
use chrono::{Local, TimeZone};
use std::error::Error;

use crate::repository::{line_repository::{self, LineRepository}, weather_repository};

use super::train_info_usecase;

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub titiel: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LineSendInput {
    pub qiita_items: Vec<Item>,
    pub zenn_items: Vec<Item>,
    pub abnormal_train: Vec<String>,
    pub weather_forecast: String,
}

pub struct LineUsecase {
    line_repository: LineRepository,
    today_date: String,
}

impl LineUsecase {
    pub fn new(line_repository: LineRepository) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let today_date = Local::now()
            .format("%Y-%m-%d")
            .to_string();
        Ok(LineUsecase {
            line_repository,
            today_date,
        })
    }

    pub async fn handle(&self, input_data: LineSendInput) -> Result<(), Box<dyn Error>> {
        let weather_message = self.create_weather_forecast_message(&input_data.weather_forecast);
        self.line_repository.send_message(&weather_message).await?;

        let train_info_message = self.create_train_info_message(&input_data.abnormal_train);
        self.line_repository.send_message(&train_info_message).await?;

        Ok(())
    }

    // fn create_message(&self, items: &[Item], media: &str) -> String {
    //     let formatted_items: Vec<String> = items
    //         .iter()
    //         .enumerate()
    //         .map(|(i, item)| format!("{}. {} {}", i+1, item.title, item.url))
    //         .collect();

    //     format!(
    //         "{}の{}おすすめ記事を送ります✍\n\n{}",
    //         self.today_date,
    //         media,
    //         formatted_items.join("\n")
    //     )
    // }

    fn create_train_info_message(&self, abnormal_train: &[String]) -> String {
        if abnormal_train.is_empty() {
            format!(
                "{}: 大阪メトロの電車遅延はありませんでした",
                self.today_date
            )
        } else {
            let delayed_trains = abnormal_train.join(", ");
            format!(
                "{}: 以下の電車で遅延が発生しています: {}。詳細はこちら: https://subway.osakametro.co.jp/guide/subway_information.php",
                self.today_date, delayed_trains
            )
        }
    }

    fn create_weather_forecast_message(&self, weather_forecast: &str) -> String {
        format!(
            "{}の天気予報です: {}\n 詳しくはこちら: https://www.jma.go.jp/bosai/forecast/#area_type=offices&area_code=270000",
            self.today_date, weather_forecast
        )
    }
}