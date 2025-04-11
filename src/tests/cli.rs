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

const XDG_CONFIG_DIR: &str = "config";
const XDG_STATE_DIR: &str = "state";

const VERSION_1: u64 = 1;
const VERSION_2: u64 = 2;

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
        .stdout("Doing nothing. Configuration entry already has the same value.\n");

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
        "$schema": "https://raw.githubusercontent.com/cosmic-utils/cosmic-ctl/refs/heads/main/schema.json",
        "operations": [
            {
                "component": COSMIC_COMP,
                "version": VERSION_1,
                "operation": WRITE_OPERATION,
                "xdg_directory": XDG_CONFIG_DIR,
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
        "$schema": "https://raw.githubusercontent.com/cosmic-utils/cosmic-ctl/refs/heads/main/schema.json",
        "operations": [
            {
                "component": COSMIC_COMP,
                "version": VERSION_1,
                "operation": WRITE_OPERATION,
                "xdg_directory": XDG_CONFIG_DIR,
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
            "Using JSON format for input file\nOperations completed successfully. 2 writes, 0 reads, 0 deletes, 0 entries skipped.\n",
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
        .stdout("Backup completed successfully. 1 total entries backed up in JSON format.\n");

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
            "Using JSON format for output file\nBacking up [{}]: {}/v{}/{}\nCompleted backup for {} directory: 1 entries\nCompleted backup for {} directory: 0 entries\nBackup completed successfully. 1 total entries backed up in JSON format.\n",
            XDG_CONFIG_DIR, COSMIC_COMP, VERSION_1, ENTRY_AUTOTILE, XDG_CONFIG_DIR, XDG_STATE_DIR
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
    let temp_dir2 = TempDir::new().unwrap();
    let state_home = temp_dir2.path().to_str().unwrap();

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
        .env("XDG_STATE_HOME", state_home)
        .args(["reset", "--force", "--verbose"])
        .assert()
        .success()
        .stdout(format!(
            "Deleting [{}]: {}\nCompleted reset for {} directory: 1 entries deleted\nNo configuration entries found in {}.\nSuccessfully deleted 1 configuration entries.\n",
            XDG_CONFIG_DIR,
            config_path.display(),
            XDG_CONFIG_DIR,
            XDG_STATE_DIR
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
        .stdout("Successfully deleted 0 configuration entries.\n");
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

#[test]
fn test_apply_command_toml() {
    let temp_dir = TempDir::new().unwrap();
    let config_home = temp_dir.path().to_str().unwrap();

    // Create TOML configuration
    let mut entries = toml::Table::new();
    entries.insert(
        ENTRY_AUTOTILE.to_string(),
        toml::Value::String(VALUE_TRUE.to_string()),
    );
    entries.insert(
        ENTRY_AUTOTILE_BEHAVIOR.to_string(),
        toml::Value::String(VALUE_PER_WORKSPACE.to_string()),
    );

    let mut operation = toml::Table::new();
    operation.insert(
        "component".to_string(),
        toml::Value::String(COSMIC_COMP.to_string()),
    );
    operation.insert(
        "version".to_string(),
        toml::Value::Integer(VERSION_1 as i64),
    );
    operation.insert(
        "operation".to_string(),
        toml::Value::String(WRITE_OPERATION.to_string()),
    );
    operation.insert(
        "xdg_directory".to_string(),
        toml::Value::String(XDG_CONFIG_DIR.to_string()),
    );
    operation.insert("entries".to_string(), toml::Value::Table(entries));

    let mut operations = Vec::new();
    operations.push(toml::Value::Table(operation));

    let mut root = toml::Table::new();
    root.insert(
        "$schema".to_string(),
        toml::Value::String(
            "https://raw.githubusercontent.com/cosmic-utils/cosmic-ctl/refs/heads/main/schema.json"
                .to_string(),
        ),
    );
    root.insert("operations".to_string(), toml::Value::Array(operations));

    let config_file = temp_dir.path().join("config.toml");
    fs::write(&config_file, toml::to_string_pretty(&root).unwrap()).unwrap();

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
fn test_apply_command_ron() {
    let temp_dir = TempDir::new().unwrap();
    let config_home = temp_dir.path().to_str().unwrap();

    use crate::schema::{ConfigFile, Entry, EntryContent, Operation};
    use std::collections::HashMap;

    let mut entries = HashMap::new();
    entries.insert(ENTRY_AUTOTILE.to_string(), VALUE_TRUE.to_string());
    entries.insert(
        ENTRY_AUTOTILE_BEHAVIOR.to_string(),
        VALUE_PER_WORKSPACE.to_string(),
    );

    let config = ConfigFile {
        schema: None,
        operations: vec![Entry {
            component: Some(COSMIC_COMP.to_string()),
            file: None,
            value: None,
            version: Some(VERSION_1),
            operation: Operation::Write,
            xdg_directory: Some(XDG_CONFIG_DIR.to_string()),
            entries: Some(EntryContent::WriteEntries(entries)),
        }],
    };

    let ron_config =
        ron::ser::to_string_pretty(&config, ron::ser::PrettyConfig::default()).unwrap();

    let config_file = temp_dir.path().join("config.ron");
    fs::write(&config_file, ron_config).unwrap();

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
fn test_backup_command_toml() {
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

    let backup_file = temp_dir.path().join("backup.toml");

    Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .env("XDG_CONFIG_HOME", config_home)
        .arg(BACKUP_OPERATION)
        .arg(&backup_file)
        .assert()
        .success()
        .stdout("Backup completed successfully. 1 total entries backed up in TOML format.\n");

    assert!(backup_file.exists());

    let backup_content = fs::read_to_string(&backup_file).unwrap();
    let toml_data: toml::Table = toml::from_str(&backup_content).unwrap();

    assert!(toml_data.get("operations").is_some());
    assert!(toml_data.get("$schema").is_some());
}

#[test]
fn test_backup_command_ron() {
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

    let backup_file = temp_dir.path().join("backup.ron");

    Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .env("XDG_CONFIG_HOME", config_home)
        .arg(BACKUP_OPERATION)
        .arg(&backup_file)
        .assert()
        .success()
        .stdout("Backup completed successfully. 1 total entries backed up in RON format.\n");

    assert!(backup_file.exists());

    let backup_content = fs::read_to_string(&backup_file).unwrap();

    use crate::schema::ConfigFile;
    let config: ConfigFile = ron::from_str(&backup_content).unwrap();

    assert!(!config.operations.is_empty());
}

#[test]
fn test_backup_command_with_explicit_format() {
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

    let backup_file = temp_dir.path().join("backup.txt");

    Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .env("XDG_CONFIG_HOME", config_home)
        .args([BACKUP_OPERATION, "--format", "json"])
        .arg(&backup_file)
        .assert()
        .success()
        .stdout("Backup completed successfully. 1 total entries backed up in JSON format.\n");

    assert!(backup_file.exists());

    let backup_content = fs::read_to_string(&backup_file).unwrap();
    let json_data: serde_json::Value = serde_json::from_str(&backup_content).unwrap();

    assert!(json_data.get("operations").is_some());
    assert!(json_data.get("$schema").is_some());
}

#[test]
fn test_file_write_command() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test-config.conf");
    let file_path_str = file_path.to_str().unwrap();

    Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .args([
            WRITE_OPERATION,
            "--file",
            file_path_str,
            "Example configuration content",
        ])
        .assert()
        .success()
        .stdout("Configuration file written successfully.\n");

    assert!(file_path.exists());
    assert_eq!(
        fs::read_to_string(&file_path).unwrap(),
        "Example configuration content"
    );

    Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .args([
            WRITE_OPERATION,
            "--file",
            file_path_str,
            "Example configuration content",
        ])
        .assert()
        .success()
        .stdout("Doing nothing. Configuration file already has the same value.\n");
}

#[test]
fn test_file_read_command() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test-config.conf");
    let file_path_str = file_path.to_str().unwrap();
    let content = "Example configuration content";

    fs::write(&file_path, content).unwrap();

    Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .args([READ_OPERATION, "--file", file_path_str])
        .assert()
        .success()
        .stdout(format!("{}\n", content));
}

#[test]
fn test_file_delete_command() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test-config.conf");
    let file_path_str = file_path.to_str().unwrap();

    fs::write(&file_path, "Example configuration content").unwrap();
    assert!(file_path.exists());

    Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .args([DELETE_OPERATION, "--file", file_path_str])
        .assert()
        .success()
        .stdout("Configuration file deleted successfully.\n");

    assert!(!file_path.exists());

    Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .args([DELETE_OPERATION, "--file", file_path_str])
        .assert()
        .failure();
}

#[test]
fn test_apply_command_with_file_operations_json() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test-config.conf");
    let test_file_str = test_file.to_str().unwrap();

    let config_json = json!({
        "operations": [
            {
                "file": test_file_str,
                "operation": WRITE_OPERATION,
                "value": "This is a direct file write test"
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
        .arg(APPLY_OPERATION)
        .arg(&config_file)
        .assert()
        .success()
        .stdout(
            "Operations completed successfully. 1 writes, 0 reads, 0 deletes, 0 entries skipped.\n",
        );

    assert!(test_file.exists());
    assert_eq!(
        fs::read_to_string(&test_file).unwrap(),
        "This is a direct file write test"
    );
}

#[test]
fn test_apply_command_with_file_operations_toml() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test-config.conf");
    let read_file = temp_dir.path().join("read-me.conf");
    let delete_file = temp_dir.path().join("delete-me.conf");

    // Create files for read and delete operations
    fs::write(&read_file, "Content to be read").unwrap();
    fs::write(&delete_file, "Content to be deleted").unwrap();

    // Create TOML configuration with file operations
    let mut operation1 = toml::Table::new();
    operation1.insert(
        "file".to_string(),
        toml::Value::String(test_file.to_str().unwrap().to_string()),
    );
    operation1.insert(
        "operation".to_string(),
        toml::Value::String(WRITE_OPERATION.to_string()),
    );
    operation1.insert(
        "value".to_string(),
        toml::Value::String("This is a direct TOML file write".to_string()),
    );

    let mut operation2 = toml::Table::new();
    operation2.insert(
        "file".to_string(),
        toml::Value::String(read_file.to_str().unwrap().to_string()),
    );
    operation2.insert(
        "operation".to_string(),
        toml::Value::String(READ_OPERATION.to_string()),
    );

    let mut operation3 = toml::Table::new();
    operation3.insert(
        "file".to_string(),
        toml::Value::String(delete_file.to_str().unwrap().to_string()),
    );
    operation3.insert(
        "operation".to_string(),
        toml::Value::String(DELETE_OPERATION.to_string()),
    );

    let mut operations = Vec::new();
    operations.push(toml::Value::Table(operation1));
    operations.push(toml::Value::Table(operation2));
    operations.push(toml::Value::Table(operation3));

    let mut root = toml::Table::new();
    root.insert("operations".to_string(), toml::Value::Array(operations));

    let config_file = temp_dir.path().join("config.toml");
    fs::write(&config_file, toml::to_string_pretty(&root).unwrap()).unwrap();

    let output = Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .arg(APPLY_OPERATION)
        .arg("--verbose")
        .arg(&config_file)
        .assert()
        .success();

    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();

    assert!(stdout.contains("Content to be read"));
    assert!(stdout.contains("Deleted:"));
    assert!(stdout.contains("Operations completed successfully. 1 writes, 1 reads, 1 deletes"));

    assert!(test_file.exists());
    assert!(!delete_file.exists());
    assert_eq!(
        fs::read_to_string(&test_file).unwrap(),
        "This is a direct TOML file write"
    );
}

#[test]
fn test_apply_command_with_file_operations_ron() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test-config.conf");
    let test_file_str = test_file.to_str().unwrap();

    let ron_config = format!(
        r#"(
            operations: [
                (
                    file: "{test_file_str}",
                    operation: "write",
                    value: "This is a direct RON file write"
                )
            ]
        )"#,
        test_file_str = test_file_str.replace("\\", "\\\\")
    );

    let config_file = temp_dir.path().join("config.ron");
    fs::write(&config_file, ron_config).unwrap();

    Command::cargo_bin("cosmic-ctl")
        .unwrap()
        .arg(APPLY_OPERATION)
        .arg(&config_file)
        .assert()
        .success()
        .stdout(
            "Operations completed successfully. 1 writes, 0 reads, 0 deletes, 0 entries skipped.\n",
        );

    assert!(test_file.exists());
    assert_eq!(
        fs::read_to_string(&test_file).unwrap(),
        "This is a direct RON file write"
    );
}

#[test]
fn test_apply_command_mixed_operations() {
    let temp_dir = TempDir::new().unwrap();
    let config_home = temp_dir.path().to_str().unwrap();
    let test_file = temp_dir.path().join("test-config.conf");
    let test_file_str = test_file.to_str().unwrap();

    let config_json = json!({
        "operations": [
            {
                "component": COSMIC_COMP,
                "version": VERSION_1,
                "operation": WRITE_OPERATION,
                "xdg_directory": XDG_CONFIG_DIR,
                "entries": {
                    ENTRY_AUTOTILE: VALUE_TRUE
                }
            },
            {
                "file": test_file_str,
                "operation": WRITE_OPERATION,
                "value": "Direct file write example"
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
        .arg(&config_file)
        .assert()
        .success()
        .stdout(
            "Operations completed successfully. 2 writes, 0 reads, 0 deletes, 0 entries skipped.\n",
        );

    let config_path = temp_dir
        .path()
        .join("cosmic")
        .join(COSMIC_COMP)
        .join(format!("v{}", VERSION_1))
        .join(ENTRY_AUTOTILE);

    assert!(config_path.exists());
    assert_eq!(fs::read_to_string(config_path).unwrap(), VALUE_TRUE);

    assert!(test_file.exists());
    assert_eq!(
        fs::read_to_string(&test_file).unwrap(),
        "Direct file write example"
    );
}
