extern crate clap;

use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::io::Write;

use clap::{Arg, App};

fn main() {
    let matches = App::new("Project Initializer")
        .version("0.1.0")
        .author("Alex V.")
        .about("Initialize your projects the smart way.")
        .arg(Arg::with_name("name")
            .value_name("NAME")
            .help("The directory to initialize project.")
            .required(true)
            .index(1))
        .arg(Arg::with_name("license")
            .short("l")
            .long("license")
            .value_name("LICENSE")
            .help("Choose a license.")
            .takes_value(true)
            .possible_values(&["mit"])
            .default_value("mit"))
        .get_matches();

    create_git_repo(matches.value_of("name").unwrap());
    create_gitignore();
    create_venv();
    create_license(matches.value_of("license").unwrap());
    create_readme(matches.value_of("name").unwrap());
}

fn create_venv() {
    Command::new("python3")
        .args(&["-m", "venv", "venv"])
        .output()
        .expect("failed to create venv");
}

fn create_gitignore() {
    let contents = include_str!("resources/gitignores/python");
    fs::write(".gitignore", contents).expect("failed to create .gitignore");
}

fn create_readme(name: &str) {
    fs::write("README.md", format!("# {}", name)).expect("failed to create readme");
}

fn create_license(license_name: &str) {

    let license_text = match license_name {
        "mit" => include_str!("resources/licenses/mit"),
        _ => "hi",
    };

    fs::write("LICENSE", license_text).expect("failed to create license");
}

fn create_git_repo(name: &str) {
    match fs::create_dir(name) {
        Err(why) => println!("failed to create directory: {:?}", why.kind()),
        Ok(_) => {},
    }

    let dpath = Path::new(name);

    assert!(env::set_current_dir(&dpath).is_ok());

    Command::new("git")
        .arg("init")
        .output()
        .expect("failed to initialize git repo");

    let mut file = fs::OpenOptions::new().append(true).open(".git/info/exclude").unwrap();

    write!(file, "{}", include_str!("resources/gitexclude")).expect("failed to update gitexclude");
}
