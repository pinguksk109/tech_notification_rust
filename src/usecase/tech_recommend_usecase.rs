use log::{info, LevelFilter};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::collections::HashMap;
use crate::repository::qiita_api_repository::{QiitaApiRepository};
use crate::repository::zenn_api_repository::{ZennApiRepository};
use crate::domain::item::Item;

#[derive(Serialize, Deserialize)]
pub struct QiitaRecommendOutput {
    pub items: Vec<Item>,
}

#[derive(Serialize, Deserialize)]
pub struct ZennRecommendOutput {
    pub items: Vec<Item>,
}

pub struct TechRecommendUsecase<'a> {
    qiita_api_repository: &'a QiitaApiRepository,
    zenn_api_repository: &'a ZennApiRepository,
    qiita_target_page_count: u32,
    zenn_target_page_count: u32,
}

impl <'a> TechRecommendUsecase<'a> {
    pub fn new(qiita_api_repository: &'a QiitaApiRepository, zenn_api_repository: &'a ZennApiRepository) -> Self {
        TechRecommendUsecase{
            qiita_api_repository,
            zenn_api_repository,
            qiita_target_page_count: 10,
            zenn_target_page_count: 10,
        }
    }

    pub async fn qiita_handle(&self) -> QiitaRecommendOutput {
        let mut items = Vec::new();

        for page in 1..=self.qiita_target_page_count {
            match self.get_qiita_items(page).await {
                Ok(data) => {
                    if let Some(articles) = data.as_array() {
                        for item in articles {
                            if let Some(likes_count) = item.get("likes_count").and_then(|v| v.as_u64()) {
                                if likes_count >= 3 {
                                    if let (Some(title), Some(url)) = (
                                        item.get("title").and_then(|v| v.as_str()),
                                        item.get("url").and_then(|v| v.as_str()),
                                    ) {
                                        items.push(Item {
                                            title: title.to_string(),
                                            url: url.to_string(),
                                            likes_count: likes_count as u32,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error fetching Qiita items on page {}: {:?}", page, e);
                }
            }
        }

        let ranking_5items = Item::get_5ranking_items(&items);
        QiitaRecommendOutput { items: ranking_5items }
    }

    pub async fn zenn_handle(&self) -> ZennRecommendOutput {
        let mut items = Vec::new();

        for page in 1..=self.zenn_target_page_count {
            let response = self.get_zenn_items(page);
            match response.await {
                Ok(data) => {
                    if let Some(articles) = data["articles"].as_array() {
                        for article in articles {
                            if let Some(liked_count) = article["liked_count"].as_u64() {
                                if liked_count >= 3 {
                                    if let (Some(title), Some(path)) = (
                                        article["title"].as_str(),
                                        article["path"].as_str(),
                                    ) {
                                        let item = Item {
                                            title: title.to_string(),
                                            url: format!("https://zenn.dev{}", path),
                                            likes_count: liked_count as u32,
                                        };
                                        items.push(item);
                                    }
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error fetching Zenn items: {:?}", e);
                }
            }
        }

        let ranking_5items = Item::get_5ranking_items(&items);
        ZennRecommendOutput { items: ranking_5items }
    }

    async fn get_qiita_items(&self, page_number: u32) -> Result<Value, Box<dyn std::error::Error>> {
        let response = self.qiita_api_repository.get_items(page_number).await;
        info!("Qiita {}ページ目取得完了", page_number);
        response
    }

    async fn get_zenn_items(&self, page_number: u32) -> Result<Value, Box<dyn std::error::Error>> {
        let response = self.zenn_api_repository.get_items(page_number).await;
        info!("Zenn {}ページ目取得完了", page_number);
        response
    }
}