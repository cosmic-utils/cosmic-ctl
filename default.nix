{
  pkgs ? import <nixpkgs> { },
  ...
}:
pkgs.callPackage (
  {
    lib,
    rustPlatform,
    versionCheckHook,
    cosmic-comp,
  }:
  let
    version = "1.1.0";
  in
  rustPlatform.buildRustPackage {
    pname = "cosmic-ctl";
    inherit version;

    src = builtins.path {
      name = "cosmic-ctl-source";
      path = ./.;
    };

    useFetchCargoVendor = true;
    cargoHash = "sha256-EReo2hkBaIO1YOBx4D9rQSXlx+3NK5VQtj59jfZZI/0=";

    doInstallCheck = true;
    nativeInstallCheckInputs = [ versionCheckHook ];
    versionCheckProgram = "${placeholder "out"}/bin/cosmic-ctl";

    meta = {
      description = "CLI for COSMIC Desktop configuration management";
      changelog = "https://github.com/cosmic-utils/cosmic-ctl/releases/tag/v${version}";
      homepage = "https://github.com/cosmic-utils/cosmic-ctl";
      license = lib.licenses.gpl3Only;
      maintainers = with lib.maintainers; [ HeitorAugustoLN ];
      mainProgram = "cosmic-ctl";
      inherit (cosmic-comp.meta) platforms;
    };
  }
) { }
