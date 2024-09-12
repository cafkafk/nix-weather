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
fn get_installable_drv_path(
  host: &str,
  config_dir: &str,
  installable: &str,
) -> std::io::Result<std::process::Output> {
  Command::new("nix")
    .current_dir(Path::new(config_dir))
    .args(["build", "--quiet", installable, "--dry-run", "--json"])
    .output()
}

pub fn get_requisites(host: &str, config_dir: &str, installable: Option<String>) -> String {
  let mut drv_path;
  if let Some(installable) = installable {
    drv_path = get_installable_drv_path(host, config_dir, &installable).unwrap();
  } else {
    drv_path = get_config_drv_path(host, config_dir).unwrap();
  }

  let drv_path_json: Value =
    serde_json::from_str(&String::from_utf8(drv_path.stdout).unwrap()).unwrap();
  let drv_path = drv_path_json[0]["drvPath"].clone();

  log::debug!("drv_path: {}", &drv_path);

  let get_drv_requisites = Command::new("nix-store")
    .args(["--query", "--requisites", drv_path.as_str().unwrap()])
    .stdout(Stdio::piped())
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

  String::from_utf8(drv_requisites_to_hash.wait_with_output().unwrap().stdout).unwrap()
}
