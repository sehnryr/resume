{
  description = "my resume development environment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            (import rust-overlay)
          ];
        };

        stableToolchain = pkgs.lib.hiPrio (
          pkgs.rust-bin.stable.latest.minimal.override {
            extensions = [
              "rust-docs"
              "clippy"
              "rust-src"
            ];
          }
        );

        nightlyFmt = pkgs.rust-bin.selectLatestNightlyWith (
          toolchain:
          toolchain.minimal.override {
            extensions = [ "rustfmt" ];
          }
        );
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Rust toolchain
            stableToolchain
            nightlyFmt

            # Lightweight HTTP server
            miniserve
          ];
        };
      }
    );
}
