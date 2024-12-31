mod repository;
mod usecase;

use repository::weather_repository::WeatherRepository;
use repository::scraper_repository::ScraperRepository;
use usecase::weather_usecase::WeatherUsecase;
use usecase::train_info_usecase::TrainInfoUsecase;

#[tokio::main]
async fn main() {
    let weather_repository = WeatherRepository::new();
    let weather_usecase = WeatherUsecase::new(weather_repository);
    
    match weather_usecase.handle().await {
        Ok(response) => {
            println!("取得したデータ: {}", response.forecast);
        }
        Err(err) => {
            eprintln!("エラーが発生しました: {}", err)
        }
    }

    let scraper_repository = ScraperRepository::new();
    let train_info_usecase = TrainInfoUsecase::new(scraper_repository);

    match train_info_usecase.handle().await {
        Ok(response) => {
            println!("取得したデータ: {:?}", response.abnormal_train);
        }
        Err(err) => {
            eprintln!("エラーが発生しました: {}", err)
        }
    }
}


// use lambda_runtime::{handler_fn, Context, Error};
// use serde::{Deserialize, Serialize};

// #[derive(Deserialize)]
// struct Request {
//     pub name: String,
// }

// #[derive(Serialize)]
// struct Response {
//     pub message: String,
// }

// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     let func = handler_fn(my_handler);
//     lambda_runtime::run(func).await?;
//     Ok(())
// }

// async fn my_handler(event: Request, _: Context) -> Result<Response, Error> {
//     let name = event.name;
//     let message = format!("Hello, {}!", name);
//     Ok(Response { message })
// }