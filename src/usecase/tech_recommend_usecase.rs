use log::{info, LevelFilter};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::repository::{QiitaApiRepository};
use crate::domain::item::Items;
use crate::application::base::IOutput;

#[derive(Serialize, Deserialize)]
pub struct QiitaRecommendOutput {
    pub items: Vec<Item>,
}
