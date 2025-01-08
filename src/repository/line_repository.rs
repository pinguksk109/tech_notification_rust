use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, AUTHORIZATION};
use serde_json::json;
use std::env;
use std::error::Error;

pub struct LineRepository {
    to: String,
    beader_token: String,
}
