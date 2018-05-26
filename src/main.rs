extern crate clap;
extern crate serde_json;
extern crate chrono;

use std::io::ErrorKind;
use serde_json::{Value};
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::io::{Write};

use chrono::Datelike;
use clap::{Arg, App};

fn main() {

    let matches = App::new("Project Initializer")
        .version("0.1.0")
        .author("jalexvig")
        .about("Initialize your projects the smart way.")
        .arg(Arg::with_name("name")
            .value_name("NAME")
            .help("The directory to initialize project.")
            .required(true)
            .index(1))
        .arg(Arg::with_name("preferences")
            .short("p")
            .long("preferences")
            .value_name("PREFERENCES")
            .help("Filepath for json preferences.")
            .takes_value(true)
            .default_value("default"))
        .get_matches();

    let prefs = get_pref(matches.value_of("preferences").unwrap());

    let name_proj = matches.value_of("name").unwrap();

    create_git_repo(name_proj);
    create_gitignore();

    create_readme(name_proj);

    create_license(&prefs);

    execute_commands(&prefs);
}

fn execute_commands(prefs: &Value) {

    if let Value::Array(ref commands) = prefs["commands"] {
        for c in commands {
            if let Some(c) = c.as_str() {
                execute_command(c);
            }
        }
    }
}

fn execute_command(command: &str) {

    let command: Vec<&str> = command.split(' ').collect();

    if let Some((first, tail)) = command.split_first() {
        Command::new(first)
            .args(tail)
            .output()
            .expect("failed to execute command");
    }
}

fn get_pref(pref_name: &str) -> Value {

    let fname = format!("{}{}", pref_name, ".json");

    let dpath = match std::env::home_dir() {
        Some(dpath_home) => dpath_home.join(".proji"),
        None => panic!("could not find home directory")
    };

    match fs::create_dir(&dpath) {
        Err(e) => {
            match e.kind() {
                ErrorKind::AlreadyExists => {},
                _ => {
                    panic!("error creating profile directory")
                }
            }
        },
        Ok(_) => {
            let contents = include_str!("resources/profiles/default.json");
            let fpath = dpath.join("default.json");
            fs::write(fpath, contents).expect("failed to create default profile");
        }
    }

    let fpath_prefs = dpath.join(fname);

    let file = fs::File::open(fpath_prefs).expect(&format!("could not open profile: {}", pref_name));

    serde_json::from_reader(file).expect("error reading json file")
}

fn create_gitignore() {
    fs::write(".gitignore", "").expect("failed to create .gitignore");
}

fn create_readme(name: &str) {
    fs::write("README.md", format!("# {}", name)).expect("failed to create readme");
}

fn create_license(prefs: &Value) {

    let now = chrono::Local::now();
    let year = now.year().to_string();

    let name = match prefs["name"].as_str() {
        Some(n) => n,
        None => "[NAME]"
    };

    let license_text = match prefs["license"].as_str() {
        Some(license_type) => {
            match license_type {
                "mit" => Some(format!(include_str!("resources/licenses/mit"), year, name)),
                _ => {
                    println!("no license {} available. consider contributing it.", license_type);
                    None
                },
            }
        },
        None => None,
    };

    if let Some(t) = license_text {
        fs::write("LICENSE", t).expect("failed to create license");
    }
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
