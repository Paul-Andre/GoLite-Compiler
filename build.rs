// build.rs

use std::process::Command;
use std::env;
use std::path::Path;
use std::process::Stdio;

fn main() {


    let out_dir = env::var("OUT_DIR").unwrap();

    eprintln!("The output directory is {}", out_dir);

    // below detail how to improve the portability of these commands.
    //Command::new("cd").args(&["./src"]).status().unwrap();
    //Command::new("cd").args(&["src"]).status().unwrap();

    let root = Path::new("src");
    assert!(env::set_current_dir(&root).is_ok());
    eprintln!("Successfully changed working directory to {}!", root.display());

    Command::new("make")
        .stdout(Stdio::null())
        .status().unwrap();

    //println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-search=native=src");
    println!("cargo:rustc-link-lib=static=bisonparser");
    println!("cargo:rustc-link-lib=fl");
}
