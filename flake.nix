{
  description = "Goblin flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            pkgs.sqlx-cli
          ];

          shellHook = ''
            # Goblin-specific environment setup
            export ETH_RPC_URL="http://127.0.0.1:8547"
            export ADDRESS="0x3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E"
            export PRIVATE_KEY="0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659"

            export ARB_WASM_CONTRACT="0x0000000000000000000000000000000000000071"
            export CREATE3_FACTORY="0x525c2aBA45F66987217323E8a05EA400C65D06DC"
            export GOBLIN_SALT="0x000000000000000000000000000000000000000000000000a000000000000107"
            export CONTRACT="0x8888ef09a63b6328468fce63a09fc185de807722"

            export BASE_TOKEN="0x85D9a8a4bd77b9b5559c1B7FCb8eC9635922Ed49"
            export QUOTE_TOKEN="0x4A2bA922052bA54e29c5417bC979Daaf7D5Fe4f4"

            # SQLite database
            export DATABASE_URL="sqlite://$XDG_DATA_HOME/goblin/goblin.db"

            # Timezone
            export TZ="GMT-2"
          '';
        };
      });
}
