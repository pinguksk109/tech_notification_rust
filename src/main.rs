mod usecase;
mod repository;
mod domain;

use std::vec;

use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::Value;
use usecase::line_usecase::{LineSendInput, LineUsecase, Item};
use usecase::train_info_usecase::TrainInfoUsecase;
use usecase::weather_usecase::WeatherUsecase;
use usecase::tech_recommend_usecase::{TechRecommendUsecase, ZennRecommendOutput, QiitaRecommendOutput};
use repository::line_repository::{LineRepository};
use repository::scraper_repository::{ScraperRepository};
use repository::weather_repository::{WeatherRepository};
use repository::qiita_api_repository::{QiitaApiRepository};
use repository::zenn_api_repository::{ZennApiRepository};

fn convert_item(item: crate::domain::item::Item) -> crate::usecase::line_usecase::Item {
    crate::usecase::line_usecase::Item {
        title: item.title,
        url: item.url,
    }
}

async fn lambda_handler(_event: LambdaEvent<Value>) -> Result<(String), Error> {
    let weather_repository = WeatherRepository::new();
    let weather_usecase = WeatherUsecase::new(weather_repository);

    let scraper_repository = ScraperRepository::new();
    let train_info_usecase = TrainInfoUsecase::new(scraper_repository);

    let line_repository = LineRepository::new()?;
    let line_usecase = LineUsecase::new(line_repository)?;

    let qiita_api_repository = QiitaApiRepository::new();
    let zenn_api_repository = ZennApiRepository::new();
    let tech_recommend_usecase = TechRecommendUsecase::new(&qiita_api_repository, &zenn_api_repository);

    let qiita_output = tech_recommend_usecase.qiita_handle().await;
    let zenn_output = tech_recommend_usecase.zenn_handle().await;
    let weather_output = weather_usecase.handle().await?;
    let abnormal_train_output = train_info_usecase.handle().await?;

    let qiita_items: Vec<crate::usecase::line_usecase::Item> = qiita_output.items.into_iter().map(convert_item).collect();
    let zenn_items: Vec<crate::usecase::line_usecase::Item> = zenn_output.items.into_iter().map(convert_item).collect();

    line_usecase
    .handle(LineSendInput {
        qiita_items: qiita_items,
        zenn_items: zenn_items,
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