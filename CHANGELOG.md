# Changelog

## [1.4.0](https://github.com/cosmic-utils/cosmic-ctl/compare/v1.3.0...v1.4.0) (2025-04-12)


### Features

* **cli:** add interactive mode using inquire ([#9](https://github.com/cosmic-utils/cosmic-ctl/issues/9)) ([f1602db](https://github.com/cosmic-utils/cosmic-ctl/commit/f1602db8ea5dcca78ef719577a17c44f3aeb9eae))

## [1.3.0](https://github.com/cosmic-utils/cosmic-ctl/compare/v1.2.0...v1.3.0) (2025-04-11)


### Features

* add direct file operations to configuration system ([#6](https://github.com/cosmic-utils/cosmic-ctl/issues/6)) ([32f873a](https://github.com/cosmic-utils/cosmic-ctl/commit/32f873a58d127859fe623069198eb47285405d3c))
* remove YAML support ([#8](https://github.com/cosmic-utils/cosmic-ctl/issues/8)) ([490ce19](https://github.com/cosmic-utils/cosmic-ctl/commit/490ce1931051304cbb573b5e49ac03aa42c630ea))

## [1.2.0](https://github.com/cosmic-utils/cosmic-ctl/compare/v1.1.0...v1.2.0) (2025-04-11)


### Features

* **commands:** add support for YAML, TOML and RON formats ([#4](https://github.com/cosmic-utils/cosmic-ctl/issues/4)) ([5b746ff](https://github.com/cosmic-utils/cosmic-ctl/commit/5b746ff387c4df6fa88684179f6a5b8e6c8e63c7))

## [1.1.0](https://github.com/cosmic-utils/cosmic-ctl/compare/v1.0.0...v1.1.0) (2024-12-29)


### Features

* **config:** use atomic writes for safer configuration writing ([12e48f4](https://github.com/cosmic-utils/cosmic-ctl/commit/12e48f412e420b2e59cf97684c4088c61fc18e8e))

## 1.0.0 (2024-12-16)


### ⚠ BREAKING CHANGES

* Restructured CLI architecture to use separate command modules
* unify configuration operations under new schema

### Features

* add `reset` command to delete all configuration entries ([c910e14](https://github.com/cosmic-utils/cosmic-ctl/commit/c910e1405d15aa292910d5e6d639076cb6bae330))
* add backup functionality for configurations ([0d93278](https://github.com/cosmic-utils/cosmic-ctl/commit/0d9327843eadb21ece4c1baf01469dfee5172055))
* add basic command support ([eb215a7](https://github.com/cosmic-utils/cosmic-ctl/commit/eb215a76d87fa887c4640ac0f4c08fb350d091e0))
* add list command to display configuration entries ([0305672](https://github.com/cosmic-utils/cosmic-ctl/commit/0305672da4924cdaa93f9639809b3d87d53c4b3b))
* add pattern-based exclusion to reset command ([a104bf1](https://github.com/cosmic-utils/cosmic-ctl/commit/a104bf1c41adce7f867aaedb3c267d867c219ebf))
* add support for applying configurations from JSON files ([c937b23](https://github.com/cosmic-utils/cosmic-ctl/commit/c937b2395b7fcc157c50d8c5d12d9f1b963afb11))
* add support for escape characters and improve descriptions ([b5030c2](https://github.com/cosmic-utils/cosmic-ctl/commit/b5030c235dd6d1e373390a67d2f1d1b506476dbb))
* add support for specifying XDG directories in configuration commands ([4297c69](https://github.com/cosmic-utils/cosmic-ctl/commit/4297c694ab9ae1b475e60353f5b24178901ed0f8))
* add verbose output for backup and add more apply tests ([65854b6](https://github.com/cosmic-utils/cosmic-ctl/commit/65854b6229b69e425bfb167b6b8f6ac2d8fbc083))
* **cli:** skip unchanged configuration writes and improve apply feedback ([c951ba7](https://github.com/cosmic-utils/cosmic-ctl/commit/c951ba79614c08175df7e8b8e7c786fe1ea01cf3))
* **config:** use etcetera for XDG path handling ([caf2daa](https://github.com/cosmic-utils/cosmic-ctl/commit/caf2daa975e2dea0a415559a988cab1167d873e9))
* implement read, write and delete operations ([30c7803](https://github.com/cosmic-utils/cosmic-ctl/commit/30c7803008421c2df5c27c22cb316ca27890a739))
* include JSON schema reference in backups ([5f8ec38](https://github.com/cosmic-utils/cosmic-ctl/commit/5f8ec38a275955eda02ab2266239e74a55be5099))


### Bug Fixes

* change default XDG directories from 'data' to 'state' in backup and reset commands ([604f65f](https://github.com/cosmic-utils/cosmic-ctl/commit/604f65f5000f14cdad520889927e9288220cfcf8))
* do debug build in cli testing CI ([c711afa](https://github.com/cosmic-utils/cosmic-ctl/commit/c711afa3ddff72c4eb4c88b1d2117400863fde82))
* **example:** correct actual value for `active_page` ([a7f6766](https://github.com/cosmic-utils/cosmic-ctl/commit/a7f6766d6753094106032653bd81de297afd4f28))
* **nix:** update cargo hash ([2d0aca0](https://github.com/cosmic-utils/cosmic-ctl/commit/2d0aca0c4d01af17881f3d681f13eeb6bdd8a593))
* restrict configuration path to the COSMIC directory ([c7295eb](https://github.com/cosmic-utils/cosmic-ctl/commit/c7295eb708cca8bb1bb51e72468d8af3b3c906db))
* **tests:** update reset command empty config test to match new output ([8e487ab](https://github.com/cosmic-utils/cosmic-ctl/commit/8e487ab1899b0a401aaab60edeac89945d6e49f8))


### Code Refactoring

* modularize command implementation ([b4f05da](https://github.com/cosmic-utils/cosmic-ctl/commit/b4f05dab739a1c680539e0c6cdd6f6718c84be18))
* unify configuration operations under new schema ([44065a8](https://github.com/cosmic-utils/cosmic-ctl/commit/44065a88a2f70c69fc3e16234eb18ba2e25ddfde))
