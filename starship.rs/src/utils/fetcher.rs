use anyhow::{Context, Result};
use once_cell::sync::Lazy;
use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{Duration, SystemTime};

// Shared HTTP client with connection pooling
#[allow(dead_code)]
static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .timeout(Duration::from_secs(30))
        .pool_max_idle_per_host(10)
        .tcp_keepalive(Duration::from_secs(60))
        .build()
        .unwrap_or_else(|_| Client::new())
});

#[allow(dead_code)]
#[derive(Clone)]
pub struct DocumentationCache {
    cache: Arc<RwLock<HashMap<String, (String, SystemTime)>>>,
    cache_ttl: Duration,
}

impl DocumentationCache {
    pub fn new() -> Self {
        Self::with_ttl(Duration::from_secs(3600))
    }

    pub fn with_ttl(cache_ttl: Duration) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            cache_ttl,
        }
    }

    #[allow(dead_code)]
    pub async fn fetch_html(&self, url: &str) -> Result<String> {
        // Check cache first
        {
            let cache = self.cache.read().await;
            if let Some((content, timestamp)) = cache.get(url) {
                if timestamp.elapsed().unwrap_or(Duration::MAX) < self.cache_ttl {
                    return Ok(content.clone());
                }
            }
        }

        // Fetch from network using shared client with connection pooling
        let response = HTTP_CLIENT
            .get(url)
            .send()
            .await
            .with_context(|| format!("Failed to fetch URL: {}", url))?;

        let content = response
            .text()
            .await
            .context("Failed to read response body")?;

        // Update cache
        {
            let mut cache = self.cache.write().await;
            cache.insert(url.to_string(), (content.clone(), SystemTime::now()));
        }

        Ok(content)
    }

    #[allow(dead_code)]
    pub async fn parse_html(&self, html: &str) -> Result<Html> {
        Ok(Html::parse_document(html))
    }

    #[allow(dead_code)]
    pub fn extract_text_by_selector(&self, html: &Html, selector_str: &str) -> Result<Vec<String>> {
        let selector = Selector::parse(selector_str)
            .map_err(|e| anyhow::anyhow!("Invalid selector '{}': {:?}", selector_str, e))?;

        let mut results = Vec::new();
        for element in html.select(&selector) {
            let text = element.text().collect::<String>();
            if !text.trim().is_empty() {
                results.push(text.trim().to_string());
            }
        }
        Ok(results)
    }

    #[allow(dead_code)]
    pub fn extract_links(&self, html: &Html, base_url: &str) -> Result<Vec<String>> {
        let selector = Selector::parse("a[href]")
            .map_err(|e| anyhow::anyhow!("Failed to parse link selector: {:?}", e))?;

        let mut links = Vec::new();
        for element in html.select(&selector) {
            if let Some(href) = element.value().attr("href") {
                let url = if href.starts_with("http") {
                    href.to_string()
                } else if href.starts_with('/') {
                    format!("https://starship.rs{}", href)
                } else {
                    format!("{}/{}", base_url.trim_end_matches('/'), href)
                };
                links.push(url);
            }
        }
        Ok(links)
    }

    #[allow(dead_code)]
    pub async fn clear_cache(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
    }
}

impl Default for DocumentationCache {
    fn default() -> Self {
        Self::new()
    }
}

