use std::env;
use std::fs;
use std::ops::Add;
use std::path::Path;
use std::process::exit;

fn main() {
    let mut binary = std::env::var("OUT_DIR").unwrap();
    binary.push_str("/../../../");
    let binary = std::path::Path::new(&binary);
    if !binary.exists() {
        eprintln!("Path to the binaries does not exist! {}", binary.display());
        exit(1);
    }

    let abs = match binary.canonicalize() {
        Err(_) => {
            eprintln!("Cannot resolve absolute path for{}", binary.display());
            exit(1);
        },
        Ok(path) => {
            path
        }
    };

    println!("cargo:rustc-link-arg=-Wl,-rpath={}", abs.to_str().unwrap());

    let lib_pathes = match std::env::var("LD_LIBRARY_PATH") {
        Err(_) => {
            String::new()
        },
        Ok(pathes) => {
            pathes
        }
    };

    if !lib_pathes.is_empty() {
        for path in lib_pathes.split(':') {
            if path.contains(".rustup") {
                let abs = Path::new(path);
                let abs = abs.canonicalize().unwrap();
                println!("cargo:rustc-link-arg=-Wl,-rpath={}", abs.to_str().unwrap());
            }
        }
    }
}
