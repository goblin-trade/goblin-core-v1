{
  description = "Goblin flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      pkgs = import nixpkgs { system = "x86_64-linux"; };
    in {
      devShell.x86_64-linux = pkgs.mkShell {
        buildInputs = [];

        # Move shellHook inside mkShell
        shellHook = ''
          export ETH_RPC_URL="http://127.0.0.1:8547"
          export ADDRESS="0x3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E"
          export PRIVATE_KEY="0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659"

          export CREATE3_FACTORY="0xA6E41fFD769491a42A6e5Ce453259b93983a22EF"
          export GOBLIN_SALT="0x000000000000000000000000000000000000000000000000400000000000485b"
          export CONTRACT="0x8888415db80eabcf580283a3d65249887d3161b0"

          # Base token at nonce 2, quote token at nonce 3
          export BASE_TOKEN="0xe1080224B632A93951A7CFA33EeEa9Fd81558b5e"
          export QUOTE_TOKEN="0x3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E"
        '';
      };
    };
}
