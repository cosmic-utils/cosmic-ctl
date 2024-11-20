{
  pkgs ? import <nixpkgs> { },
  ...
}:
pkgs.mkShell {
  strictDeps = true;

  nativeBuildInputs = with pkgs; [
    cargo
    nixfmt-rfc-style
    rustc
    rustfmt
  ];
}
