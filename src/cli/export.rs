use anyhow::{Context, Result};
use clap::Args;
use tracer::storage::Storage;
use tracer::types::*;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

#[derive(Args)]
pub struct ExportArgs {
    /// Output file (default: stdout)
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Filter by status
    #[arg(long, value_parser = clap::value_parser!(Status))]
    pub status: Option<Status>,
}

#[derive(Args)]
pub struct ImportArgs {
    /// Input file (default: stdin)
    #[arg(short, long)]
    pub input: Option<PathBuf>,

    /// Skip existing issues (don't update)
    #[arg(long)]
    pub skip_existing: bool,

    /// Dry run (show what would be imported)
    #[arg(long)]
    pub dry_run: bool,
}

pub fn execute_export(args: ExportArgs, storage: &dyn Storage) -> Result<()> {
    let filter = IssueFilter {
        status: args.status,
        priority: None,
        issue_type: None,
        assignee: None,
        labels: Vec::new(),
        limit: None,
    };

    let mut issues = storage.search_issues("", &filter)?;
    
    // Sort by ID for consistent output
    issues.sort_by(|a, b| a.id.cmp(&b.id));

    // Add dependencies to each issue
    for issue in &mut issues {
        issue.dependencies = storage.get_dependency_records(&issue.id)?;
    }

    // Write JSONL
    let output: Box<dyn Write> = if let Some(path) = args.output {
        Box::new(BufWriter::new(File::create(path)?))
    } else {
        Box::new(BufWriter::new(std::io::stdout()))
    };

    export_jsonl(&issues, output)?;

    Ok(())
}

fn export_jsonl(issues: &[Issue], mut writer: Box<dyn Write>) -> Result<()> {
    for issue in issues {
        serde_json::to_writer(&mut writer, issue)?;
        writeln!(writer)?;
    }
    writer.flush()?;
    Ok(())
}

pub fn execute_import(args: ImportArgs, storage: &mut Box<dyn Storage>, actor: &str) -> Result<()> {
    let reader: Box<dyn BufRead> = if let Some(path) = args.input {
        Box::new(BufReader::new(File::open(path)?))
    } else {
        Box::new(BufReader::new(std::io::stdin()))
    };

    let mut issues = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let issue: Issue = serde_json::from_str(&line)
            .context(format!("Failed to parse JSONL line: {}", line))?;
        issues.push(issue);
    }

    if args.dry_run {
        println!("Dry run: would import {} issue(s)", issues.len());
        return Ok(());
    }

    let mut created = 0;
    let mut updated = 0;
    let mut skipped = 0;

    for issue in issues {
        let exists = storage.get_issue(&issue.id)?.is_some();
        
        if exists {
            if args.skip_existing {
                skipped += 1;
                continue;
            }
            // Update existing issue
            let updates = tracer::storage::IssueUpdates {
                title: Some(issue.title.clone()),
                description: Some(issue.description.clone()),
                design: Some(issue.design.clone()),
                acceptance_criteria: Some(issue.acceptance_criteria.clone()),
                notes: Some(issue.notes.clone()),
                status: Some(issue.status),
                priority: Some(issue.priority),
                issue_type: Some(issue.issue_type),
                assignee: Some(issue.assignee.clone()),
                estimated_minutes: Some(issue.estimated_minutes),
                external_ref: Some(issue.external_ref.clone()),
            };
            storage.update_issue(&issue.id, &updates, actor)?;
            updated += 1;
        } else {
            // Create new issue
            storage.create_issue(&issue, actor)?;
            created += 1;
        }

        // Import dependencies
        for dep in &issue.dependencies {
            // Check if dependency already exists
            let existing_deps = storage.get_dependency_records(&issue.id)?;
            let already_exists = existing_deps.iter().any(|d| 
                d.depends_on_id == dep.depends_on_id && d.dep_type == dep.dep_type
            );
            
            if !already_exists {
                storage.add_dependency(dep, actor)?;
            }
        }
    }

    use colored::Colorize;
    println!("✓ Import complete:");
    println!("  Created: {}", created.to_string().green());
    println!("  Updated: {}", updated.to_string().yellow());
    if skipped > 0 {
        println!("  Skipped: {}", skipped.to_string().dimmed());
    }

    Ok(())
}

