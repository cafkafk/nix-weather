// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
//
// SPDX-License-Identifier: AGPL-3.0-only

use std::{env, io, net::SocketAddr};
use std::time::{Duration, Instant};

use dns_lookup::lookup_host;
use futures::future::join_all;
use gethostname::gethostname;
use itertools::Itertools;

#[allow(unused)]
use log::{debug, error, info, trace, warn};

use crate::nix::get_requisites;

mod cli;
mod net;
mod nix;

/// The initial time to wait on http 104, in milliseconds
const SLIDE: u64 = 100;

const DEFAULT_CACHE: &str = "cache.nixos.org";

#[tokio::main(flavor = "multi_thread")]
async fn main() -> io::Result<()> {
    let initial_time = Instant::now();

    let host_name: String;
    let cache_url: String;

    pretty_env_logger::init();

    let matches = cli::build_cli().get_matches();

    // TODO
    match matches
        .get_one::<u8>("verbose")
        .expect("Count's are defaulted")
    {
        0 => env::set_var("RUST_LOG", "error"),
        1 => env::set_var("RUST_LOG", "warn"),
        2 => env::set_var("RUST_LOG", "info"),
        3 => env::set_var("RUST_LOG", "debug"),
        4 => env::set_var("RUST_LOG", "trace"),
        _ => {
            trace!("More than four -v flags don't increase log level.");
            env::set_var("RUST_LOG", "trace")
        }
    }

    if let Some(name) = matches.get_one::<String>("name") {
        host_name = name.to_owned();
    } else {
        host_name = gethostname().into_string().unwrap();
    }

    if let Some(cache) = matches.get_one::<String>("cache") {
        cache_url = cache.to_owned();
    } else {
        cache_url = DEFAULT_CACHE.to_string();
    }

    let domain = cache_url.to_owned();
    let ips: Vec<std::net::IpAddr> = lookup_host(&domain).unwrap();

    debug!("{:#?}", &ips);

    let domain_addr = SocketAddr::new(ips[0], 443);

    let client = reqwest::Client::builder()
        .resolve(&domain, domain_addr)
        .build()
        .unwrap();

    let binding = get_requisites(&host_name);

    let get_requisites_duration = initial_time.elapsed().as_secs();

    let network_time = Instant::now();

    let lines = binding
        .lines()
        .map(|line| line.to_owned())
        .collect::<Vec<_>>();

    let count = lines.len();

    let tasks = lines
        .into_iter()
        .map(|hash| {
            let client = client.clone();
            let domain = domain.clone();
            tokio::spawn(async move {
                info!("connecting to {domain} {domain_addr:#?} for {hash}");
                net::nar_exists(client, &domain, &hash, SLIDE).await
            })
        })
        .collect_vec();

    let sum: usize = join_all(tasks)
        .await
        .into_iter()
        .map(|result| result.unwrap())
        .sum();

    println!("Found Nix Requisites in {} seconds", get_requisites_duration);
    println!("Checked {count} packages in {} seconds", network_time.elapsed().as_secs());
    println!("");
    println!("Found {:#?}/{} ({:.2}%) in cache", sum, count, (sum as f64 /count as f64) * 100.);

    Ok(())
}
