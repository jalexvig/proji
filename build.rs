use std::{env, error::Error, fs::{self, File}, io::Write, path::Path};

// Build script to iterate over profiles - from https://stackoverflow.com/questions/50553370
fn main() -> Result<(), Box<Error>> {

    let dpath = env::current_dir()?.join("src").join("resources").join("profiles");

    let out_dir = env::var("OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join("all_profiles.rs");
    let mut all_profiles = File::create(&dest_path)?;

    writeln!(&mut all_profiles, r#"["#, )?;

    for f in fs::read_dir(dpath)? {
        let f = f?;

        if !f.file_type()?.is_file() {
            continue;
        };

        let name;

        if let Some(n) = f.file_name().to_str() {
            name = n.to_string();
        } else {
            continue;
        }

        writeln!(
            &mut all_profiles,
            r#"("{name}", include_bytes!("{fpath}")),"#,
            name = name,
            fpath = f.path().display(),
        )?;
    };

    writeln!(&mut all_profiles, r#"];"#, )?;

    Ok(())
}