/// Auto-export issues to JSONL file
pub fn auto_export(storage: &dyn Storage, jsonl_path: &Path) -> Result<()> {
    // Get all dirty issues
    let dirty_ids = storage.get_dirty_issues()?;
    if dirty_ids.is_empty() {
        return Ok(()); // Nothing to export
    }

    // Read existing JSONL into memory
    let mut existing_issues: std::collections::HashMap<String, Issue> = std::collections::HashMap::new();
    if jsonl_path.exists() {
        let file = File::open(jsonl_path)?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            if let Ok(issue) = serde_json::from_str::<Issue>(&line) {
                existing_issues.insert(issue.id.clone(), issue);
            }
        }
    }

    // Update dirty issues
    for id in &dirty_ids {
        if let Some(issue) = storage.get_issue(id)? {
            let mut issue_with_deps = issue.clone();
            issue_with_deps.dependencies = storage.get_dependency_records(id)?;
            existing_issues.insert(id.clone(), issue_with_deps);
        } else {
            // Issue was deleted, remove from map
            existing_issues.remove(id);
        }
    }

    // Write all issues back (sorted by ID)
    let mut all_issues: Vec<Issue> = existing_issues.into_values().collect();
    all_issues.sort_by(|a, b| a.id.cmp(&b.id));

    let file = File::create(jsonl_path)?;
    let writer = BufWriter::new(file);
    export_jsonl(&all_issues, Box::new(writer))?;

    Ok(())
}

/// Auto-import issues from JSONL if it's newer than the database
pub fn auto_import(storage: &mut Box<dyn Storage>, jsonl_path: &PathBuf, actor: &str) -> Result<bool> {
    if !jsonl_path.exists() {
        return Ok(false);
    }

    // Read JSONL and compute hash
    let jsonl_data = std::fs::read(jsonl_path)?;
    let current_hash = tracer::utils::compute_hash(&jsonl_data);

    // Check last import hash
    let last_hash = storage.get_metadata("last_import_hash")?;
    
    if last_hash.as_deref() == Some(&current_hash) {
        return Ok(false); // No changes
    }

    // Import the JSONL
    let reader = BufReader::new(std::io::Cursor::new(jsonl_data));
    let mut issues = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let issue: Issue = serde_json::from_str(&line)?;
        issues.push(issue);
    }

    // Import all issues
    for issue in issues {
        let exists = storage.get_issue(&issue.id)?.is_some();
        
        if exists {
            let updates = tracer::storage::IssueUpdates {
                title: Some(issue.title.clone()),
                description: Some(issue.description.clone()),
                design: Some(issue.design.clone()),
                acceptance_criteria: Some(issue.acceptance_criteria.clone()),
                notes: Some(issue.notes.clone()),
                status: Some(issue.status),
                priority: Some(issue.priority),
                issue_type: Some(issue.issue_type),
                assignee: Some(issue.assignee.clone()),
                estimated_minutes: Some(issue.estimated_minutes),
                external_ref: Some(issue.external_ref.clone()),
            };
            storage.update_issue(&issue.id, &updates, actor)?;
        } else {
            storage.create_issue(&issue, actor)?;
        }

        // Import dependencies
        for dep in &issue.dependencies {
            let existing_deps = storage.get_dependency_records(&issue.id)?;
            let already_exists = existing_deps.iter().any(|d| 
                d.depends_on_id == dep.depends_on_id && d.dep_type == dep.dep_type
            );
            
            if !already_exists {
                storage.add_dependency(dep, actor)?;
            }
        }
    }

    // Store the hash
    storage.set_metadata("last_import_hash", &current_hash)?;

    Ok(true)
}

