mod usecase;
mod repository;

use std::vec;

use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::Value;
use usecase::line_usecase::{LineSendInput, LineUsecase, Item};
use usecase::train_info_usecase::TrainInfoUsecase;
use usecase::weather_usecase::WeatherUsecase;
use repository::line_repository::{LineRepository};
use repository::scraper_repository::{ScraperRepository};
use repository::weather_repository::{WeatherRepository};

async fn lambda_handler(_event: LambdaEvent<Value>) -> Result<(String), Error> {
    let weather_repository = WeatherRepository::new();
    let weather_usecase = WeatherUsecase::new(weather_repository);

    let scraper_repository = ScraperRepository::new();
    let train_info_usecase = TrainInfoUsecase::new(scraper_repository);

    let line_repository = LineRepository::new()?;
    let line_usecase = LineUsecase::new(line_repository)?;

    // let line_repository = LineRepository::new()?;
    // let scraper_repository = ScraperRepository::new()?;
    // let weather_repository = WeatherRepository::new()?;

    // let weather_usecase = WeatherUsecase::new(weather_repository);
    // let train_info_usecase = TrainInfoUsecase::new(scraper_repository);
    // let line_usecase = LineUsecase::new(line_repository);

    let weather_output = weather_usecase.handle().await?;
    let abnormal_train_output = train_info_usecase.handle().await?;
    line_usecase
    .handle(LineSendInput {
        qiita_items: vec![
            Item {
                titiel: "Qiita記事1".to_string(),  // "title"の綴りを確認してください
                url: "http://example.com/qiita_article".to_string(),
            },
        ],
        zenn_items: vec![
            Item {
                titiel: "Zenn記事1".to_string(),
                url: "http://example.com/zenn_article".to_string(),
            },
        ],
        abnormal_train: abnormal_train_output.abnormal_train,
        weather_forecast: weather_output.forecast,
    })
    .await?;

    Ok("OK".to_string())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(lambda_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}