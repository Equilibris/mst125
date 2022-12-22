{
  inputs = {
    nixpkgs.url = "nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = { self, flake-utils, fenix, nixpkgs, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system} // { inherit (
          fenix.packages.${system}.latest
        ) rust-src; };
      in rec {
        devShell = pkgs.mkShell {
          packages = with pkgs; [
            python310
            (fenix.packages."${system}".latest.withComponents [
              "cargo"
              "clippy"
              "rust-src"
              "rustc"
              "rustfmt"
              "miri"
            ])
          ];
          RUST_SRC_PATH = "${pkgs.rust-src}/lib/rustlib/src/rust/library";
        };

        shellHook = ''
          export DEBUG=1
          cargo build
        '';
      });
}
