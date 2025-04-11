{
  mkShell,
  cargo,
  clippy,
  rustc,
  rustfmt,
}:
mkShell {
  strictDeps = true;

  nativeBuildInputs = [
    cargo
    clippy
    rustc
    rustfmt
  ];
}
