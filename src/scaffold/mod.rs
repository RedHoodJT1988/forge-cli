use anyhow::{Context, Result};
use colored::*;
use std::fs;
use std::path::{Path};
use walkdir::WalkDir;

const PLACEHOLDER: &str = "__PROJECT_NAME__";

pub fn scaffold_project(
    target_dir: &Path,
    project_name: &str,
    template_path: &Path,
) -> Result<()> {
    if target_dir.exists() {
        anyhow::bail!("Directory '{}' already exists.", target_dir.display());
    }

    println!(
        "{} Creating project in '{}'...",
        "✓".green(),
        target_dir.display()
    );

    for entry in WalkDir::new(template_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        let relative_path = path.strip_prefix(template_path)?;
        let target_path = target_dir.join(relative_path);

        if path.is_dir() {
            fs::create_dir_all(&target_path)
                .with_context(|| format!("Failed to create directory: {}", target_path.display()))?;
        } else {
            let content_bytes = fs::read(path)
                .with_context(|| format!("Failed to read template file: {}", path.display()))?;

            let new_content = if let Ok(content_str) = String::from_utf8(content_bytes.clone()) {
                content_str.replace(PLACEHOLDER, project_name).into_bytes()
            } else {
                content_bytes
            };

            fs::write(&target_path, new_content)
                .with_context(|| format!("Failed to write file: {}", target_path.display()))?;
        }
    }

    println!(
        "{} {}",
        "✓".green(),
        "Project scaffolded successfully!".bold()
    );
    Ok(())
}

//
// --- START OF FIXES ---
//
#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::prelude::*; // Import traits for .child(), .write_str(), etc.
    use predicates::prelude::*;
    use std::fs;
    // We don't need `tempfile` anymore, so we remove it.
    // use tempfile::tempdir; // <-- REMOVE THIS

    #[test]
    fn test_scaffold_project_copies_and_replaces() {
        // 1. Setup a temporary template directory using assert_fs
        let template_dir = assert_fs::TempDir::new().unwrap();
        let template_file = template_dir.child("src/main.rs"); // <-- .child() now works
        template_file
            .write_str("fn main() { println!(\"__PROJECT_NAME__\"); }")
            .unwrap();

        let template_binary_file = template_dir.child("icon.png"); // <-- .child() now works
        template_binary_file
            .write_binary(b"\x89PNG\r\n\x1a\n")
            .unwrap();

        // 2. Setup a temporary target directory using assert_fs
        let target_root = assert_fs::TempDir::new().unwrap();
        let project_name = "test-runner";
        let target_project_dir = target_root.child(project_name); // <-- .child() now works

        // 3. Run the scaffolding function
        // We pass the .path() of the ChildPath to the function
        scaffold_project(target_project_dir.path(), project_name, template_dir.path()).unwrap();

        // 4. Assert the files were created
        let expected_file = target_project_dir.child("src/main.rs"); // <-- .child()
        expected_file.assert(predicate::path::is_file()); // <-- .assert() now works

        let expected_binary = target_project_dir.child("icon.png"); // <-- .child()
        expected_binary.assert(predicate::path::is_file()); // <-- .assert() now works

        // 5. Assert the placeholder was replaced
        // We use .path() to read the file
        let content = fs::read_to_string(expected_file.path()).unwrap();
        assert!(content.contains("fn main() { println!(\"test-runner\"); }"));
        assert!(!content.contains("__PROJECT_NAME__"));

        // 6. Assert binary file was copied correctly
        let binary_content = fs::read(expected_binary.path()).unwrap();
        assert_eq!(binary_content, b"\x89PNG\r\n\x1a\n");
    }

    #[test]
    fn test_scaffold_project_fails_if_exists() {
        // 1. Setup a target directory that already exists
        let target_root = assert_fs::TempDir::new().unwrap(); // Use assert_fs
        let project_name = "already-exists";
        let target_project_dir = target_root.child(project_name); // Use .child()
        fs::create_dir_all(target_project_dir.path()).unwrap(); // Use .path()

        // 2. Setup a dummy template dir
        let template_dir = assert_fs::TempDir::new().unwrap(); // Use assert_fs

        // 3. Run and assert failure
        let result =
            scaffold_project(target_project_dir.path(), project_name, template_dir.path());
        
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            // Use .path() to get the display path
            format!("Directory '{}' already exists.", target_project_dir.path().display())
        );
    }
}