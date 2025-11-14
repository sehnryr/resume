{
  description = "my resume framework development environment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixpkgs-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      fenix,
      ...
    }:
    let
      inherit (nixpkgs) lib;

      supportedSystems = [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-darwin"
        "x86_64-linux"
      ];

      forAllSystems =
        systems: f:
        lib.genAttrs systems (
          system:
          f (
            import nixpkgs {
              inherit system;
              overlays = [ fenix.overlays.default ];
            }
          )
        );
    in
    {
      devShells = forAllSystems supportedSystems (pkgs: {
        default = pkgs.mkShell {
          buildInputs = [
            pkgs.rust-analyzer

            # Rust components
            pkgs.fenix.stable.rustc
            pkgs.fenix.stable.cargo
            pkgs.fenix.stable.rust-std
            pkgs.fenix.stable.clippy
            pkgs.fenix.stable.rust-src
            pkgs.fenix.stable.rust-docs
            pkgs.fenix.latest.rustfmt

            # Web server for development
            pkgs.miniserve
          ];
        };
      });
    };
}
