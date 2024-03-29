use std::time::Duration;

use async_recursion::async_recursion;
use reqwest::Client;
use tokio::time::sleep;

const MAX_SLIDE: u64 = 1000;

#[async_recursion]
pub async fn nar_exists(client: Client, domain: &str, hash: &str, slide: u64) -> usize {
    let response = client
        .head(format!("https://{domain}/{hash}.narinfo"))
        .send()
        .await;

    match response {
        Ok(response) if response.status().as_u16() == 200 => 1,
        Ok(response) if response.status().as_u16() == 404 => 0,
        _ => {
            // We're so fast now we get rate limited.
            //
            // Writng an actual sliding window seems kinda hard,
            // so we do this instead.
            sleep(Duration::from_millis(slide)).await;
            nar_exists(client, domain, hash, std::cmp::min(slide * 2, MAX_SLIDE)).await
        }
    }
}
