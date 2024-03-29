use serde_json::Value;
use std::{
    path::Path,
    process::{Command, Stdio},
};

pub fn get_requisites(host: &str, config_dir: &str) -> String {
    let get_drv_path = Command::new("nix")
        .current_dir(Path::new(config_dir))
        .env("NIXPKGS_ALLOW_INSECURE", "1") // FIXME Idk but fix it
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
            "--option",
            "eval-cache",
            "true",
        ])
        .output()
        .unwrap();

    let drv_path_json: Value =
        serde_json::from_str(&String::from_utf8(get_drv_path.stdout).unwrap()).unwrap();
    let drv_path = drv_path_json[0]["drvPath"].clone();

    println!("drv_path: {}", &drv_path);

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
