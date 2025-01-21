use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    pub title: String,
    pub url: String,
    pub likes_count: u32,
}

impl Item {
    pub fn get_5ranking_items(items: &[Item]) -> Vec<Item> {
        let mut sorted_items = items.to_vec();
        sorted_items.sort_by(|a, b| b.likes_count.cmp(&a.likes_count));
        sorted_items.into_iter().take(5).collect()
    }
}