// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
// SPDX-FileContributor: Maximilian Marx
//
// SPDX-License-Identifier: EUPL-1.2

use clap::{arg, command, crate_authors, value_parser, ArgAction, ArgGroup, Command};

const DEFAULT_CACHE: &str = "cache.nixos.org";

pub fn build_cli() -> Command {
  command!()
    .author(crate_authors!("\n"))
    // TODO: parse multiple installables, like e.g. build does?
    .arg(arg!([installable] "A nix installable").required(false))
    .arg(
      arg!(--cache <CACHE> "Check a specific cache")
        .required(false)
        .default_value(DEFAULT_CACHE),
    )
    .arg(
      arg!(-n --name <HOST> "Hostname of machine.")
        .required(false)
        .value_parser(value_parser!(String)),
    )
    .arg(
      arg!(-c --config <FILE> "Path to NixOS config.")
        .required(false)
        .value_parser(value_parser!(String)),
    )
    .arg(
      arg!(--timestamp "Add timestamp to log output.")
        .action(ArgAction::SetTrue)
        .required(false),
    )
    .arg(arg!(-v --verbose ... "Verbosity level."))
    .arg(
      arg!(printBuildLogs: -L "Verbosity level.")
        .long("print-build-logs")
        .conflicts_with("verbose"),
    )
    .arg(arg!(-'4' --"only-ipv4" "Use IPv4 addresses only.").action(ArgAction::SetTrue))
    .arg(arg!(-'6' --"only-ipv6" "Use IPv6 addresses only.").action(ArgAction::SetTrue))
    .group(
      ArgGroup::new("address_family")
        .args(["only-ipv4", "only-ipv6"])
        .required(false),
    )
}
