{
  description = "CLI for COSMIC Desktop configuration management";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    inputs:
    let
      supportedSystems = [
        "aarch64-linux"
        "x86_64-linux"
      ];

      forAllSystems =
        f:
        inputs.nixpkgs.lib.listToAttrs (
          map (system: {
            name = system;
            value = f {
              inherit system;
              pkgs = import inputs.nixpkgs { inherit system; };
            };
          }) supportedSystems
        );
    in
    {
      devShells = forAllSystems (
        { pkgs, ... }:
        {
          default = import ./shell.nix { inherit pkgs; };
        }
      );

      formatter = forAllSystems ({ pkgs, ... }: pkgs.treefmt);

      packages = forAllSystems (
        { pkgs, system }:
        {
          default = inputs.self.packages.${system}.cosmic-ctl;
          cosmic-ctl = import ./. { inherit pkgs; };
        }
      );
    };
}
