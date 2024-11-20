use assert_cmd::Command;
use serde_json::json;
use std::fs;
use tempfile::TempDir;

const COMPONENT: &str = "com.system76.CosmicComp";
const ENTRY: &str = "autotile";
const VERSION: &str = "1";
const VALUE: &str = "true";

#[test]
fn test_write_command() {
    let temp_dir = TempDir::new().unwrap();
    let config_home = temp_dir.path().to_str().unwrap();

    Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .env("XDG_CONFIG_HOME", config_home)
        .args([
            "write",
            "--version",
            VERSION,
            "--component",
            COMPONENT,
            "--entry",
            ENTRY,
            VALUE,
        ])
        .assert()
        .success()
        .stdout("Configuration entry written successfully.\n");

    let config_path = temp_dir
        .path()
        .join("cosmic")
        .join(COMPONENT)
        .join(format!("v{}", VERSION))
        .join(ENTRY);

    assert!(config_path.exists());
    assert_eq!(fs::read_to_string(config_path).unwrap(), VALUE);
}

#[test]
fn test_read_command() {
    let temp_dir = TempDir::new().unwrap();
    let config_home = temp_dir.path().to_str().unwrap();

    Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .env("XDG_CONFIG_HOME", config_home)
        .args([
            "write",
            "--version",
            VERSION,
            "--component",
            COMPONENT,
            "--entry",
            ENTRY,
            VALUE,
        ])
        .assert()
        .success();

    Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .env("XDG_CONFIG_HOME", config_home)
        .args([
            "read",
            "--version",
            VERSION,
            "--component",
            COMPONENT,
            "--entry",
            ENTRY,
        ])
        .assert()
        .success()
        .stdout(format!("{}\n", VALUE));
}

#[test]
fn test_delete_command() {
    let temp_dir = TempDir::new().unwrap();
    let config_home = temp_dir.path().to_str().unwrap();

    Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .env("XDG_CONFIG_HOME", config_home)
        .args([
            "write",
            "--version",
            VERSION,
            "--component",
            COMPONENT,
            "--entry",
            ENTRY,
            VALUE,
        ])
        .assert()
        .success();

    Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .env("XDG_CONFIG_HOME", config_home)
        .args([
            "delete",
            "--version",
            VERSION,
            "--component",
            COMPONENT,
            "--entry",
            ENTRY,
        ])
        .assert()
        .success()
        .stdout("Configuration entry deleted successfully.\n");

    let config_path = temp_dir
        .path()
        .join("cosmic")
        .join(COMPONENT)
        .join(format!("v{}", VERSION))
        .join(ENTRY);

    assert!(!config_path.exists());
}

#[test]
fn test_apply_command() {
    let temp_dir = TempDir::new().unwrap();
    let config_home = temp_dir.path().to_str().unwrap();

    // Create a temporary JSON config file
    let config_json = json!({
        "$schema": "https://raw.githubusercontent.com/HeitorAugustoLN/cosmic-ctl/refs/heads/main/schema.json",
        "configurations": [
            {
                "component": "com.system76.CosmicComp",
                "version": 1,
                "entries": {
                    "autotile": "true",
                    "autotile_behavior": "PerWorkspace"
                }
            }
        ]
    });

    let config_file = temp_dir.path().join("config.json");
    fs::write(
        &config_file,
        serde_json::to_string_pretty(&config_json).unwrap(),
    )
    .unwrap();

    Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .env("XDG_CONFIG_HOME", config_home)
        .arg("apply")
        .arg(config_file)
        .assert()
        .success()
        .stdout("Configurations applied successfully.\n");

    let autotile_path = temp_dir
        .path()
        .join("cosmic")
        .join("com.system76.CosmicComp")
        .join("v1")
        .join("autotile");
    let autotile_behavior_path = temp_dir
        .path()
        .join("cosmic")
        .join("com.system76.CosmicComp")
        .join("v1")
        .join("autotile_behavior");

    assert!(autotile_path.exists());
    assert!(autotile_behavior_path.exists());
    assert_eq!(fs::read_to_string(autotile_path).unwrap(), "true");
    assert_eq!(
        fs::read_to_string(autotile_behavior_path).unwrap(),
        "PerWorkspace"
    );
}

#[test]
fn test_backup_command() {
    let temp_dir = TempDir::new().unwrap();
    let config_home = temp_dir.path().to_str().unwrap();

    Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .env("XDG_CONFIG_HOME", config_home)
        .args([
            "write",
            "--version",
            VERSION,
            "--component",
            COMPONENT,
            "--entry",
            ENTRY,
            VALUE,
        ])
        .assert()
        .success();

    let backup_file = temp_dir.path().join("backup.json");

    Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .env("XDG_CONFIG_HOME", config_home)
        .arg("backup")
        .arg(&backup_file)
        .assert()
        .success()
        .stdout("Backup completed successfully.\n");

    assert!(backup_file.exists());

    let backup_content: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(backup_file).unwrap()).unwrap();
    assert!(backup_content.get("configurations").is_some());
    assert!(backup_content.get("$schema").is_some());
}
