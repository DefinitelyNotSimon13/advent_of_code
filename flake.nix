{
  description = "A basic flake with a shell";
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-24.11";
  inputs.systems.url = "github:nix-systems/default";
  inputs.flake-utils = {
    url = "github:numtide/flake-utils";
    inputs.systems.follows = "systems";
  };

  outputs =
    { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            pkg-config
            bacon
            clippy
            cargo
            rustc
            rustfmt
            rust-analyzer
            cargo-watch
          ];

          buildInputs =
            with pkgs;
            [
            ];

        };
      }
    );
}
