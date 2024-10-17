{
  description = "ethereum-rs project";
  inputs = {
    nixpkgs.url = "github:NixOs/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    foundry.url = "github:shazow/foundry.nix/monthly"; # Use monthly branch for permanent releases

  };
  outputs = { self, nixpkgs, rust-overlay, flake-utils, foundry, ... }@inputs:
  flake-utils.lib.eachDefaultSystem (system:
  let
    pkgs = import nixpkgs {
      inherit system;
      overlays = [ rust-overlay.overlays.default foundry.overlay ];
    };

    toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
    cargoTomlContents = builtins.readFile ./Cargo.toml;
    version = (builtins.fromTOML cargoTomlContents).package.version;

    ethereumEs = pkgs.rustPlatform.buildRustPackage {
      inherit version;
      name = "ethereumEs";
      buildInputs = with pkgs; [ openssl ];
      nativeBuildInputs = with pkgs; [ pkg-config openssl.dev ];

      src = pkgs.lib.cleanSourceWith { src = self; };

      cargoLock.lockFile = ./Cargo.lock;

    };
  in {

    overlays.default = final: prev: { ethereumEs = ethereumEs; };

    gitRev = if (builtins.hasAttr "rev" self) then self.rev else "dirty";

    devShells.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        foundry-bin
        solc
        toolchain
        openssl
        cargo-insta
        pkg-config
        eza
        rust-analyzer-unwrapped
        nodejs_20
        nodePackages.typescript
        nodePackages.typescript-language-server
        watchexec
      ];
      shellHook = ''
        export RUST_SRC_PATH="${toolchain}/lib/rustlib/src/rust/library"
        export CARGO_HOME="$(pwd)/.cargo"
        export PATH="$CARGO_HOME/bin:$PATH"
        export RUST_BACKTRACE=1
        export RPC_URL='127.0.0.1:8545'
        export ETHERSCAN_API_KEY='fake-key'
        export HOLESKY_PRIVATE_KEY='fake-key'
        export PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
      '';
    };
  });
}
