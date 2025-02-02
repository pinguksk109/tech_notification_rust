use reqwest::Client;
use scraper::{Html, Selector};

pub struct ScraperRepository;

impl ScraperRepository {
    pub fn new() -> Self {
        ScraperRepository
    }

    pub async fn fetch_content(&self, url: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let client = Client::new();
        let response = client.get(url).send().await?.error_for_status()?;
        let content = response.text().await?;
        Ok(content)
    }

    pub fn parse_all_lines_status(&self, html_content: &str) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
        let document = Html::parse_document(html_content);
        let subway_selector = Selector::parse("dd.headerMenuOperationArea_subway").unwrap();
        let line_name_selector = Selector::parse("li.subwayArea_LineName img").unwrap();
        let status_icon_selector = Selector::parse("li.subwayArea_Status img").unwrap();

        let mut abnormal_lines = Vec::new();

        for line in document.select(&subway_selector) {
            let line_name_img = line.select(&line_name_selector).next();
            let line_name = line_name_img
                .and_then(|img| img.value().attr("alt"))
                .unwrap_or("");

            let status_icon_img = line.select(&status_icon_selector).next();
            let status_icon_src = status_icon_img
                .and_then(|img| img.value().attr("src"))
                .unwrap_or("");

                if !status_icon_src.contains("icon_operation_normal.svg") {
                    abnormal_lines.push(line_name.to_string());
                }
            }

            Ok(abnormal_lines)
    }
}