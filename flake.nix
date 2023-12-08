{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
        diesel-cli = pkgs.diesel-cli.override { postgresqlSupport = true; };
      in
      {
        defaultPackage = naersk-lib.buildPackage {
          src = ./.;
          PQ_LIB_DIR = "${pkgs.postgresql.lib}/lib";
        };

        devShell = with pkgs; mkShell {
          buildInputs = [ cargo rustc rustfmt pre-commit rustPackages.clippy rust-analyzer postgresql diesel-cli ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
      });
}
