// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
// SPDX-FileContributor: Maximilian Marx
//
// SPDX-License-Identifier: EUPL-1.2

use std::{
  io,
  net::{IpAddr, SocketAddr},
  time::Duration,
};

use reqwest::{dns::Resolve, Client};
use tokio::time::sleep;

const MAX_SLIDE: u64 = 1000;

#[derive(Debug, Copy, Clone, Default)]
pub enum AddressFamilyFilter {
  #[default]
  Both,
  OnlyIPv4,
  OnlyIPv6,
}

impl AddressFamilyFilter {
  pub fn lookup_host(self, host: &str) -> io::Result<impl Iterator<Item = IpAddr>> {
    let addresses = dns_lookup::lookup_host(host)?;
    Ok(self.filter_addresses(addresses))
  }

  fn filter_addresses<T>(self, addresses: T) -> impl Iterator<Item = IpAddr>
  where
    T: IntoIterator<Item = IpAddr>,
  {
    addresses.into_iter().filter(move |address| match self {
      Self::Both => true,
      Self::OnlyIPv4 => matches!(address, IpAddr::V4(_)),
      Self::OnlyIPv6 => matches!(address, IpAddr::V6(_)),
    })
  }
}

impl Resolve for AddressFamilyFilter {
  fn resolve(&self, name: reqwest::dns::Name) -> reqwest::dns::Resolving {
    let filter = *self;
    Box::pin(async move {
      let addresses = filter.lookup_host(name.as_str())?;
      let socket_addresses: Box<dyn Iterator<Item = SocketAddr> + Send> =
        Box::new(addresses.map(|ip| SocketAddr::new(ip, 0)));

      Ok(socket_addresses)
    })
  }
}

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
