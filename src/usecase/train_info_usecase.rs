use serde::Serialize;
use std::error::Error;
use crate::repository::scraper_repository::ScraperRepository;

#[derive(Serialize)]
pub struct TrainInfoOutput {
    pub abnormal_train: Vec<String>,
}

pub struct TrainInfoUsecase {
    scraper_repository: ScraperRepository,
}

impl TrainInfoUsecase {
    pub fn new(scraper_repository: ScraperRepository) -> Self {
        TrainInfoUsecase { scraper_repository }
    }

    pub async fn handle(&self) -> Result<TrainInfoOutput, Box<dyn std::error::Error + Send + Sync>> {
        let target_url = "https://subway.osakametro.co.jp/guide/subway_information.php";
        let html_content = self.scraper_repository.fetch_content(target_url).await?;
        let abnormal_train_list = self
            .scraper_repository
            .parse_all_lines_status(&html_content)?;
        Ok(TrainInfoOutput {
            abnormal_train: abnormal_train_list,
        })
    }
}