use axum::{
    extract::Query, http::StatusCode, response::{Html, IntoResponse, Redirect}, routing::get, Json, Router
};
use serde::Deserialize;
use std::net::SocketAddr;
use tower_http::services::ServeDir;
mod news;
use news::fetch_news_for_symbol;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    // Router with routes
    let app = Router::new()
        .route("/", get(redirect_to_news))
        .route("/news", get(root))
        .route("/api/news", get(fetch_news))
        .fallback_service(ServeDir::new("public"));

    // Run the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running at http://{addr}");

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app,
    )
    .await
    .unwrap();
}

async fn redirect_to_news() -> impl IntoResponse {
    Redirect::permanent("/news")
}

// Serve the main HTML file
async fn root() -> impl IntoResponse {
    match tokio::fs::read_to_string("public/index.html").await {
        Ok(content) => Html(content).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Could not load index.html").into_response(),
    }
}

#[derive(Deserialize)]
struct NewsQuery {
    symbol: String,  // Symbol to fetch news for, e.g., "BTC"
}
async fn fetch_news(Query(query): Query<NewsQuery>) -> impl IntoResponse {
    let symbol = &query.symbol;

    // Fetch news for the cryptocurrency symbol using the function from news.rs
    let news = fetch_news_for_symbol(symbol).await;

    Json(news).into_response()  // Return the news as JSON
}



/*
axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app,
    )
    .await
    .unwrap();
*/