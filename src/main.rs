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
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Vacant, Occupied};

const ALL_PROFILES: &[(&str, &[u8])] = &include!(concat!(env!("OUT_DIR"), "/all_profiles.rs"));

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
        .arg(Arg::with_name("profile")
            .short("p")
            .long("profile")
            .value_name("PROFILE")
            .help("Filepath for json profile.")
            .takes_value(true)
            .default_value("default"))
        .get_matches();

    let prof = get_prof(matches.value_of("profile").unwrap());

    let name_proj = matches.value_of("name").unwrap();

    create_git_repo(name_proj);

    create_gitignore();

    create_readme(name_proj);

    create_license(&prof);

    execute_commands(&prof);
}

fn execute_commands(prof: &HashMap<String, Value>) {

    if let Some(Value::Array(ref commands)) = prof.get("commands") {

        let mut commands_str: Vec<&str> = vec![];

        for c in commands {
            if let Some(c) = c.as_str() {
                commands_str.push(c);
            }
        }

        let command = commands_str.join(" && ");

        execute_command(&command);
    }
}

fn execute_command(command: &str) {

    Command::new("sh")
        .args(&["-c", command])
        .output()
        .expect("failed to execute command");
}

fn get_prof(prof_name: &str) -> HashMap<String, Value> {

    let fname = format!("{}{}", prof_name, ".json");

    let dpath = match std::env::home_dir() {
        Some(dpath_home) => dpath_home.join(".proji"),
        None => panic!("could not find home directory")
    };

    match fs::create_dir(&dpath) {
        Err(e) => {
            match e.kind() {
                ErrorKind::AlreadyExists => {},
                _ => {
                    panic!("error creating .proji directory in HOME")
                }
            }
        },
        Ok(_) => {

            for (prof_name, prof_str) in ALL_PROFILES {
                let fpath = dpath.join(prof_name);
                fs::write(fpath, prof_str).expect(&format!("failed to create profile: {}", prof_name));
            }
        }
    }

    let mut prof_names = c3_linearize(fname, &dpath);
    prof_names.reverse();

    load_profs(prof_names, &dpath)
}

fn load_profs(fnames: Vec<String>, dpath: &std::path::PathBuf) -> HashMap<String, Value> {

    let mut res = HashMap::new();

    for fname in fnames {

        let fpath_prof = dpath.join(&fname);

        let file = fs::File::open(fpath_prof).expect(&format!("could not open profile: {}", &fname));

        let json: Value = serde_json::from_reader(file).expect("error reading json file");

//        using entry pattern: https://stackoverflow.com/questions/30851464
        if let Value::Object(m) = json {
            for (k, val) in m {

                match res.entry(k.to_string()) {
                    Vacant(entry) => { entry.insert(val); },
                    Occupied(mut entry) => {
                        match entry.get_mut() {
                            Value::Array(ref mut v_res) => {
                                if let Value::Array(v_json) = val {
                                    v_res.extend(v_json);
                                } else {
                                    println!("mismatched datatypes for key {}", &k);
                                }
                            },
                            x => *x = val
                        }
                    },
                }
            }
        } else {
            println!("can't process profile {}", &fname)
        }
    }

    res
}

fn c3_linearize(fname: String, dpath: &std::path::PathBuf) -> Vec<String> {

    let fpath = dpath.join(&fname);

    let file = fs::File::open(fpath).expect(&format!("could not open profile: {}", fname));

    let json : Value = serde_json::from_reader(file).expect("error reading json file");

    let parents: Vec<String> = match &json["inherits"] {
        &Value::Null => {
            return vec![fname.to_string()];
        },
        Value::Array(ref v) => {
            let mut p : Vec<String>= vec![];
            for elem in v {
                if let Value::String(s) = elem {
                    p.push(s.to_string());
                } else {
                    println!("couldnt parse profile {}", fname);
                }
            }
            p
        },
        Value::String(v) => vec![v.to_string()],
        _ => panic!("don't understand inherits attribute in profile {}", fname)
    };

    if parents.is_empty() {
        return vec![fname.to_string()];
    }

    let mut parent_linearizations: Vec<Vec<String>> = vec![];

    for mut parent in parents {
        if !parent.ends_with(".json") {
            parent.push_str(".json");
        }
        let lin = c3_linearize(parent, dpath);
        parent_linearizations.push(lin);
    }

    let mut merged = c3_merge(parent_linearizations);

    merged.insert(0, fname);

    merged
}

fn c3_merge(mut ls: Vec<Vec<String>>) -> Vec<String> {

    let mut res = vec![];

    while !ls.is_empty() {
        res.push(c3_merge_pass(&mut ls).expect("could not linearize inheritance"))
    }

    res
}

fn c3_merge_pass(ls: &mut Vec<Vec<String>>) -> Option<String> {

    let mut res = None;

    for v in ls.iter() {
        let elem = v.first().unwrap();
        let mut b = false;

        for v2 in ls.iter() {
            let (_, tail) = v2.split_at(1);
            if tail.contains(elem) {
                b = true;
                break;
            }
        }

        if !b {
            res = Some(elem.to_string());
            break;
        }
    }

    if let Some(ref val) = res {
        for i in (0..ls.len()).rev() {

            ls[i].retain(|x| x != val);

            if ls[i].is_empty() {
                ls.remove(i);
            }
        }
    }

    res
}

fn create_gitignore() {
    fs::write(".gitignore", "").expect("failed to create .gitignore");
}

fn create_readme(name: &str) {
    fs::write("README.md", format!("# {}", name)).expect("failed to create readme");
}

fn create_license(profs: &HashMap<String, Value>) {

    let now = chrono::Local::now();
    let year = now.year().to_string();

    let name = match profs["name"].as_str() {
        Some(n) => n,
        None => "[NAME]"
    };

    let license_text = match profs["license"].as_str() {
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
