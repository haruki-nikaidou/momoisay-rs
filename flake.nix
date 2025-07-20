{
  description = "A CLI tool to display ASCII art of Momoi";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
        momoisay = pkgs.rustPlatform.buildRustPackage rec {
          pname = "momoisay";
          version = "0.1.0";
          src = pkgs.lib.cleanSource ./.;
          cargoLock.lockFile = ./Cargo.lock;
          meta = with pkgs.lib; {
            description = "A CLI tool to display ASCII art of Momoi";
            homepage = "https://github.com/haruki-nikaidou/momoisay-rs";
            license = licenses.gpl3;
            maintainers = with maintainers; [ haruki-nikaidou ];
            platforms = platforms.all;
          };
        };
      in
      {
        packages = {
          momoisay = momoisay;
          default = momoisay;
        };
        app.default = flake-utils.lib.mkApp {
          drv = momoisay;
          exe = "momoisay";
        };
        checks = {
          inherit momoisay;
        };
      }
    );
}
