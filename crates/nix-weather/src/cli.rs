// SPDX-FileCopyrightText: 2023 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
//
// SPDX-License-Identifier: AGPL-3.0-only

use std::{cell::OnceCell, sync::OnceLock};

use clap::{arg, command, crate_authors, value_parser, Arg, ArgAction, Command};

const DEFAULT_CACHE: &str = "cache.nixos.org";

pub fn build_cli() -> Command {
    use std::path::PathBuf;

    command!()
        .author(crate_authors!("\n"))
        .arg(
            arg!(--cache <CACHE> "check a specific cache")
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
}
