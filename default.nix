{
  lib,
  rustPlatform,
  versionCheckHook,
}:

rustPlatform.buildRustPackage {
  pname = "cosmic-ctl";
  version = "1.0.0";

  src = builtins.path {
    name = "cosmic-ctl-source";
    path = ./.;
  };

  cargoHash = "sha256-ppCRFlO6cOTEpOoJsaVfMskhJU3b8fM5JWqkTAlT66w=";

  doInstallCheck = true;
  nativeInstallCheckInputs = [ versionCheckHook ];
  versionCheckProgram = "${placeholder "out"}/bin/cosmic-ctl";

  meta = {
    description = "CLI for COSMIC Desktop configuration management";
    homepage = "https://github.com/HeitorAugustoLN/cosmic-ctl";
    license = lib.licenses.gpl3Plus;
    maintainers = with lib.maintainers; [ HeitorAugustoLN ];
    mainProgram = "cosmic-ctl";
  };
}
