/*
 * author: Aleksei Kozadaev (2022)
 */

extern crate cc;

use std::process::Command;

fn main() {
    // important to build before the printlns.
    cc::Build::new().file("./src/alsa.c").compile("alsa");

    let wi_cmd = Command::new("sh")
        .arg("-c")
        .arg("find /sys/class/net/*/wireless | head -1 | cut -d / -f 5")
        .output()
        .expect("wifi command failed");
    let wi = std::str::from_utf8(&wi_cmd.stdout).unwrap().trim();

    let link_cmd = Command::new("find")
        .args([
            format!("/sys/class/net/{}/", wi).as_str(),
            "-name",
            "operstate",
            "-print0",
            "-quit",
        ])
        .output()
        .expect("find failed");

    let link_path = std::str::from_utf8(&link_cmd.stdout)
        .unwrap()
        .trim()
        .trim_matches(char::from(0));

    println!("cargo:rerun-if-changed=./src/alsa.c");
    println!("cargo:rerun-if-changed=./build.rs");
    println!("cargo:rustc-link-search=./src");
    println!("cargo:rustc-link-lib=asound");
    println!("cargo:rustc-env=LINK_PATH={}", link_path);
}
