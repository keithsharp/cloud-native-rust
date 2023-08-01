use axum::{
    http::{StatusCode, Uri},
    routing::get,
    Router,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Listening on http://0.0.0.0:3000");

    let app = Router::new().route("/", get(get_root)).fallback(fallback);
    axum::Server::bind(&"0.0.0.0:3000".parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn get_root() -> &'static str {
    println!("Got a request");
    "Hello from a containerised Axum server!"
}

async fn fallback(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("Page not found: {}", uri))
}
