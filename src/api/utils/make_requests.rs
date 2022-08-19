use reqwest::{Client};


pub async fn make_requests() {
    run_post().await.expect("TODO: panic message");
}

async fn run_post() -> Result<bool, &'static str> {
    let client = Client::new();
    let res = client.post("http://httpbin.org/post")
        .body("the exact body that is sent")
        .send()
        .await;
    Ok(true)
}
