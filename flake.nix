
{
  description = "A basic Rust devshell";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            openssl
            pkg-config
            mold
            llvmPackages_latest.llvm
            llvmPackages_latest.bintools
            llvmPackages_latest.lld
            (rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
              extensions= [ "rust-src" "rust-analyzer" "rustc-codegen-cranelift" ];
              targets = [ "wasm32-unknown-unknown" ];
            }))
          ];

          shellHook = ''
            '';
        };
      }
    );
}
