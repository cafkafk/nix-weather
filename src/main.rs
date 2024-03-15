// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
//
// SPDX-License-Identifier: AGPL-3.0-only

use std::{io, net::{IpAddr, SocketAddr}};

use dns_lookup::lookup_host;
use futures::{stream, StreamExt};
use rayon::prelude::*;

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
    use std::net::SocketAddr;

    use reqwest::StatusCode;

    pub async fn nar_exists(domain: &str, domain_addr: SocketAddr, hash: &str) -> usize {
        let response = reqwest::Client::builder()
            .resolve(domain, domain_addr)
            .build()
            .unwrap()
            .get(format!("https://{domain}/{hash}.narinfo"))
            .send()
            .await
            .unwrap();
        if response.status() == StatusCode::from_u16(200).unwrap() {
            1
        }
        else {
            0
        }
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    pretty_env_logger::init();
    let matches = cli::build_cli().get_matches();

    let hostname = "cache.nixos.org";
    let ips: Vec<std::net::IpAddr> = lookup_host(hostname).unwrap();

    debug!("{:#?}", &ips);

    let ip = ips[0];

    let binding = get_requisites("DBCAC");
    //let connection_buffer = stream::iter(binding.lines().map(|line| line.to_owned()).collect::<Vec<_>>()); //.buffer_unordered(20);
    let connection_buffer = stream::iter(binding.lines().map(|line| line.to_owned()).collect::<Vec<_>>()); //.buffer_unordered(20);
    // FIXME we take ten just for testing
    let stuff = connection_buffer.take(1000).then(|hash| async move {
        info!("connecting to {hostname} {ip:#?} for {hash}");
        net::nar_exists(hostname, SocketAddr::new(ip.clone(), 443), &hash).await
    }).collect::<Vec<usize>>();
        //map(|hash| async {net::nar_exists(hostname, SocketAddr::new(ip.clone(), 443), hash).await}).collect::<Vec<_>>();
    println!("sum {:#?}", stuff.await.par_iter().sum::<usize>());

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
