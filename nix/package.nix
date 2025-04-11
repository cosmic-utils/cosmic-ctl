{
  lib,
  rustPlatform,
  versionCheckHook,
}:
rustPlatform.buildRustPackage (finalAttrs: {
  pname = "cosmic-ext-ctl";
  version = "1.1.0";

  src = lib.fileset.toSource {
    root = ./..;
    fileset = lib.fileset.gitTracked ./..;
  };

  cargoLock = {
    lockFile = ../Cargo.lock;
    outputHashes = {
      "atomicwrites-0.4.2" = "sha256-QZSuGPrJXh+svMeFWqAXoqZQxLq/WfIiamqvjJNVhxA=";
    };
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
