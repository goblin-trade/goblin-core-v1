{
  description = "Goblin flake using Nitro devShell";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    # Nitro development environment
    nitro.url = "github:EspressoSystems/nitro-espresso-integration/def1a60f975b8a726186c97c93c2c6e808527b93";
  };

  outputs = { self, nixpkgs, nitro, ... }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
      };

      rustPlatform = pkgs.rustPlatform;

      cargoStylus = rustPlatform.buildRustPackage rec {
        pname = "cargo-stylus";
        version = "0.5.3";

        src = pkgs.fetchFromGitHub {
          owner = "OffchainLabs";
          repo = "cargo-stylus";
          rev = "v${version}";
          hash = "sha256-2KkiwX2CYt155YxY9CQ3uGwZRIl5lsnyIoYcPGaTneI=";
        };

        cargoHash = "sha256-fmsMAarWdedbY856NWdwElQQLaJCUFJ5Eb9o1vgArcE=";

        nativeBuildInputs = [
          pkgs.pkg-config
        ];

        buildInputs = [
          pkgs.openssl
        ];

        meta = {
          description = "Stylus development CLI for Arbitrum";
          homepage = "https://github.com/OffchainLabs/cargo-stylus";
          license = pkgs.lib.licenses.mit;
        };
      };


      nitroShell = nitro.devShells.${system}.default;
    in {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = nitroShell.buildInputs ++ [
          pkgs.sqlx-cli
          cargoStylus
        ];

        shellHook = ''
          ${nitroShell.shellHook or ""}

          # Goblin-specific environment setup
          export ETH_RPC_URL="http://127.0.0.1:8547"
          export ADDRESS="0x3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E"
          export PRIVATE_KEY="0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659"

          export ARB_WASM_CONTRACT="0x0000000000000000000000000000000000000071"
          export CREATE3_FACTORY="0xA6E41fFD769491a42A6e5Ce453259b93983a22EF"
          export GOBLIN_SALT="0x000000000000000000000000000000000000000000000000400000000000485b"
          export CONTRACT="0x8888415db80eabcf580283a3d65249887d3161b0"

          export BASE_TOKEN="0xe1080224B632A93951A7CFA33EeEa9Fd81558b5e"
          export QUOTE_TOKEN="0x3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E"
        '';
      };
    };
}
