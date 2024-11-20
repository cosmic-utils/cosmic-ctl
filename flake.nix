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
        function:
        inputs.nixpkgs.lib.genAttrs supportedSystems (
          system: function inputs.nixpkgs.legacyPackages.${system}
        );
    in
    {
      devShells = forAllSystems (pkgs: {
        default = import ./shell.nix { inherit pkgs; };
      });

      formatter = forAllSystems (pkgs: pkgs.treefmt);

      packages = forAllSystems (pkgs: {
        default = pkgs.callPackage ./. { };
      });
    };
}
