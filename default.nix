{
  lib,
  rustPlatform,
  versionCheckHook,
  cosmic-comp,
}:
let
  version = "1.0.0";
in
rustPlatform.buildRustPackage {
  pname = "cosmic-ctl";
  inherit version;

  src = builtins.path {
    name = "cosmic-ctl-source";
    path = ./.;
  };

  cargoHash = "sha256-Nrg7NOAUrVQcwBz7nV3hILRYHr1dprQ5VJj2u7Zf3Q0=";

  doInstallCheck = true;
  nativeInstallCheckInputs = [ versionCheckHook ];
  versionCheckProgram = "${placeholder "out"}/bin/cosmic-ctl";

  meta = {
    description = "CLI for COSMIC Desktop configuration management";
    changelog = "https://github.com/HeitorAugustoLN/cosmic-ctl/releases/tag/v${version}";
    homepage = "https://github.com/HeitorAugustoLN/cosmic-ctl";
    license = lib.licenses.gpl3Only;
    maintainers = with lib.maintainers; [ HeitorAugustoLN ];
    mainProgram = "cosmic-ctl";
    inherit (cosmic-comp.meta) platforms;
  };
}
