use std::env;
use std::fs;
use std::fs::File;
use std::fs::Permissions;
use std::io::Write;
use std::ops::Add;
use std::os::unix::prelude::PermissionsExt;
use std::path::Path;
use std::path::PathBuf;
use std::process::exit;

fn main() {
    let lib_dir = std::env::var("CARGO").unwrap();
    let mut lib_dir = Path::new(&lib_dir).parent().unwrap();
    let mut lib_dir = lib_dir.join("../lib/");
    let lib_dir = lib_dir.canonicalize().expect("Cannot find runtime path");

    let mut path_to_runtime = PathBuf::new();

    for entry in fs::read_dir(lib_dir).unwrap() {
        match entry {
            Err(_) => continue,
            Ok(entry) => {
                let path = entry.path();
                if path.is_file()
                    && path.file_name().unwrap().to_str().unwrap().contains("libstd") {
                    path_to_runtime = path.to_path_buf();
                }
            }
        }
    }

    if !path_to_runtime.exists() {
        eprintln!("Could not file runtime");
        exit(1);
    }

    let build_dir = std::env::var("OUT_DIR").unwrap() + "/../../../";
    let build_dir = Path::new(&build_dir);

    {
        let launcher_script = build_dir.join("launch.sh");
        if launcher_script.exists() {
            fs::remove_file(&launcher_script);
        }
        let mut launcher_script = File::create(launcher_script)
            .expect("Cannot create launcher script");
        let mut mode = Permissions::from_mode(0o555);
        mode.set_readonly(false);
        launcher_script.set_permissions(mode).expect("Cannot make script an executable");
        launcher_script.write_all(b"#!/bin/bash \n\n LD_LIBRARY_PATH=\"$(dirname $0)\" ./$(dirname $0)/launcher")
            .expect("Cannot write to script file");
    }

    let mut runtime_present = false;
    for entry in fs::read_dir(build_dir).unwrap() {
        match entry {
            Err(_) => continue,
            Ok(entry) => {
                let path = entry.path();
                if path.is_file() {
                    let target_runtime = path.file_name().unwrap().to_str().unwrap();
                    let needs_runtime = path_to_runtime.to_str().unwrap();
                    if target_runtime.contains("libstd") && target_runtime != needs_runtime {
                        fs::remove_file(path);
                    }
                    else if target_runtime == needs_runtime {
                        runtime_present = true;
                    }
                }
            }
        }
    }

    if !runtime_present {
        let filename = (&path_to_runtime).file_name().unwrap().to_str().unwrap();
        fs::copy(&path_to_runtime, build_dir.join(Path::new(&filename)))
            .expect("Failed to copy the runtime");
    }
}
