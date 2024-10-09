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

          GIT_COMMIT_HASH_SHORT = self.shortRev or "unknown";

        };

        packages = {
          ethereumEs = ethereumEs;
          default = packages.ethereumEs;
        };

       overlays.default = final: prev: { ethereumEs = packages.ethereumEs; };

        gitRev = if (builtins.hasAttr "rev" self) then self.rev else "dirty";
      in {
        inherit packages overlays;

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

            nodejs
            nodePackages.typescript
            nodePackages.typescript-language-server

            docker
            watchexec
            cowsay
          ];
          shellHook = ''
            alias ls=eza
            alias find=fd
            echo "hello-world-avs" | cowsay

            export RUST_SRC_PATH="${toolchain}/lib/rustlib/src/rust/library"
            export CARGO_HOME="$(pwd)/.cargo"
            export PATH="$CARGO_HOME/bin:$PATH"
            export RUST_BACKTRACE=1
            export CARGO_NET_GIT_FETCH_WITH_CLI=true
            export ETH_RPC_URL='127.0.0.1:8545'
            export ETHERSCAN_API_KEY='11'
            cargo install cargo-expand
            cargo install --version 0.5.7 sqlx-cli --no-default-features --features postgres
            # cargo install cargo-udeps ## lets not do this till we ready for prod
            cargo install bunyan
          '';
        };
      });
}
