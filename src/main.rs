// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
//
// SPDX-License-Identifier: AGPL-3.0-only

use std::{io, net::SocketAddr, env, sync::OnceLock};

use dns_lookup::lookup_host;
use futures::future::join_all;
use itertools::Itertools;
use gethostname::gethostname;

#[allow(unused)]
use log::{debug, error, info, trace, warn};

use crate::nix::get_requisites;

mod cli;
mod net;
mod nix;

/// The initial time to wait on http 104, in milliseconds
const SLIDE: u64 = 100;

const DEFAULT_CACHE: &str = "cache.nixos.org";

const HOST_NAME: OnceLock<String> = OnceLock::new();
const CACHE_URL: OnceLock<String> = OnceLock::new();

#[tokio::main(flavor = "multi_thread")]
async fn main() -> io::Result<()> {
    pretty_env_logger::init();

    let matches = cli::build_cli().get_matches();

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
        HOST_NAME.get_or_init(|| name.to_owned());
    }
    else {
        HOST_NAME.get_or_init(|| gethostname().into_string().unwrap());
    }

    if let Some(cache) = matches.get_one::<String>("cache") {
        trace!("Got cache argument: {cache}");
        CACHE_URL.get_or_init(|| cache.to_owned());
    }
    else {
        trace!("No cache argument, using default: {}", DEFAULT_CACHE.to_string());
        CACHE_URL.get_or_init(|| DEFAULT_CACHE.to_string());
    }

    let domain = CACHE_URL.get().unwrap().to_owned();
    let ips: Vec<std::net::IpAddr> = lookup_host(&domain).unwrap();

    debug!("{:#?}", &ips);

    let domain_addr = SocketAddr::new(ips[0], 443);

    let client = reqwest::Client::builder()
        .resolve(&domain, domain_addr)
        .build()
        .unwrap();

    let binding = get_requisites(HOST_NAME.get().unwrap());

    let tasks = binding
        .lines()
        .map(|line| line.to_owned())
        .collect::<Vec<_>>()
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

    println!("sum {:#?}", sum);

    Ok(())
}
