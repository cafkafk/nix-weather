// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
//
// SPDX-License-Identifier: EUPL-1.2

use std::time::Duration;

use reqwest::Client;
use tokio::time::sleep;

use log;

const MAX_SLIDE: u64 = 1000;

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
            log::trace!("rate limited! {slide}");
            sleep(Duration::from_millis(slide)).await;
            Box::pin(nar_exists(
                client,
                domain,
                hash,
                std::cmp::min(slide * 2, MAX_SLIDE),
            ))
            .await
        }
    }
}
