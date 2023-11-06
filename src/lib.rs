use duct::cmd;
use std::fs;
use std::io::{prelude::*, Error};
use toml_edit::{value, Document};
use typed_builder::TypedBuilder;

#[derive(PartialEq, Debug, TypedBuilder)]
pub struct CargoCommandOptions {
    runs: u8,
    #[builder(default = 0)]
    warmup_runs: u8,
    prepare_command: String,
    cargo_command: String,
    output_dir: String,
    run_name: String,
    compile_path: String,
}

pub fn inspect(opts: &CargoCommandOptions) -> Result<(), Error> {
    let runs = format!("--runs={}", &opts.runs);
    std::env::set_current_dir(&opts.compile_path).unwrap();

    cmd!("pwd").run()?;
    cmd!(
        "hyperfine",
        "-p",
        &opts.prepare_command,
        "--warmup",
        &opts.warmup_runs.to_string(),
        &opts.cargo_command,
        "--export-json",
        format!("{}/{}.json", &opts.output_dir, &opts.run_name),
        &runs
    )
    .run()?;
    Ok(())
}
pub fn enable_o3(cargo_dir: &str) -> Result<(), Error> {
    let mut cargo_toml = fs::File::open(format!("{cargo_dir}/Cargo.toml"))?;
    let mut cargo_toml_string = String::new();
    cargo_toml.read_to_string(&mut cargo_toml_string)?;

    let mut toml = cargo_toml_string.parse::<Document>().unwrap();

    toml["profile"]["dev"]["opt-level"] = value(1);
    toml["profile"]["dev"]["package"]["*"]["opt-level"] = value(3);

    let new_toml_string = toml.to_string();

    let mut cargo_toml = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(format!("{cargo_dir}/Cargo.toml"))?;
    cargo_toml
        .write_all(new_toml_string.as_bytes())
        .expect("Failed Write");
    Ok(())
}

pub fn disable_o3(cargo_dir: &str) -> Result<(), Error> {
    let mut cargo_toml = fs::File::open(format!("{cargo_dir}/Cargo.toml"))?;
    let mut cargo_toml_string = String::new();
    cargo_toml.read_to_string(&mut cargo_toml_string)?;

    let mut toml = cargo_toml_string.parse::<Document>().unwrap();

    toml["profile"]["dev"]["opt-level"] = value(0);
    toml["profile"]["dev"]["package"]["*"]["opt-level"] = value(0);

    let new_toml_string = toml.to_string();

    let mut cargo_toml = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(format!("{cargo_dir}/Cargo.toml"))?;
    cargo_toml
        .write_all(new_toml_string.as_bytes())
        .expect("Failed Write");
    Ok(())
}
pub fn enable_cranelift(cargo_dir: &str) -> Result<(), Error> {
    let mut config_toml = fs::File::open(format!("{cargo_dir}/.cargo/config.toml"))?;
    let mut config_toml_string = String::new();
    config_toml.read_to_string(&mut config_toml_string)?;

    let mut toml = config_toml_string.parse::<Document>().unwrap();
    let codegen = &mut toml["profile"]["server-dev"];
    if let toml_edit::Item::Table(t) = codegen {
        if t.contains_key("codegen-backend") {
            return Ok(());
        } else {
            t["codegen-backend"] = value("cranelift");
        }
    }

    let new_toml_string = toml.to_string();

    let mut config_toml = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(format!("{cargo_dir}/.cargo/config.toml"))?;
    config_toml
        .write_all(new_toml_string.as_bytes())
        .expect("Failed Write");

    let mut config_toml = fs::File::open(format!("{cargo_dir}/.cargo/config.toml"))?;
    let mut config_toml_string = String::new();
    config_toml.read_to_string(&mut config_toml_string)?;
    Ok(())
}
pub fn disable_cranelift(cargo_dir: &str) -> Result<(), Error> {
    let mut config_toml = fs::File::open(format!("{cargo_dir}/.cargo/config.toml"))?;
    let mut config_toml_string = String::new();
    config_toml.read_to_string(&mut config_toml_string)?;

    let mut toml = config_toml_string.parse::<Document>().unwrap();
    let codegen = &mut toml["profile"]["server-dev"];
    if let toml_edit::Item::Table(t) = codegen {
        if t.contains_key("codegen-backend") {
            t.remove("codegen-backend");
        } else {
            return Ok(());
        }
    }

    let new_toml_string = toml.to_string();

    let mut config_toml = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(format!("{cargo_dir}/.cargo/config.toml"))?;
    config_toml
        .write_all(new_toml_string.as_bytes())
        .expect("Failed Write");

    let mut config_toml = fs::File::open(format!("{cargo_dir}/.cargo/config.toml"))?;
    let mut config_toml_string = String::new();
    config_toml.read_to_string(&mut config_toml_string)?;
    Ok(())
}
