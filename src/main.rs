mod scaffold;

use anyhow::{bail, Result};
use clap::{Parser, Subcommand, ValueEnum};
use colored::*;
use scaffold::{scaffold_project, scaffold_project_embedded};
use std::path::PathBuf;
use include_dir::{include_dir, Dir};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create a new Trestle project
    New {
        /// The name of the project to create
        name: String,

        /// The database to use for the project
        #[arg(long, value_enum, help = "Database type")]
        db: Option<Database>,

        /// The frontend framework to use
        #[arg(long, value_enum, default_value_t = Frontend::Dioxus, help = "Frontend framework")]
        frontend: Frontend,
    },
}

#[derive(ValueEnum, Clone, Debug, Copy, PartialEq, Eq)]
#[clap(rename_all = "kebab_case")]
pub enum Database {
    Postgres,
    Mysql,
    Mongodb,
    Firebase,
}

#[derive(ValueEnum, Clone, Debug, Copy, Default, PartialEq, Eq)]
#[clap(rename_all = "kebab_case")]
pub enum Frontend {
    #[default]
    Dioxus,
    Htmx,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New {
            name,
            db,
            frontend,
        } => {
            let project_path = PathBuf::from(name);

            // Extract just the final component as the project name
            let project_name = project_path
                .file_name()
                .ok_or_else(|| anyhow::anyhow!("Invalid project path: {}", name))?
                .to_str()
                .ok_or_else(|| anyhow::anyhow!("Project name contains invalid UTF-8"))?;

            println!(
                "🔥 Initializing new Trestle project '{}'...",
                project_name.bold().cyan()
            );

            // Determine the template directory based on flags
            let template_name = match (frontend, db) {
                (Frontend::Htmx, None) => "static-htmx",
                (Frontend::Dioxus, None) => "static-dioxus",
                (Frontend::Htmx, Some(Database::Postgres)) => "postgres-htmx",
                (Frontend::Dioxus, Some(Database::Postgres)) => "postgres-dioxus",
                (Frontend::Htmx, Some(Database::Mysql)) => "mysql-htmx",
                (Frontend::Dioxus, Some(Database::Mysql)) => "mysql-dioxus",
                (Frontend::Htmx, Some(Database::Mongodb)) => "mongodb-htmx",
                (Frontend::Dioxus, Some(Database::Mongodb)) => "mongodb-dioxus",
                (Frontend::Htmx, Some(Database::Firebase)) => "firebase-htmx",
                (Frontend::Dioxus, Some(Database::Firebase)) => "firebase-dioxus", 
            };

            // Prefer embedded templates (works in crates.io installs). Fallback to FS for local dev.
            static TEMPLATES: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates");
            if TEMPLATES.get_dir(template_name).is_some() {
                scaffold_project_embedded(&project_path, project_name, template_name, &TEMPLATES)?;
            } else {
                let template_path = PathBuf::from("templates").join(template_name);
                if !template_path.exists() {
                    let error_msg = format!(
                        "Template '{}' not found in embedded assets or on disk!",
                        template_path.display()
                    );
                    bail!(error_msg.red().to_string());
                }

                scaffold_project(&project_path, project_name, &template_path)?;
            }

            println!("\n🎉 Success! Your project is ready.");
            println!("\nNext steps:");
            println!("   1. {}", format!("cd {}", name).cyan());
            if db.is_some() {
                println!("   2. {}", "cp .env.example .env".cyan());
                println!("   3. {}", "Update .env with your credentials".cyan());
                println!("   4. {}", "cargo run".cyan());
            } else {
                println!("   2. {}", "cargo run".cyan());
            }
        }
    }

    Ok(())
}