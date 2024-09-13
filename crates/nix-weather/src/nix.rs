// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
//
// SPDX-License-Identifier: EUPL-1.2

use serde_json::Value;
use std::{
  path::Path,
  process::{Command, Stdio},
};

/// Get nixosConfiguration derivation path
#[inline]
fn get_config_drv_path(host: &str, config_dir: &str) -> std::io::Result<std::process::Output> {
  Command::new("nix")
    .current_dir(Path::new(config_dir))
    .args([
      "build",
      "--quiet",
      &format!(
        "./#nixosConfigurations.{}.config.system.build.toplevel",
        host
      ),
      "--dry-run",
      "--json",
    ])
    .output()
}

/// Get installable derivation path
#[inline]
fn get_installable_drv_path(installable: &str) -> std::io::Result<std::process::Output> {
  Command::new("nix")
    .args(["build", "--quiet", installable, "--dry-run", "--json"])
    .output()
}

/// Takes a drv_path and gets all it's requisites from the nix store.
#[inline]
fn get_requisites_from_drv_path(drv_path: &str) -> std::io::Result<std::process::Child> {
  Command::new("nix-store")
    .args(["--query", "--requisites", drv_path])
    .stdout(Stdio::piped())
    .spawn()
}

/// Turns requisites into hashes
#[inline]
fn requisites_to_hashes(
  drv_requisites: std::process::Child,
) -> std::io::Result<std::process::Child> {
  let drv_requisites_remove_base = Command::new("cut")
    .args(["-d", "/", "-f4"])
    .stdin(Stdio::from(drv_requisites.stdout.unwrap()))
    .stdout(Stdio::piped())
    .spawn()
    .unwrap();
  Command::new("cut")
    .args(["-d", "-", "-f1"])
    .stdin(Stdio::from(drv_requisites_remove_base.stdout.unwrap()))
    .stdout(Stdio::piped())
    .spawn()
}

pub fn get_requisites(host: &str, config_dir: &str, installable: Option<String>) -> String {
  // If the users specified an installable, we interpret that, instead of trying
  // to guess their config location.
  let drv_path;
  if let Some(installable) = installable {
    drv_path = get_installable_drv_path(&installable).unwrap();
  } else {
    drv_path = get_config_drv_path(host, config_dir).unwrap();
  }

  let drv_path_json: Value =
    serde_json::from_str(&String::from_utf8(drv_path.stdout).unwrap()).unwrap();
  let drv_path = drv_path_json[0]["drvPath"].clone();

  log::debug!("drv_path: {}", &drv_path);

  let drv_requisites = get_requisites_from_drv_path(drv_path.as_str().unwrap()).unwrap();

  let drv_requisite_hashes = requisites_to_hashes(drv_requisites);

  String::from_utf8(
    drv_requisite_hashes
      .unwrap()
      .wait_with_output()
      .unwrap()
      .stdout,
  )
  .unwrap()
}
