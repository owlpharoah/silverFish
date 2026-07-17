use reqwest::{Client, header::CONTENT_TYPE};
use scraper::Html;
use url::Url;

#[derive(Debug)]
pub enum FetchError {
    Request(reqwest::Error),
    NotHtml,
}

impl From<reqwest::Error> for FetchError {
    fn from(err: reqwest::Error) -> Self {
        FetchError::Request(err)
    }
}

pub struct Fetcher {
    pub client: Client,
}

impl Fetcher {
    pub async fn fetch(&self, url: &Url) -> Result<Html, FetchError> {
        let response = self
            .client
            .get(url.as_str())
            .send()
            .await?
            .error_for_status()?;

        let is_html = response
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .is_some_and(|ct| ct.starts_with("text/html"));

        if !is_html {
            return Err(FetchError::NotHtml);
        }

        let html = response.text().await?;
        Ok(Html::parse_document(&html))
    }
}
