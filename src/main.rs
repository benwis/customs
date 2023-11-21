use clap::Parser;
use customs::{
    disable_cranelift, disable_mold, disable_o3, enable_cranelift, enable_mold, enable_o3,
    enable_parallel, inspect, CargoCommandOptions,
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
    runs: u8,
    #[arg(short, long, default_value_t = 0)]
    warmup_runs: u8,
}

fn main() -> Result<(), Error> {
    println!("Running customs check on cargo!");
    let args = Args::parse();
    let sed_command =
        r#"sed -i -e "s|<dfn>[^<]*</dfn>|<dfn>$(date +%m%s)</dfn>|g" app/src/routes/index.rs"#;
    //1. Assume a clean state for compilation tests
    //2. Do clean comcargo_dirompile run with hyperfine (Clean)
    println!("Default Options");
    let clean_clean = CargoCommandOptions::builder()
        .prepare_command("cargo clean".to_string())
        .runs(args.runs)
        .cargo_command("cargo leptos build".to_string())
        .output_dir(args.output_dir.to_string())
        .run_name("clean".to_string())
        .compile_path(args.cargo_dir.to_string())
        .build();
    inspect(&clean_clean)?;
    let clean_incremental = CargoCommandOptions::builder()
        .prepare_command(sed_command.to_string())
        .runs(args.runs)
        .cargo_command("cargo leptos build".to_string())
        .output_dir(args.output_dir.to_string())
        .run_name("incremental".to_string())
        .compile_path(args.cargo_dir.to_string())
        .build();
    inspect(&clean_incremental)?;

    //4. Enable Mold (Mold)
    println!("Mold Enabled");
    enable_mold(&args.cargo_dir)?;
    let clean_clean = CargoCommandOptions::builder()
        .prepare_command("cargo clean".to_string())
        .runs(args.runs)
        .cargo_command(" cargo leptos build".to_string())
        .output_dir(args.output_dir.to_string())
        .run_name("clean_mold".to_string())
        .compile_path(args.cargo_dir.to_string())
        .build();
    inspect(&clean_clean)?;

    let clean_incremental = CargoCommandOptions::builder()
        .prepare_command(sed_command.to_string())
        .runs(args.runs)
        .cargo_command(" cargo leptos build".to_string())
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
        .runs(args.runs)
        .cargo_command(" cargo leptos build".to_string())
        .output_dir(args.output_dir.to_string())
        .run_name("clean_mold_o3".to_string())
        .compile_path(args.cargo_dir.to_string())
        .build();
    inspect(&clean_clean)?;

    let clean_incremental = CargoCommandOptions::builder()
        .prepare_command(sed_command.to_string())
        .runs(args.runs)
        .warmup_runs(args.warmup_runs)
        .cargo_command(" cargo leptos build".to_string())
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
        .runs(args.runs)
        .cargo_command(" cargo leptos build".to_string())
        .output_dir(args.output_dir.to_string())
        .run_name("clean_mold_o3_cranelift".to_string())
        .compile_path(args.cargo_dir.to_string())
        .build();
    inspect(&clean_clean)?;

    let clean_incremental = CargoCommandOptions::builder()
        .prepare_command(sed_command.to_string())
        .runs(args.runs)
        .cargo_command(" cargo leptos build".to_string())
        .output_dir(args.output_dir.to_string())
        .run_name("incremental_mold_o3_cranelift".to_string())
        .compile_path(args.cargo_dir.to_string())
        .build();
    inspect(&clean_incremental)?;

    //10. Disable Mold
    disable_mold(&args.cargo_dir)?;
    println!("Cranelift and O3 Enabled");
    let clean_clean = CargoCommandOptions::builder()
        .prepare_command("cargo clean".to_string())
        .runs(args.runs)
        .cargo_command("cargo leptos build".to_string())
        .output_dir(args.output_dir.to_string())
        .run_name("clean_o3_cranelift".to_string())
        .compile_path(args.cargo_dir.to_string())
        .build();
    inspect(&clean_clean)?;

    let clean_incremental = CargoCommandOptions::builder()
        .prepare_command(sed_command.to_string())
        .runs(args.runs)
        .cargo_command("cargo leptos build".to_string())
        .output_dir(args.output_dir.to_string())
        .run_name("incremental_o3_cranelift".to_string())
        .compile_path(args.cargo_dir.to_string())
        .build();
    inspect(&clean_incremental)?;

    //12. Disable O3 and Enable Mold (Mold and Cranelift)
    println!("Cranelift and Mold Enabled");
    disable_o3(&args.cargo_dir)?;
    enable_mold(&args.cargo_dir)?;
    let clean_clean = CargoCommandOptions::builder()
        .prepare_command("cargo clean".to_string())
        .runs(args.runs)
        .cargo_command(" cargo leptos build".to_string())
        .output_dir(args.output_dir.to_string())
        .run_name("clean_mold_cranelift".to_string())
        .compile_path(args.cargo_dir.to_string())
        .build();
    inspect(&clean_clean)?;

    let clean_incremental = CargoCommandOptions::builder()
        .prepare_command(sed_command.to_string())
        .runs(args.runs)
        .cargo_command(" cargo leptos build".to_string())
        .output_dir(args.output_dir.to_string())
        .run_name("incremental_mold_cranelift".to_string())
        .compile_path(args.cargo_dir.to_string())
        .build();
    inspect(&clean_incremental)?;

    //12. Disable Mold(Cranelift)
    println!("Cranelift Enabled");
    disable_mold(&args.cargo_dir)?;
    let clean_clean = CargoCommandOptions::builder()
        .prepare_command("cargo clean".to_string())
        .runs(args.runs)
        .cargo_command("cargo leptos build".to_string())
        .output_dir(args.output_dir.to_string())
        .run_name("clean_cranelift".to_string())
        .compile_path(args.cargo_dir.to_string())
        .build();
    inspect(&clean_clean)?;

    let clean_incremental = CargoCommandOptions::builder()
        .prepare_command(sed_command.to_string())
        .runs(args.runs)
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
        .runs(args.runs)
        .cargo_command("cargo leptos build".to_string())
        .output_dir(args.output_dir.to_string())
        .run_name("clean_o3".to_string())
        .compile_path(args.cargo_dir.to_string())
        .build();
    inspect(&clean_clean)?;

    let clean_incremental = CargoCommandOptions::builder()
        .prepare_command(sed_command.to_string())
        .runs(args.runs)
        .cargo_command("cargo leptos build".to_string())
        .output_dir(args.output_dir.to_string())
        .run_name("incremental_o3".to_string())
        .compile_path(args.cargo_dir.to_string())
        .build();
    inspect(&clean_incremental)?;
    disable_o3(&args.cargo_dir)?;

    // Enable Parallel Compilation
    println!("Parallel Enabled");
    enable_parallel(&args.cargo_dir)?;
    let clean_clean = CargoCommandOptions::builder()
        .prepare_command("cargo clean".to_string())
        .runs(args.runs)
        .cargo_command(" cargo leptos build".to_string())
        .output_dir(args.output_dir.to_string())
        .run_name("clean_parallel".to_string())
        .compile_path(args.cargo_dir.to_string())
        .build();
    inspect(&clean_clean)?;

    let clean_incremental = CargoCommandOptions::builder()
        .prepare_command(sed_command.to_string())
        .runs(args.runs)
        .cargo_command(" cargo leptos build".to_string())
        .output_dir(args.output_dir.to_string())
        .run_name("incremental_parallel".to_string())
        .compile_path(args.cargo_dir.to_string())
        .build();
    inspect(&clean_incremental)?;

    // Enable Parallel and O3
    println!("Parallel and O3 Enabled");
    enable_o3(&args.cargo_dir)?;

    let clean_clean = CargoCommandOptions::builder()
        .prepare_command("cargo clean".to_string())
        .runs(args.runs)
        .cargo_command(" cargo leptos build".to_string())
        .output_dir(args.output_dir.to_string())
        .run_name("clean_parallel_o3".to_string())
        .compile_path(args.cargo_dir.to_string())
        .build();
    inspect(&clean_clean)?;

    let clean_incremental = CargoCommandOptions::builder()
        .prepare_command(sed_command.to_string())
        .runs(args.runs)
        .cargo_command(" cargo leptos build".to_string())
        .output_dir(args.output_dir.to_string())
        .run_name("incremental_parallel_o3".to_string())
        .compile_path(args.cargo_dir.to_string())
        .build();
    inspect(&clean_incremental)?;

    // Enable Parallel, O3, and Cranelift
    println!("Parallel, O3, and Cranelift Enabled");
    enable_cranelift(&args.cargo_dir)?;
    let clean_clean = CargoCommandOptions::builder()
        .prepare_command("cargo clean".to_string())
        .runs(args.runs)
        .cargo_command(" cargo leptos build".to_string())
        .output_dir(args.output_dir.to_string())
        .run_name("clean_parallel_o3_cranelift".to_string())
        .compile_path(args.cargo_dir.to_string())
        .build();
    inspect(&clean_clean)?;

    let clean_incremental = CargoCommandOptions::builder()
        .prepare_command(sed_command.to_string())
        .runs(args.runs)
        .cargo_command(" cargo leptos build".to_string())
        .output_dir(args.output_dir.to_string())
        .run_name("incremental_parallel_o3_cranelift".to_string())
        .compile_path(args.cargo_dir.to_string())
        .build();
    inspect(&clean_incremental)?;

    // Enable Parallel, O3, Cranelift, and Mold
    println!("Parallel, O3, Cranelift, and Mold Enabled");
    enable_mold(&args.cargo_dir)?;
    let clean_clean = CargoCommandOptions::builder()
        .prepare_command("cargo clean".to_string())
        .runs(args.runs)
        .cargo_command(" cargo leptos build".to_string())
        .output_dir(args.output_dir.to_string())
        .run_name("clean_parallel_o3_cranelift_mold".to_string())
        .compile_path(args.cargo_dir.to_string())
        .build();
    inspect(&clean_clean)?;

    let clean_incremental = CargoCommandOptions::builder()
        .prepare_command(sed_command.to_string())
        .runs(args.runs)
        .cargo_command(" cargo leptos build".to_string())
        .output_dir(args.output_dir.to_string())
        .run_name("incremental_parallel_o3_cranelift_mold".to_string())
        .compile_path(args.cargo_dir.to_string())
        .build();
    inspect(&clean_incremental)?;

    // Enable Parallel, Cranelift, and Mold
    println!("Parallel, Cranelift, and Mold Enabled");
    disable_o3(&args.cargo_dir)?;
    let clean_clean = CargoCommandOptions::builder()
        .prepare_command("cargo clean".to_string())
        .runs(args.runs)
        .cargo_command(" cargo leptos build".to_string())
        .output_dir(args.output_dir.to_string())
        .run_name("clean_parallel_cranelift_mold".to_string())
        .compile_path(args.cargo_dir.to_string())
        .build();
    inspect(&clean_clean)?;

    let clean_incremental = CargoCommandOptions::builder()
        .prepare_command(sed_command.to_string())
        .runs(args.runs)
        .cargo_command(" cargo leptos build".to_string())
        .output_dir(args.output_dir.to_string())
        .run_name("incremental_parallel_cranelift_mold".to_string())
        .compile_path(args.cargo_dir.to_string())
        .build();
    inspect(&clean_incremental)?;

    // Enable Parallel and Cranelift
    println!("Parallel and Cranelift Enabled");
    disable_mold(&args.cargo_dir)?;
    let clean_clean = CargoCommandOptions::builder()
        .prepare_command("cargo clean".to_string())
        .runs(args.runs)
        .cargo_command(" cargo leptos build".to_string())
        .output_dir(args.output_dir.to_string())
        .run_name("clean_parallel_cranelift".to_string())
        .compile_path(args.cargo_dir.to_string())
        .build();
    inspect(&clean_clean)?;

    let clean_incremental = CargoCommandOptions::builder()
        .prepare_command(sed_command.to_string())
        .runs(args.runs)
        .cargo_command("cargo leptos build".to_string())
        .output_dir(args.output_dir.to_string())
        .run_name("incremental_parallel_cranelift".to_string())
        .compile_path(args.cargo_dir.to_string())
        .build();
    inspect(&clean_incremental)?;

    // Enable Parallel and Mold
    println!("Parallel and Mold Enabled");
    disable_cranelift(&args.cargo_dir)?;
    enable_mold(&args.cargo_dir)?;
    let clean_clean = CargoCommandOptions::builder()
        .prepare_command("cargo clean".to_string())
        .runs(args.runs)
        .cargo_command("cargo leptos build".to_string())
        .output_dir(args.output_dir.to_string())
        .run_name("clean_parallel_mold".to_string())
        .compile_path(args.cargo_dir.to_string())
        .build();
    inspect(&clean_clean)?;

    let clean_incremental = CargoCommandOptions::builder()
        .prepare_command(sed_command.to_string())
        .runs(args.runs)
        .cargo_command("cargo leptos build".to_string())
        .output_dir(args.output_dir.to_string())
        .run_name("incremental_parallel_mold".to_string())
        .compile_path(args.cargo_dir.to_string())
        .build();
    inspect(&clean_incremental)?;

    // Summarize Results
    Ok(())
}
