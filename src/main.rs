use clap::Parser;
use customs::{
    disable_cranelift, disable_o3, enable_cranelift, enable_o3, inspect, CargoCommandOptions,
};
use std::io::Error;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    cargo_dir: String,
    #[arg(short, long)]
    output_dir: String,
    #[arg(short, long, default_value_t = 10)]
    num_runs: u8,
    #[arg(short, long, default_value_t = 0)]
    num_warmup_runs: u8,
}

fn main() -> Result<(), Error> {
    println!("Running customs check on cargo!");
    let args = Args::parse();
    let sed_command =
        r#"sed -i -e "s|<dfn>[^<]*</dfn>|<dfn>$(date +%m%s)</dfn>|g" src/routes/index.rs"#;
    //1. Assume a clean state for compilation tests
    //2. Do clean compile and measure results with hyperfine?
    //3. Incremental compile run with hyperfine (Clean)
    println!("Default Options");
    let clean_clean = CargoCommandOptions::builder()
        .prepare_command("cargo clean".to_string())
        .num_runs(args.num_runs)
        .cargo_command("cargo leptos build".to_string())
        .output_dir(args.output_dir.to_string())
        .run_name("clean".to_string())
        .compile_path(args.cargo_dir.to_string())
        .build();
    inspect(&clean_clean)?;
    let clean_incremental = CargoCommandOptions::builder()
        .prepare_command(sed_command.to_string())
        .num_runs(args.num_runs)
        .cargo_command("cargo leptos build".to_string())
        .output_dir(args.output_dir.to_string())
        .run_name("clean".to_string())
        .compile_path(args.cargo_dir.to_string())
        .build();
    inspect(&clean_incremental)?;

    //4. Enable Mold (Mold)
    println!("Mold Enabled");
    let clean_clean = CargoCommandOptions::builder()
        .prepare_command("cargo clean".to_string())
        .num_runs(args.num_runs)
        .cargo_command("mold -run cargo leptos build".to_string())
        .output_dir(args.output_dir.to_string())
        .run_name("clean_mold".to_string())
        .compile_path(args.cargo_dir.to_string())
        .build();
    inspect(&clean_clean)?;

    let clean_incremental = CargoCommandOptions::builder()
        .prepare_command(sed_command.to_string())
        .num_runs(args.num_runs)
        .cargo_command("mold -run cargo leptos build".to_string())
        .output_dir(args.output_dir.to_string())
        .run_name("incremental_mold".to_string())
        .compile_path(args.cargo_dir.to_string())
        .build();
    inspect(&clean_incremental)?;

    //6. Enable O3 (Mold and O3)
    enable_o3(&args.cargo_dir)?;
    println!("O3 and Mold Enabled");
    let clean_clean = CargoCommandOptions::builder()
        .prepare_command("cargo clean".to_string())
        .num_runs(args.num_runs)
        .cargo_command("mold -run cargo leptos build".to_string())
        .output_dir(args.output_dir.to_string())
        .run_name("clean_mold_o3".to_string())
        .compile_path(args.cargo_dir.to_string())
        .build();
    inspect(&clean_clean)?;

    let clean_incremental = CargoCommandOptions::builder()
        .prepare_command(sed_command.to_string())
        .num_runs(args.num_runs)
        .num_warmup_runs(args.num_warmup_runs)
        .cargo_command("mold -run cargo leptos build".to_string())
        .output_dir(args.output_dir.to_string())
        .run_name("incremental_mold_o3".to_string())
        .compile_path(args.cargo_dir.to_string())
        .build();
    inspect(&clean_incremental)?;

    //8. Enable Cranelift (Mold, O3, and Cranelift)
    enable_cranelift(&args.cargo_dir)?;
    println!("O3, Mold, And Cranelift Enabled");
    let clean_clean = CargoCommandOptions::builder()
        .prepare_command("cargo clean".to_string())
        .num_runs(args.num_runs)
        .cargo_command("mold -run cargo leptos build".to_string())
        .output_dir(args.output_dir.to_string())
        .run_name("clean_mold_o3_cranelift".to_string())
        .compile_path(args.cargo_dir.to_string())
        .build();
    inspect(&clean_clean)?;

    let clean_incremental = CargoCommandOptions::builder()
        .prepare_command(sed_command.to_string())
        .num_runs(args.num_runs)
        .cargo_command("mold -run cargo leptos build".to_string())
        .output_dir(args.output_dir.to_string())
        .run_name("incremental_mold_o3_cranelift".to_string())
        .compile_path(args.cargo_dir.to_string())
        .build();
    inspect(&clean_incremental)?;

    //10. Disable Mold
    println!("Cranelift and O3 Enabled");
    let clean_clean = CargoCommandOptions::builder()
        .prepare_command("cargo clean".to_string())
        .num_runs(args.num_runs)
        .cargo_command("cargo leptos build".to_string())
        .output_dir(args.output_dir.to_string())
        .run_name("clean_o3_cranelift".to_string())
        .compile_path(args.cargo_dir.to_string())
        .build();
    inspect(&clean_clean)?;

    let clean_incremental = CargoCommandOptions::builder()
        .prepare_command(sed_command.to_string())
        .num_runs(args.num_runs)
        .cargo_command("cargo leptos build".to_string())
        .output_dir(args.output_dir.to_string())
        .run_name("incremental_o3_cranelift".to_string())
        .compile_path(args.cargo_dir.to_string())
        .build();
    inspect(&clean_incremental)?;

    //12. Disable O3 and Enable Mold (Mold and Cranelift)
    println!("Cranelift and Mold Enabled");
    disable_o3(&args.cargo_dir)?;
    let clean_clean = CargoCommandOptions::builder()
        .prepare_command("cargo clean".to_string())
        .num_runs(args.num_runs)
        .cargo_command("mold -run cargo leptos build".to_string())
        .output_dir(args.output_dir.to_string())
        .run_name("clean_mold_cranelift".to_string())
        .compile_path(args.cargo_dir.to_string())
        .build();
    inspect(&clean_clean)?;

    let clean_incremental = CargoCommandOptions::builder()
        .prepare_command(sed_command.to_string())
        .num_runs(args.num_runs)
        .cargo_command("mold -run cargo leptos build".to_string())
        .output_dir(args.output_dir.to_string())
        .run_name("incremental_mold_cranelift".to_string())
        .compile_path(args.cargo_dir.to_string())
        .build();
    inspect(&clean_incremental)?;

    //12. Disable Mold(Cranelift)
    println!("Cranelift Enabled");
    disable_o3(&args.cargo_dir)?;
    let clean_clean = CargoCommandOptions::builder()
        .prepare_command("cargo clean".to_string())
        .num_runs(args.num_runs)
        .cargo_command("cargo leptos build".to_string())
        .output_dir(args.output_dir.to_string())
        .run_name("clean_cranelift".to_string())
        .compile_path(args.cargo_dir.to_string())
        .build();
    inspect(&clean_clean)?;

    let clean_incremental = CargoCommandOptions::builder()
        .prepare_command(sed_command.to_string())
        .num_runs(args.num_runs)
        .cargo_command("cargo leptos build".to_string())
        .output_dir(args.output_dir.to_string())
        .run_name("incremental_cranelift".to_string())
        .compile_path(args.cargo_dir.to_string())
        .build();
    inspect(&clean_incremental)?;

    //12. Disable Cranelift(O3)
    println!("O3 Enabled");
    disable_cranelift(&args.cargo_dir)?;
    enable_o3(&args.cargo_dir)?;
    let clean_clean = CargoCommandOptions::builder()
        .prepare_command("cargo clean".to_string())
        .num_runs(args.num_runs)
        .cargo_command("cargo leptos build".to_string())
        .output_dir(args.output_dir.to_string())
        .run_name("clean_o3".to_string())
        .compile_path(args.cargo_dir.to_string())
        .build();
    inspect(&clean_clean)?;

    let clean_incremental = CargoCommandOptions::builder()
        .prepare_command(sed_command.to_string())
        .num_runs(args.num_runs)
        .cargo_command("cargo leptos build".to_string())
        .output_dir(args.output_dir.to_string())
        .run_name("incremental_o3".to_string())
        .compile_path(args.cargo_dir.to_string())
        .build();
    inspect(&clean_incremental)?;
    disable_o3(&args.cargo_dir)?;

    // Summarize Results
    Ok(())
}
