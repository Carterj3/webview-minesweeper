use std::process::Command;
use std::path::PathBuf;

use std::env;

fn main()
{
    build_elm();
}

fn build_elm()
{
    let elm_path = "src/Main.elm";
    
    let elm_output: PathBuf = [&env::var("OUT_DIR").unwrap(), "elm.js"].iter().collect();
    let elm_output = elm_output.to_str().unwrap();

    let elm_input: PathBuf =[&env::var("CARGO_MANIFEST_DIR").unwrap(), "../minesweeper-ui", elm_path].iter().collect();
    let elm_input = elm_input.to_str().unwrap();

    let ui_dir: PathBuf = [&env::var("CARGO_MANIFEST_DIR").unwrap(), "../minesweeper-ui"].iter().collect();
    let ui_dir = ui_dir.to_str().unwrap();

    if Command::new("elm")
        .current_dir(ui_dir)
        .args(&["make", elm_path, &format!("--output={}", elm_output)])
        .status()
        .unwrap()
        .success()
    {
        println!("cargo:rerun-if-changed=\"{}\"", elm_input);
    } else {
        panic!("Failed to create elm");
    }
}