/*
 * author: Aleksei Kozadaev (2022)
 */

extern crate cc;

fn main() {
    // important to build before the printlns.
    cc::Build::new().file("./src/alsa.c").compile("alsa");

    println!("cargo:rerun-if-changed=./src/alsa.c");
    println!("cargo:rerun-if-changed=./build.rs");
    println!("cargo:rustc-link-search=./src");
    println!("cargo:rustc-link-lib=asound");
}
