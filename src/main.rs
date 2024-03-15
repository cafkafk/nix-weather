// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
//
// SPDX-License-Identifier: AGPL-3.0-only

use std::{io, net::{IpAddr, SocketAddr}};

use dns_lookup::lookup_host;
use futures::{stream, StreamExt, future::join_all};
use rayon::prelude::*;
use itertools::Itertools;

#[allow(unused)]
use log::{debug, error, info, trace, warn};

use crate::nix::get_requisites;

mod cli;

mod nix {

    use serde_json::{Result, Value};
    use std::{
        path::Path,
        process::{Command, Stdio},
        str::Lines,
    };

    pub fn get_requisites(host: &str) -> String {
        let get_drv_path = Command::new("nix")
            .current_dir(Path::new("/home/ces/org/src/git/afk-nixos"))
            .env("NIXPKGS_ALLOW_INSECURE", "1")
            .args([
                "build",
                "--impure",
                "--quiet",
                &format!(
                    "./#nixosConfigurations.{}.config.system.build.toplevel",
                    host
                ),
                "--dry-run",
                "--json",
            ])
            .output()
            .unwrap();

        // println!("{:#?}", &get_drv_path.stdout);
        // let res: Value = serde_json::from_str(&String::from_utf8(output.stdout).unwrap()).unwrap();
        let drv_path_json: Value =
            serde_json::from_str(&String::from_utf8(get_drv_path.stdout).unwrap()).unwrap();
        let drv_path = drv_path_json[0]["drvPath"].clone();

        println!("drv_path: {}", &drv_path);

        let get_drv_requisites = Command::new("nix-store")
            .args(["--query", "--requisites", drv_path.as_str().unwrap()])
            .stdout(Stdio::piped())
            //.output()
            .spawn()
            .unwrap();
        let drv_requisites_remove_base = Command::new("cut")
            .args(["-d", "/", "-f4"])
            .stdin(Stdio::from(get_drv_requisites.stdout.unwrap()))
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        let drv_requisites_to_hash = Command::new("cut")
            .args(["-d", "-", "-f1"])
            .stdin(Stdio::from(drv_requisites_remove_base.stdout.unwrap()))
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        //println!("{:#?}", drv_requisites_to_hash.wait_with_output);

        String::from_utf8(drv_requisites_to_hash.wait_with_output().unwrap().stdout).unwrap()

        // for hash in lines {
        //     println!("{hash}");
        // }
        // println!("{:#?}", get_drv_requisites.stderr);

        // println!(
        //     "{:#?}",
        //     String::from_utf8(get_drv_requisites.stdout).unwrap()
        // );
    }
}

mod net {
    use std::{net::SocketAddr, time::Duration};

    use reqwest::{StatusCode, ClientBuilder, Client};
    use async_recursion::async_recursion;
    use tokio::time::sleep;

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
                nar_exists(client, domain, hash, slide * 2).await
            }
        }

        // match response.status().as_u16() {
        //     200 => 1,
        //     // Retry on ConnectionReset
        //     104 => {
        //         // We're so fast now we get rate limited.
        //         //
        //         // Writng an actual sliding window seems kinda hard,
        //         // so we do this instead.
        //         sleep(Duration::from_millis(slide)).await;
        //         nar_exists(client, domain, hash, slide * 2).await
        //     },
        //     _ => 0
        // }
    }
}

// #[tokio::main(flavor = "multi_thread", worker_threads = 100)]
// #[tokio::main(flavor = "multi_thread", worker_threads = 500)]
#[tokio::main(flavor = "multi_thread")]
async fn main() -> io::Result<()> {
    pretty_env_logger::init();
    let matches = cli::build_cli().get_matches();

    let domain = "cache.nixos.org";
    let ips: Vec<std::net::IpAddr> = lookup_host(domain).unwrap();

    debug!("{:#?}", &ips);

    let domain_addr = SocketAddr::new(ips[0], 443);

    let client = reqwest::Client::builder().resolve(domain, domain_addr).build().unwrap();

    let binding = get_requisites("DBCAC");
    let connection_buffer = binding.lines().map(|line| line.to_owned()).collect::<Vec<_>>();

    // FIXME make constant
    let slide = 100;

    // FIXME we take ten just for testing
    let tasks = connection_buffer
        .into_iter()
        //.take(1000)
        .map(|hash| {
            let client = client.clone();
            tokio::spawn(async move {
                info!("connecting to {domain} {domain_addr:#?} for {hash}");
                net::nar_exists(client, domain, &hash, slide).await
            })
        })
        .collect_vec();

    let sum: usize = join_all(tasks).await.into_iter().map(|result| result.unwrap()).sum();

    println!("sum {:#?}", sum);
        //map(|hash| async {net::nar_exists(hostname, SocketAddr::new(ip.clone(), 443), hash).await}).collect::<Vec<_>>();

    // let response = reqwest::Client::builder()
    //     .resolve(
    //         url,
    //         SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 443),
    //     )
    //     .build()
    //     .unwrap()
    //     .get(url)
    //     .send()
    //     .await?;

    Ok(())
}
