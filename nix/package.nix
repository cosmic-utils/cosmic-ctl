{
  lib,
  rustPlatform,
  versionCheckHook,
}:
rustPlatform.buildRustPackage (finalAttrs: {
  pname = "cosmic-ext-ctl";
  version = "1.5.0";

  src = lib.fileset.toSource {
    root = ./..;
    fileset = lib.fileset.gitTracked ./..;
  };

  cargoLock = {
    lockFile = ../Cargo.lock;
    outputHashes = {
      "atomicwrites-0.4.2" = "sha256-QZSuGPrJXh+svMeFWqAXoqZQxLq/WfIiamqvjJNVhxA=";
      "clipboard_macos-0.1.0" = "sha256-+8CGmBf1Gl9gnBDtuKtkzUE5rySebhH7Bsq/kNlJofY=";
      "cosmic-client-toolkit-0.1.0" = "sha256-7EFXDQ6aHiXq0qrjeyjqtOuC3B5JLpHQTXbPwtC+fRo=";
      "cosmic-config-0.1.0" = "sha256-BLzfRvHUISkQADq6OVeJDdYZJrBWFVvGCo7IR30uEeI=";
      "cosmic-freedesktop-icons-0.3.0" = "sha256-XAcoKxMp1fyclalkkqVMoO7+TVekj/Tq2C9XFM9FFCk=";
      "cosmic-text-0.14.2" = "sha256-Wt5ejab5EkuyGiAd9DZ1Sc8IMxDq29lwAvKnFcbhX5o=";
      "iced_glyphon-0.6.0" = "sha256-u1vnsOjP8npQ57NNSikotuHxpi4Mp/rV9038vAgCsfQ=";
      "smithay-clipboard-0.8.0" = "sha256-4InFXm0ahrqFrtNLeqIuE3yeOpxKZJZx+Bc0yQDtv34=";
      "softbuffer-0.4.1" = "sha256-a0bUFz6O8CWRweNt/OxTvflnPYwO5nm6vsyc/WcXyNg=";
      "taffy-0.3.11" = "sha256-SCx9GEIJjWdoNVyq+RZAGn0N71qraKZxf9ZWhvyzLaI=";
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
