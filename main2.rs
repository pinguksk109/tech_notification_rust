mod repository;

use repository::weather_repository::WeatherRepository;

#[tokio::main]
async fn main2() {
    let repository = WeatherRepository::new();
    
    let area_code = 130000;

    match repository.fetch(area_code).await {
        Ok(json_data) => {
            println!("取得したデータ: {:?}", json_data);
        }
        Err(err) => {
            eprintln!("エラーが発生しました: {}", err);
        }
    }
}
