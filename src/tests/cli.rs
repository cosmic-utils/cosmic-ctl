use assert_cmd::Command;
use serde_json::json;
use std::fs;
use tempfile::TempDir;

const WRITE_OPERATION: &str = "write";
const READ_OPERATION: &str = "read";
const DELETE_OPERATION: &str = "delete";
const APPLY_OPERATION: &str = "apply";
const BACKUP_OPERATION: &str = "backup";

const COSMIC_COMP: &str = "com.system76.CosmicComp";

const ENTRY_AUTOTILE: &str = "autotile";
const ENTRY_AUTOTILE_BEHAVIOR: &str = "autotile_behavior";
const ENTRY_XKB_CONFIG: &str = "xkb_config";

const VERSION_1: i32 = 1;
const VERSION_2: i32 = 2;

const VALUE_TRUE: &str = "true";
const VALUE_PER_WORKSPACE: &str = "PerWorkspace";
const VALUE_XKB_CONFIG: &str = "(\n    rules: \"\",\n    model: \"\",\n    layout: \"br\",\n    variant: \"\",\n    options: None,\n    repeat_delay: 600,\n    repeat_rate: 25,\n)";

#[test]
fn test_write_command() {
    let temp_dir = TempDir::new().unwrap();
    let config_home = temp_dir.path().to_str().unwrap();

    Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .env("XDG_CONFIG_HOME", config_home)
        .args([
            WRITE_OPERATION,
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
            WRITE_OPERATION,
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
            WRITE_OPERATION,
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
            READ_OPERATION,
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
            WRITE_OPERATION,
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
            DELETE_OPERATION,
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
        "operations": [
            {
                "component": COSMIC_COMP,
                "version": VERSION_1,
                "operation": WRITE_OPERATION,
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
        .arg(APPLY_OPERATION)
        .arg(config_file)
        .assert()
        .success()
        .stdout(
            "Operations completed successfully. 2 writes, 0 reads, 0 deletes, 0 entries skipped.\n",
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
fn test_apply_command_verbose() {
    let temp_dir = TempDir::new().unwrap();
    let config_home = temp_dir.path().to_str().unwrap();

    let config_json = json!({
        "$schema": "https://raw.githubusercontent.com/HeitorAugustoLN/cosmic-ctl/refs/heads/main/schema.json",
        "operations": [
            {
                "component": COSMIC_COMP,
                "version": VERSION_1,
                "operation": WRITE_OPERATION,
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
        .args([APPLY_OPERATION, "--verbose"])
        .arg(&config_file)
        .assert()
        .success()
        .stdout(
            "Operations completed successfully. 2 writes, 0 reads, 0 deletes, 0 entries skipped.\n",
        );

    let output = Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .env("XDG_CONFIG_HOME", config_home)
        .args([APPLY_OPERATION, "--verbose"])
        .arg(&config_file)
        .assert()
        .success();

    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();

    assert!(stdout.contains("Skipping com.system76.CosmicComp/v1/autotile - value unchanged"));
    assert!(
        stdout.contains("Skipping com.system76.CosmicComp/v1/autotile_behavior - value unchanged")
    );
    assert!(stdout.contains(
        "Operations completed successfully. 0 writes, 0 reads, 0 deletes, 2 entries skipped."
    ));

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
            WRITE_OPERATION,
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
        .arg(BACKUP_OPERATION)
        .arg(&backup_file)
        .assert()
        .success()
        .stdout("Backup completed successfully. 1 entries backed up.\n");

    assert!(backup_file.exists());

    let backup_content: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(backup_file).unwrap()).unwrap();
    assert!(backup_content.get("operations").is_some());
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
            WRITE_OPERATION,
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
        .args([BACKUP_OPERATION, "--verbose"])
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
    assert!(backup_content.get("operations").is_some());
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
            WRITE_OPERATION,
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
            WRITE_OPERATION,
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
            WRITE_OPERATION,
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

#[test]
fn test_reset_command_with_exclude() {
    let temp_dir = TempDir::new().unwrap();
    let config_home = temp_dir.path().to_str().unwrap();

    Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .env("XDG_CONFIG_HOME", config_home)
        .args([
            WRITE_OPERATION,
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
            WRITE_OPERATION,
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

    Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .env("XDG_CONFIG_HOME", config_home)
        .args([
            WRITE_OPERATION,
            "--version",
            &VERSION_2.to_string(),
            "--component",
            COSMIC_COMP,
            "--entry",
            ENTRY_AUTOTILE,
            VALUE_TRUE,
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

    let autotile_v2_path = temp_dir
        .path()
        .join("cosmic")
        .join(COSMIC_COMP)
        .join(format!("v{}", VERSION_2))
        .join(ENTRY_AUTOTILE);

    assert!(autotile_path.exists());
    assert!(autotile_behavior_path.exists());
    assert!(autotile_v2_path.exists());

    Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .env("XDG_CONFIG_HOME", config_home)
        .args([
            "reset",
            "--force",
            "--exclude",
            &format!("{}/v{}/{}", COSMIC_COMP, VERSION_1, ENTRY_AUTOTILE),
        ])
        .assert()
        .success();

    assert!(autotile_path.exists());
    assert!(!autotile_behavior_path.exists());
    assert!(!autotile_v2_path.exists());
}

#[test]
fn test_reset_command_with_exclude_entire_component() {
    let temp_dir = TempDir::new().unwrap();
    let config_home = temp_dir.path().to_str().unwrap();

    Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .env("XDG_CONFIG_HOME", config_home)
        .args([
            WRITE_OPERATION,
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
            WRITE_OPERATION,
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

    Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .env("XDG_CONFIG_HOME", config_home)
        .args([
            WRITE_OPERATION,
            "--version",
            &VERSION_2.to_string(),
            "--component",
            COSMIC_COMP,
            "--entry",
            ENTRY_AUTOTILE,
            VALUE_TRUE,
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

    let autotile_v2_path = temp_dir
        .path()
        .join("cosmic")
        .join(COSMIC_COMP)
        .join(format!("v{}", VERSION_2))
        .join(ENTRY_AUTOTILE);

    assert!(autotile_path.exists());
    assert!(autotile_behavior_path.exists());
    assert!(autotile_v2_path.exists());

    Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .env("XDG_CONFIG_HOME", config_home)
        .args(["reset", "--force", "--exclude", &format!("{}", COSMIC_COMP)])
        .assert()
        .success();

    assert!(autotile_path.exists());
    assert!(autotile_behavior_path.exists());
    assert!(autotile_v2_path.exists());
}

#[test]
fn test_reset_command_with_exclude_component_version() {
    let temp_dir = TempDir::new().unwrap();
    let config_home = temp_dir.path().to_str().unwrap();

    Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .env("XDG_CONFIG_HOME", config_home)
        .args([
            WRITE_OPERATION,
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
            WRITE_OPERATION,
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

    Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .env("XDG_CONFIG_HOME", config_home)
        .args([
            WRITE_OPERATION,
            "--version",
            &VERSION_2.to_string(),
            "--component",
            COSMIC_COMP,
            "--entry",
            ENTRY_AUTOTILE,
            VALUE_TRUE,
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

    let autotile_v2_path = temp_dir
        .path()
        .join("cosmic")
        .join(COSMIC_COMP)
        .join(format!("v{}", VERSION_2))
        .join(ENTRY_AUTOTILE);

    assert!(autotile_path.exists());
    assert!(autotile_behavior_path.exists());
    assert!(autotile_v2_path.exists());

    Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .env("XDG_CONFIG_HOME", config_home)
        .args([
            "reset",
            "--force",
            "--exclude",
            &format!("{}/v{}", COSMIC_COMP, VERSION_2),
        ])
        .assert()
        .success();

    assert!(!autotile_path.exists());
    assert!(!autotile_behavior_path.exists());
    assert!(autotile_v2_path.exists());
}

#[test]
fn test_reset_command_with_exclude_brace_expansion() {
    let temp_dir = TempDir::new().unwrap();
    let config_home = temp_dir.path().to_str().unwrap();

    Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .env("XDG_CONFIG_HOME", config_home)
        .args([
            WRITE_OPERATION,
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
            WRITE_OPERATION,
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

    Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .env("XDG_CONFIG_HOME", config_home)
        .args([
            WRITE_OPERATION,
            "--version",
            &VERSION_2.to_string(),
            "--component",
            COSMIC_COMP,
            "--entry",
            ENTRY_AUTOTILE,
            VALUE_TRUE,
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

    let autotile_v2_path = temp_dir
        .path()
        .join("cosmic")
        .join(COSMIC_COMP)
        .join(format!("v{}", VERSION_2))
        .join(ENTRY_AUTOTILE);

    assert!(autotile_path.exists());
    assert!(autotile_behavior_path.exists());
    assert!(autotile_v2_path.exists());

    Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .env("XDG_CONFIG_HOME", config_home)
        .args([
            "reset",
            "--force",
            "--exclude",
            &format!(
                "{}/{{v{}/{{{},{}}},v{}/{}}}",
                COSMIC_COMP,
                VERSION_1,
                ENTRY_AUTOTILE,
                ENTRY_AUTOTILE_BEHAVIOR,
                VERSION_2,
                ENTRY_AUTOTILE
            ),
        ])
        .assert()
        .success();

    assert!(autotile_path.exists());
    assert!(autotile_behavior_path.exists());
    assert!(autotile_v2_path.exists());
}

#[test]
fn test_reset_command_with_exclude_with_wildcard() {
    let temp_dir = TempDir::new().unwrap();
    let config_home = temp_dir.path().to_str().unwrap();

    Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .env("XDG_CONFIG_HOME", config_home)
        .args([
            WRITE_OPERATION,
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
            WRITE_OPERATION,
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

    Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .env("XDG_CONFIG_HOME", config_home)
        .args([
            WRITE_OPERATION,
            "--version",
            &VERSION_2.to_string(),
            "--component",
            COSMIC_COMP,
            "--entry",
            ENTRY_AUTOTILE,
            VALUE_TRUE,
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

    let autotile_v2_path = temp_dir
        .path()
        .join("cosmic")
        .join(COSMIC_COMP)
        .join(format!("v{}", VERSION_2))
        .join(ENTRY_AUTOTILE);

    assert!(autotile_path.exists());
    assert!(autotile_behavior_path.exists());
    assert!(autotile_v2_path.exists());

    Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .env("XDG_CONFIG_HOME", config_home)
        .args([
            "reset",
            "--force",
            "--exclude",
            &format!("{}/v{}/{}*", COSMIC_COMP, VERSION_1, ENTRY_AUTOTILE),
        ])
        .assert()
        .success();

    assert!(autotile_path.exists());
    assert!(autotile_behavior_path.exists());
    assert!(!autotile_v2_path.exists());
}

#[test]
fn test_reset_command_with_exclude_with_brace_expansion_and_wildcard() {
    let temp_dir = TempDir::new().unwrap();
    let config_home = temp_dir.path().to_str().unwrap();

    Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .env("XDG_CONFIG_HOME", config_home)
        .args([
            WRITE_OPERATION,
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
            WRITE_OPERATION,
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

    Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .env("XDG_CONFIG_HOME", config_home)
        .args([
            WRITE_OPERATION,
            "--version",
            &VERSION_2.to_string(),
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
            WRITE_OPERATION,
            "--version",
            &VERSION_1.to_string(),
            "--component",
            COSMIC_COMP,
            "--entry",
            ENTRY_XKB_CONFIG,
            VALUE_XKB_CONFIG,
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

    let autotile_v2_path = temp_dir
        .path()
        .join("cosmic")
        .join(COSMIC_COMP)
        .join(format!("v{}", VERSION_2))
        .join(ENTRY_AUTOTILE);

    let xkb_config_path = temp_dir
        .path()
        .join("cosmic")
        .join(COSMIC_COMP)
        .join(format!("v{}", VERSION_1))
        .join(ENTRY_XKB_CONFIG);

    assert!(autotile_path.exists());
    assert!(autotile_behavior_path.exists());
    assert!(autotile_v2_path.exists());
    assert!(xkb_config_path.exists());

    Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .env("XDG_CONFIG_HOME", config_home)
        .args([
            "reset",
            "--force",
            "--exclude",
            &format!(
                "{}/{{v{}/{}*,v{}/{}*}}",
                COSMIC_COMP, VERSION_1, ENTRY_AUTOTILE, VERSION_2, ENTRY_AUTOTILE
            ),
        ])
        .assert()
        .success();

    assert!(autotile_path.exists());
    assert!(autotile_behavior_path.exists());
    assert!(autotile_v2_path.exists());
    assert!(!xkb_config_path.exists())
}
