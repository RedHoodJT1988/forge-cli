use anyhow::{Context, Result};
use colored::*;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

// A placeholder to be replaced in template files
const PLACEHOLDER: &str = "__PROJECT_NAME__";

/// Scaffold a new project by copying a template directory.
pub fn scaffold_project(project_name: &str, template_path: &Path) -> Result<()> {
  let target_dir = PathBuf::from(project_name);
  if target_dir.exists() {
    anyhow::bail!("Directory '{}' already exists.", project_name);
  }

  println!(
    "{} Creating project in '{}'...",
    "✓".green(),
    target_dir.display()
  );

  // Walk through the template directory
  for entry in WalkDir::new(template_path).into_iter().filter_map(|e| e.ok()) {
    let path = entry.path();

    // Create a relative path to preserve the directory structure
    let relative_path = path.strip_prefix(template_path)?;
    let target_path = target_dir.join(relative_path);

    if path.is_dir() {
      fs::create_dir_all(&target_path)
        .with_context(|| format!("Failed to craete directory: {}", target_path.display()))?;
    } else {
      if let Some(parent) = target_path.parent() {
        if !parent.exists() {
          fs::create_dir_all(parent)?;
        }
      }

      // Read, replace placeholder, and write file
      let content = fs::read(path)?;

      // Try to convert to string to replace, otherwise just copy bytes
      if let Ok(str_content) = String::from_utf8(content.clone()) {
        let new_content = str_content.replace(PLACEHOLDER, project_name);
        fs::write(&target_path, new_content.as_bytes())
          .with_context(|| format!("Failed to write file: {}", target_path.display()))?;
      } else {
        fs::write(&target_path, &content)
          .with_context(|| format!("Failed to write binary file: {}", target_path.display()))?;
      }
    }
  }

  println!(
    "{}{}",
    "✓".green(),
    "Project scaffolded successfully!".bold()
  );
  Ok(())
}