# Customs
A crate to make it slightly easier to benchmark cargo runs under different settings.

## Installation
1. Install [hyperfine](https://github.com/sharkdp/hyperfine), which we use for benchmarking. There are many ways to do it, but one is below
```bash
cargo install hyperfine
```
2. Install [mold](https://github.com/rui314/mold), which might involve compiling it. See the page for details. I use Nix, which has a package for it already.
It is an alternative linker for Rust programs written by @rui314. Consider sponsoring them if it works well for you
3. Install [cranelift](https://github.com/rust-lang/rustc_codegen_cranelift), which is now included in
as a rustup component in the latest nightly for x86_64 linux. Details below. It shows that it works for other platforms,
see the README for details
```bash
rustup component add rustc-codegen-cranelift-preview --toolchain nightly
```
## Usage
1. Create a dev-server profile in Cargo.toml by adding the section below
```toml
[profile.server-dev]
inherits="dev"
```
2. Enable the unstable codegen-backend feature in .cargo/config.toml. If the folder or file doesn't exist, create them.
To actually enable it, you need to add `codegen-backend = "cranelift"` below the profile name, but 
the tool will handle this for you.
```toml
[unstable]
codegen-backend = true

[profile.server-dev]
```
3. Run it
```bash
cargo run -- --cargo-dir=<"path_to_dir_to_compile"> --output-dir=<"path to dir to put output json files in" > --num-runs=3
```
4. Available commands can be found
```bash
cargo run -- --help
```

## Note
This is currently in a very beta state. I will probably break the bin crate out into an example.
