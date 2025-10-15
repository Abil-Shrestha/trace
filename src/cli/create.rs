use anyhow::{Context, Result};
use chrono::Utc;
use clap::Args;
use std::path::Path;
use trace::storage::Storage;
use trace::types::*;
use std::path::PathBuf;

#[derive(Args)]
pub struct CreateArgs {
    /// Issue title
    #[arg(value_name = "TITLE")]
    pub title: Option<String>,

    /// Issue description
    #[arg(short, long)]
    pub description: Option<String>,

    /// Priority (0=highest, 4=lowest)
    #[arg(short, long, default_value = "2")]
    pub priority: i32,

    /// Issue type
    #[arg(short = 't', long, value_parser = clap::value_parser!(IssueType), default_value = "task")]
    pub issue_type: IssueType,

    /// Assignee
    #[arg(short, long)]
    pub assignee: Option<String>,

    /// Labels (comma-separated)
    #[arg(short, long, value_delimiter = ',')]
    pub labels: Vec<String>,

    /// Dependencies (format: type:id or just id for blocks)
    #[arg(long, value_delimiter = ',')]
    pub deps: Vec<String>,

    /// Explicit issue ID (for avoiding collisions)
    #[arg(long)]
    pub id: Option<String>,

    /// Create from markdown file
    #[arg(short = 'f', long)]
    pub file: Option<PathBuf>,
}

pub fn execute(args: CreateArgs, storage: &mut Box<dyn Storage>, actor: &str, prefix: &str, json: bool) -> Result<()> {
    if let Some(file_path) = args.file {
        // Create issues from markdown file
        create_from_file(&file_path, storage, actor, prefix, json)?;
    } else {
        // Create single issue
        let title = args.title.clone().context("Title is required (or use --file)")?;
        create_single(args, &title, storage, actor, prefix, json)?;
    }
    
    Ok(())
}

fn create_single(args: CreateArgs, title: &str, storage: &mut Box<dyn Storage>, actor: &str, prefix: &str, json: bool) -> Result<()> {
    let now = Utc::now();
    
    // Generate or use explicit ID
    let id = if let Some(explicit_id) = args.id {
        explicit_id
    } else {
        storage.generate_id(prefix)?
    };

    let issue = Issue {
        id: id.clone(),
        title: title.to_string(),
        description: args.description.unwrap_or_default(),
        design: String::new(),
        acceptance_criteria: String::new(),
        notes: String::new(),
        status: Status::Open,
        priority: args.priority,
        issue_type: args.issue_type,
        assignee: args.assignee.unwrap_or_default(),
        estimated_minutes: None,
        created_at: now,
        updated_at: now,
        closed_at: None,
        external_ref: None,
        dependencies: Vec::new(),
    };

    storage.create_issue(&issue, actor)?;

    // Add labels
    for label in &args.labels {
        storage.add_label(&id, label, actor)?;
    }

    // Add dependencies
    for dep_spec in &args.deps {
        let (dep_type, depends_on_id) = trace::utils::parse_dependency_spec(dep_spec)?;
        let dep = Dependency {
            issue_id: id.clone(),
            depends_on_id,
            dep_type,
            created_at: now,
            created_by: actor.to_string(),
        };
        storage.add_dependency(&dep, actor)?;
    }

    if json {
        println!("{}", serde_json::to_string_pretty(&issue)?);
    } else {
        use colored::Colorize;
        println!("✓ Created issue {} {}", id.bold().cyan(), title);
        if !args.labels.is_empty() {
            println!("  Labels: {}", args.labels.join(", "));
        }
        if !args.deps.is_empty() {
            println!("  Dependencies: {}", args.deps.join(", "));
        }
    }

    Ok(())
}

fn create_from_file(_file_path: &Path, _storage: &mut Box<dyn Storage>, _actor: &str, _prefix: &str, _json: bool) -> Result<()> {
    // TODO: Implement markdown file parsing
    anyhow::bail!("Creating from markdown files is not yet implemented");
}

