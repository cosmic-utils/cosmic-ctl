{
  pkgs ? import <nixpkgs> { },
  ...
}:
pkgs.mkShell {
  strictDeps = true;

  nativeBuildInputs = with pkgs; [
    cargo
    clippy
    nixfmt-rfc-style
    rustc
    rustfmt
  ];
}
