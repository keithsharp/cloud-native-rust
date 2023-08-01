#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Fetching page from https://www.rust-lang.org");

    let response = reqwest::Client::new()
        .post("https://www.rust-lang.org")
        .send()
        .await?;

    println!("Got response: {:?}", response);

    Ok(())
}
