use reqwest::Client;
use serde::Deserialize;
use std::env;

#[derive(Debug, serde::Serialize)]
pub struct NewsArticle {
    pub title: String,
    pub source: String,
    pub published_at: String,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CryptoPanicResponse {
    results: Vec<Article>,
}

#[derive(Debug, Deserialize)]
struct Article {
    title: String,
    published_at: String,
    domain: String,
    url: String,
    body: Option<String>,
}

pub async fn fetch_news_for_symbol(symbol: &str) -> Vec<NewsArticle> {
    let cp_key = env::var("CRYPTOPANIC_API_KEY").expect("CryptoPanic API key not found");
    // let cmc_key = env::var("COINMARKETCAP_API_KEY").expect("CoinMarketCap API key not found");
    // let url = if symbol.is_empty() {
    //     format!(
    //         "https://{}.coinmarketcap.com/v1/content/posts/top?symbol={}",
    //         cmc_key, symbol.to_uppercase()
    //     )
    // } else {
    //     format!(
    //         "https://cryptopanic.com/api/v1/posts/?auth_token={}&currencies={}",
    //         cp_key, symbol.to_uppercase()
    //     )
    // };
    let url = format!(
        "https://cryptopanic.com/api/v1/posts/?auth_token={}&currencies={}",
        cp_key, symbol.to_uppercase()
    );

    let client = Client::new();
    let response = client.get(url).send().await;

    if let Ok(resp) = response {
        if let Ok(parsed) = resp.json::<CryptoPanicResponse>().await {
            return parsed
                .results
                .into_iter()
                .map(|item| NewsArticle {
                    title: item.title,
                    source: item.domain,
                    published_at: item.published_at,
                    url: item.body.or(Some(item.url)), // fallback to URL
                })
                .collect();
        }
    }

    vec![NewsArticle {
        title: format!("No news found for {}", symbol),
        source: "CryptoPanic".to_string(),
        published_at: chrono::Utc::now().to_rfc3339(),
        url: Some("Try a different symbol or check your API key/tier.".to_string()),
    }]
}
