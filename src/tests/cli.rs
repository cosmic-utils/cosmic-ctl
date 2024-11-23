use assert_cmd::Command;
use serde_json::json;
use std::fs;
use tempfile::TempDir;

const COSMIC_COMP: &str = "com.system76.CosmicComp";

const ENTRY_AUTOTILE: &str = "autotile";
const ENTRY_AUTOTILE_BEHAVIOR: &str = "autotile_behavior";

const VERSION_1: i32 = 1;
const VERSION_2: i32 = 2;

const VALUE_TRUE: &str = "true";
const VALUE_PER_WORKSPACE: &str = "PerWorkspace";

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
            &VERSION_1.to_string(),
            "--component",
            COSMIC_COMP,
            "--entry",
            ENTRY_AUTOTILE,
            VALUE_TRUE,
        ])
        .assert()
        .success()
        .stdout("Configuration entry written successfully.\n");

    Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .env("XDG_CONFIG_HOME", config_home)
        .args([
            "write",
            "--version",
            &VERSION_1.to_string(),
            "--component",
            COSMIC_COMP,
            "--entry",
            ENTRY_AUTOTILE,
            VALUE_TRUE,
        ])
        .assert()
        .success()
        .stdout("Doing nothing, entry already has this value.\n");

    let config_path = temp_dir
        .path()
        .join("cosmic")
        .join(COSMIC_COMP)
        .join(format!("v{}", VERSION_1))
        .join(ENTRY_AUTOTILE);

    assert!(config_path.exists());
    assert_eq!(fs::read_to_string(config_path).unwrap(), VALUE_TRUE);
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
            &VERSION_1.to_string(),
            "--component",
            COSMIC_COMP,
            "--entry",
            ENTRY_AUTOTILE,
            VALUE_TRUE,
        ])
        .assert()
        .success();

    Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .env("XDG_CONFIG_HOME", config_home)
        .args([
            "read",
            "--version",
            &VERSION_1.to_string(),
            "--component",
            COSMIC_COMP,
            "--entry",
            ENTRY_AUTOTILE,
        ])
        .assert()
        .success()
        .stdout(format!("{}\n", VALUE_TRUE));
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
            &VERSION_1.to_string(),
            "--component",
            COSMIC_COMP,
            "--entry",
            ENTRY_AUTOTILE,
            VALUE_TRUE,
        ])
        .assert()
        .success();

    Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .env("XDG_CONFIG_HOME", config_home)
        .args([
            "delete",
            "--version",
            &VERSION_1.to_string(),
            "--component",
            COSMIC_COMP,
            "--entry",
            ENTRY_AUTOTILE,
        ])
        .assert()
        .success()
        .stdout("Configuration entry deleted successfully.\n");

    let config_path = temp_dir
        .path()
        .join("cosmic")
        .join(COSMIC_COMP)
        .join(format!("v{}", VERSION_1))
        .join(ENTRY_AUTOTILE);

    assert!(!config_path.exists());
}

