{
  lib,
  rustPlatform,
  versionCheckHook,
}:

rustPlatform.buildRustPackage (finalAttrs: {
  pname = "cosmic-ext-ctl";
  version =
    let
      cargoData = lib.importTOML (finalAttrs.src + "/Cargo.toml");
    in
    cargoData.package.version;

  src = lib.fileset.toSource {
    root = ./..;
    fileset = lib.fileset.gitTracked ./..;
  };

  cargoDeps = rustPlatform.importCargoLock {
    lockFile = finalAttrs.src + "/Cargo.lock";
    allowBuiltinFetchGit = true;
  };

  doInstallCheck = true;
  nativeInstallCheckInputs = [ versionCheckHook ];
  versionCheckProgram = "${placeholder "out"}/bin/cosmic-ctl";

  meta = {
    changelog = "https://github.com/cosmic-utils/cosmic-ctl/releases/tag/v${finalAttrs.version}";
    description = "CLI for COSMIC Desktop configuration management";
    homepage = "https://github.com/cosmic-utils/cosmic-ctl";
    license = lib.licenses.gpl3Only;
    mainProgram = "cosmic-ctl";
    maintainers = with lib.maintainers; [ HeitorAugustoLN ];
  };
})
