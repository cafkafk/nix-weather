// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
// SPDX-FileContributor: Maximilian Marx
//
// SPDX-License-Identifier: EUPL-1.2

use std::sync::Arc;
use std::time::Instant;
use std::{env, io, net::SocketAddr};

use futures::future::join_all;
use gethostname::gethostname;
use itertools::Itertools;
use net::AddressFamilyFilter;

use crate::nix::get_requisites;

mod cli;
mod net;
mod nix;

/// The initial time to wait on http 104, in milliseconds
const SLIDE: u64 = 100;

// Open files limit to try to set
const NOFILES_LIMIT: u64 = 16384;

const DEFAULT_CACHE: &str = "cache.nixos.org";
const DEFAULT_CONFIG_DIR: &str = "/etc/nixos";

#[tokio::main(flavor = "multi_thread")]
async fn main() -> io::Result<()> {
  let initial_time = Instant::now();

  let host_name: String;
  let cache_url: String;
  let config_dir: String;

  let matches = cli::build_cli().get_matches();

  // If the users inputs more -v flags than we have log levels, send them a
  // message informing them.
  let mut very_bose = false;

  // The Normal verbose flag, allowing multiple levels. Conflicts with
  // printBuildLogs.
  match matches
    .get_one::<u8>("verbose")
    .expect("Counts aren't defaulted")
  {
    0 => env::set_var("RUST_LOG", "error"),
    1 => env::set_var("RUST_LOG", "warn"),
    2 => env::set_var("RUST_LOG", "info"),
    3 => env::set_var("RUST_LOG", "debug"),
    4 => env::set_var("RUST_LOG", "trace"),
    _ => {
      very_bose = true;
      env::set_var("RUST_LOG", "trace")
    }
  }

  // The -L flag, to give a more nix3 feel
  if matches.get_flag("printBuildLogs") {
    env::set_var("RUST_LOG", "trace")
  }

  if matches.get_flag("timestamp") {
    pretty_env_logger::formatted_timed_builder()
      .parse_env("RUST_LOG")
      .init();
  } else {
    pretty_env_logger::formatted_builder()
      .parse_env("RUST_LOG")
      .init();
  }

  if very_bose {
    log::trace!("More than four -v flags don't increase log level.");
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

  if let Some(config) = matches.get_one::<String>("config") {
    config_dir = config.to_owned();
  } else {
    config_dir = DEFAULT_CONFIG_DIR.to_string();
  }

  let address_family_filter = if matches.get_flag("only-ipv4") {
    AddressFamilyFilter::OnlyIPv4
  } else if matches.get_flag("only-ipv6") {
    AddressFamilyFilter::OnlyIPv6
  } else {
    Default::default()
  };

  let domain = cache_url.to_owned();
  let ips: Vec<std::net::IpAddr> = address_family_filter
    .lookup_host(&domain)
    .unwrap()
    .collect();

  log::debug!("{:#?}", &ips);

  // try to increase NOFILES runtime limit
  if rlimit::increase_nofile_limit(NOFILES_LIMIT).is_err() {
    log::warn!(
      "Failed to increase NOFILES limit, still at {:#?}",
      rlimit::Resource::NOFILE.get().unwrap_or_default()
    );
  }

  let domain_addr = SocketAddr::new(ips[0], 443);

  let client = reqwest::Client::builder()
    .dns_resolver(Arc::new(address_family_filter))
    .resolve(&domain, domain_addr)
    .build()
    .unwrap();

  let binding = get_requisites(
    &host_name,
    &config_dir,
    matches.get_one::<String>("installable").cloned(),
  );

  let get_requisites_duration = initial_time.elapsed().as_secs();

  println!(
    "Found Nix Requisites in {} seconds",
    get_requisites_duration
  );

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
        log::trace!("connecting to {domain} {domain_addr:#?} for {hash}");
        net::nar_exists(client, &domain, &hash, SLIDE).await
      })
    })
    .collect_vec();

  let sum: usize = join_all(tasks)
    .await
    .into_iter()
    .map(|result| result.unwrap())
    .sum();

  println!(
    "Checked {count} packages in {} seconds",
    network_time.elapsed().as_secs()
  );
  println!(
    "Found {:#?}/{} ({:.2}%) in cache",
    sum,
    count,
    (sum as f64 / count as f64) * 100.
  );

  Ok(())
}
