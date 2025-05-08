{
  description = "Application to set wallpapers from reddit as desktop-background";
  inputs = {
    os_flake.url = "github:septias/nixos-config";
    nixpkgs.follows = "os_flake/nixpkgs";
    nixpkgs-unstable.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
  };
  outputs = inputs:
    with inputs;
      flake-utils.lib.eachDefaultSystem (
        system: let
          pkgs = import nixpkgs {
            overlays = [(import rust-overlay)];
            inherit system;
          };
          rust-toolchain = pkgs.rust-bin.stable.latest.default.override {
            extensions = ["rust-src" "rustfmt" "rust-docs" "clippy" "rust-analyzer"];
          };
        in {
          formatter = pkgs.alejandra;
          devShells.default = pkgs.mkShell {
            buildInputs = [rust-toolchain];
          };
        }
      );
}