#[test]
fn test_apply_command() {
    let temp_dir = TempDir::new().unwrap();
    let config_home = temp_dir.path().to_str().unwrap();

    let config_json = json!({
        "$schema": "https://raw.githubusercontent.com/HeitorAugustoLN/cosmic-ctl/refs/heads/main/schema.json",
        "configurations": [
            {
                "component": COSMIC_COMP,
                "version": VERSION_1,
                "entries": {
                    ENTRY_AUTOTILE: VALUE_TRUE,
                    ENTRY_AUTOTILE_BEHAVIOR: VALUE_PER_WORKSPACE
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
        .stdout("Configurations applied successfully. 2 changes made, 0 entries skipped.\n");

    let autotile_path = temp_dir
        .path()
        .join("cosmic")
        .join(COSMIC_COMP)
        .join(format!("v{}", VERSION_1))
        .join(ENTRY_AUTOTILE);
    let autotile_behavior_path = temp_dir
        .path()
        .join("cosmic")
        .join(COSMIC_COMP)
        .join(format!("v{}", VERSION_1))
        .join(ENTRY_AUTOTILE_BEHAVIOR);

    assert!(autotile_path.exists());
    assert!(autotile_behavior_path.exists());
    assert_eq!(fs::read_to_string(autotile_path).unwrap(), VALUE_TRUE);
    assert_eq!(
        fs::read_to_string(autotile_behavior_path).unwrap(),
        VALUE_PER_WORKSPACE
    );
}

#[test]
fn test_apply_command_verbose() {
    let temp_dir = TempDir::new().unwrap();
    let config_home = temp_dir.path().to_str().unwrap();

    let config_json = json!({
        "$schema": "https://raw.githubusercontent.com/HeitorAugustoLN/cosmic-ctl/refs/heads/main/schema.json",
        "configurations": [
            {
                "component": COSMIC_COMP,
                "version": VERSION_1,
                "entries": {
                    ENTRY_AUTOTILE: VALUE_TRUE,
                    ENTRY_AUTOTILE_BEHAVIOR: VALUE_PER_WORKSPACE
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
        .args(["apply", "--verbose"])
        .arg(&config_file)
        .assert()
        .success()
        .stdout("Configurations applied successfully. 2 changes made, 0 entries skipped.\n");

    let output = Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .env("XDG_CONFIG_HOME", config_home)
        .args(["apply", "--verbose"])
        .arg(&config_file)
        .assert()
        .success();

    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();

    assert!(stdout.contains("Skipping com.system76.CosmicComp/v1/autotile - value unchanged"));
    assert!(
        stdout.contains("Skipping com.system76.CosmicComp/v1/autotile_behavior - value unchanged")
    );
    assert!(
        stdout.contains("Configurations applied successfully. 0 changes made, 2 entries skipped.")
    );

    let autotile_path = temp_dir
        .path()
        .join("cosmic")
        .join(COSMIC_COMP)
        .join(format!("v{}", VERSION_1))
        .join(ENTRY_AUTOTILE);
    let autotile_behavior_path = temp_dir
        .path()
        .join("cosmic")
        .join(COSMIC_COMP)
        .join(format!("v{}", VERSION_1))
        .join(ENTRY_AUTOTILE_BEHAVIOR);

    assert!(autotile_path.exists());
    assert!(autotile_behavior_path.exists());
    assert_eq!(fs::read_to_string(autotile_path).unwrap(), VALUE_TRUE);
    assert_eq!(
        fs::read_to_string(autotile_behavior_path).unwrap(),
        VALUE_PER_WORKSPACE
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
            &VERSION_1.to_string(),
            "--component",
            COSMIC_COMP,
            "--entry",
            ENTRY_AUTOTILE,
            VALUE_TRUE,
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
        .stdout("Backup completed successfully. 1 entries backed up.\n");

    assert!(backup_file.exists());

    let backup_content: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(backup_file).unwrap()).unwrap();
    assert!(backup_content.get("configurations").is_some());
    assert!(backup_content.get("$schema").is_some());
}

#[test]
fn test_backup_command_verbose() {
    let temp_dir = TempDir::new().unwrap();
    let config_home = temp_dir.path().to_str().unwrap();

    Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .env("XDG_CONFIG_HOME", config_home)
        .args([
            "write",
            "--version",
            &VERSION_1.to_string(),
            "--component",
            COSMIC_COMP,
            "--entry",
            ENTRY_AUTOTILE,
            VALUE_TRUE,
        ])
        .assert()
        .success();

    let backup_file = temp_dir.path().join("backup.json");

    Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .env("XDG_CONFIG_HOME", config_home)
        .args(["backup", "--verbose"])
        .arg(&backup_file)
        .assert()
        .success()
        .stdout(format!(
            "Backing up: {}/v{}/{}\nBackup completed successfully. 1 entries backed up.\n",
            COSMIC_COMP, VERSION_1, ENTRY_AUTOTILE
        ));

    assert!(backup_file.exists());

    let backup_content: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(backup_file).unwrap()).unwrap();
    assert!(backup_content.get("configurations").is_some());
    assert!(backup_content.get("$schema").is_some());
}

#[test]
fn test_reset_command() {
    let temp_dir = TempDir::new().unwrap();
    let config_home = temp_dir.path().to_str().unwrap();

    Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .env("XDG_CONFIG_HOME", config_home)
        .args([
            "write",
            "--version",
            &VERSION_1.to_string(),
            "--component",
            COSMIC_COMP,
            "--entry",
            ENTRY_AUTOTILE,
            VALUE_TRUE,
        ])
        .assert()
        .success();

    Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .env("XDG_CONFIG_HOME", config_home)
        .args([
            "write",
            "--version",
            &VERSION_1.to_string(),
            "--component",
            COSMIC_COMP,
            "--entry",
            ENTRY_AUTOTILE_BEHAVIOR,
            VALUE_PER_WORKSPACE,
        ])
        .assert()
        .success();

    let autotile_path = temp_dir
        .path()
        .join("cosmic")
        .join(COSMIC_COMP)
        .join(format!("v{}", VERSION_1))
        .join(ENTRY_AUTOTILE);
    let autotile_behavior_path = temp_dir
        .path()
        .join("cosmic")
        .join(COSMIC_COMP)
        .join(format!("v{}", VERSION_1))
        .join(ENTRY_AUTOTILE_BEHAVIOR);

    assert!(autotile_path.exists());
    assert!(autotile_behavior_path.exists());

    Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .env("XDG_CONFIG_HOME", config_home)
        .args(["reset", "--force"])
        .assert()
        .success()
        .stdout("Successfully deleted 2 configuration entries.\n");

    assert!(!autotile_path.exists());
    assert!(!autotile_behavior_path.exists());
    assert!(autotile_path.parent().unwrap().exists());
    assert!(autotile_behavior_path.parent().unwrap().exists())
}

#[test]
fn test_reset_command_verbose() {
    let temp_dir = TempDir::new().unwrap();
    let config_home = temp_dir.path().to_str().unwrap();

    Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .env("XDG_CONFIG_HOME", config_home)
        .args([
            "write",
            "--version",
            &VERSION_1.to_string(),
            "--component",
            COSMIC_COMP,
            "--entry",
            ENTRY_AUTOTILE,
            VALUE_TRUE,
        ])
        .assert()
        .success();

    let config_path = temp_dir
        .path()
        .join("cosmic")
        .join(COSMIC_COMP)
        .join(format!("v{}", VERSION_1))
        .join(ENTRY_AUTOTILE);

    assert!(config_path.exists());

    Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .env("XDG_CONFIG_HOME", config_home)
        .args(["reset", "--force", "--verbose"])
        .assert()
        .success()
        .stdout(format!(
            "Deleting: {}\nSuccessfully deleted 1 configuration entries.\n",
            config_path.display()
        ));

    assert!(!config_path.exists());
    assert!(config_path.parent().unwrap().exists());
}

#[test]
fn test_reset_command_empty_config() {
    let temp_dir = TempDir::new().unwrap();
    let config_home = temp_dir.path().to_str().unwrap();

    Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .env("XDG_CONFIG_HOME", config_home)
        .args(["reset", "--force"])
        .assert()
        .success()
        .stdout("No configurations to delete.\n");
}
