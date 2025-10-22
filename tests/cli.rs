use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::process::Command;

#[test]
fn test_create_static_htmx() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = assert_fs::TempDir::new()?;
    let project_name = "my-test-app";
    let project_path = temp_dir.child(project_name);

    // Run `forge-cli new my-test-app --frontend htmx`
    Command::cargo_bin("forge-cli")?
        .arg("new")
        .arg(project_path.path())
        .arg("--frontend")
        .arg("htmx")
        //.current_dir(temp_dir.path())
        .assert()
        .success();

    // Directory exists
    project_path.assert(predicate::path::exists());

    // Files exist (ChildPath supports .assert())
    project_path.child("Cargo.toml").assert(predicate::path::is_file());
    project_path.child("src/main.rs").assert(predicate::path::is_file());

    // Contents
    let cargo_toml = fs::read_to_string(project_path.child("Cargo.toml").path())?;
    assert!(cargo_toml.contains("name = \"my-test-app\""));
    assert!(!cargo_toml.contains("__PROJECT_NAME__"));

    Ok(())
}

#[test]
fn test_create_dioxus_postgres() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = assert_fs::TempDir::new()?;
    let project_name = "my-dioxus-pg-app";
    let project_path = temp_dir.child(project_name);

    Command::cargo_bin("forge-cli")?
        .arg("new")
        .arg(project_path.path())
        .arg("--frontend").arg("dioxus")
        .arg("--db").arg("postgres")
        //.current_dir(temp_dir.path())
        .assert()
        .success();

    project_path.child(".env.example").assert(predicate::path::is_file());
    project_path.child("src/db.rs").assert(predicate::path::is_file());

    let cargo_toml = fs::read_to_string(project_path.child("Cargo.toml").path())?;
    assert!(cargo_toml.contains("dioxus ="), "Dioxus dependency missing");
    assert!(cargo_toml.contains("sqlx ="), "sqlx dependency missing");
    assert!(cargo_toml.contains("postgres"), "postgres feature for sqlx missing");

    Ok(())
}
