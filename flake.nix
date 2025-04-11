{
  description = "CLI for COSMIC Desktop configuration management";

  inputs = {
    flake-compat.url = "github:edolstra/flake-compat";

    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };

    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ inputs.treefmt-nix.flakeModule ];

      systems = [
        "aarch64-linux"
        "x86_64-linux"
      ];

      perSystem =
        { pkgs, self', ... }:
        {
          devShells.default = pkgs.callPackage ./nix/shell.nix { };

          packages = {
            default = self'.packages.cosmic-ext-ctl;
            cosmic-ext-ctl = pkgs.callPackage ./nix/package.nix { };
          };

          treefmt = {
            flakeCheck = true;

            programs = {
              nixfmt.enable = true;
              rustfmt.enable = true;
            };

            projectRootFile = "flake.nix";
          };
        };
    };
}
