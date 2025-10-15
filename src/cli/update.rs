use anyhow::{Context, Result};
use clap::Args;
use trace::storage::{IssueUpdates, Storage};
use trace::types::*;

#[derive(Args)]
pub struct UpdateArgs {
    /// Issue ID
    pub id: String,

    /// New title
    #[arg(long)]
    pub title: Option<String>,

    /// New description
    #[arg(long)]
    pub description: Option<String>,

    /// New status
    #[arg(long, value_parser = clap::value_parser!(Status))]
    pub status: Option<Status>,

    /// New priority
    #[arg(long)]
    pub priority: Option<i32>,

    /// New issue type
    #[arg(long, value_parser = clap::value_parser!(IssueType))]
    pub issue_type: Option<IssueType>,

    /// New assignee
    #[arg(long)]
    pub assignee: Option<String>,
}

#[derive(Args)]
pub struct CloseArgs {
    /// Issue IDs to close
    pub ids: Vec<String>,

    /// Reason for closing
    #[arg(long, default_value = "Completed")]
    pub reason: String,
}

pub fn execute_update(args: UpdateArgs, storage: &mut Box<dyn Storage>, actor: &str, json: bool) -> Result<()> {
    // Verify issue exists
    storage.get_issue(&args.id)?
        .context(format!("Issue {} not found", args.id))?;

    let updates = IssueUpdates {
        title: args.title,
        description: args.description,
        design: None,
        acceptance_criteria: None,
        notes: None,
        status: args.status,
        priority: args.priority,
        issue_type: args.issue_type,
        assignee: args.assignee,
        estimated_minutes: None,
        external_ref: None,
    };

    storage.update_issue(&args.id, &updates, actor)?;

    if json {
        let updated = storage.get_issue(&args.id)?.unwrap();
        println!("{}", serde_json::to_string_pretty(&updated)?);
    } else {
        use colored::Colorize;
        println!("✓ Updated issue {}", args.id.bold().cyan());
    }

    Ok(())
}

pub fn execute_close(args: CloseArgs, storage: &mut Box<dyn Storage>, actor: &str, json: bool) -> Result<()> {
    let mut closed = Vec::new();

    for id in &args.ids {
        // Verify issue exists
        storage.get_issue(id)?
            .context(format!("Issue {} not found", id))?;
        
        storage.close_issue(id, &args.reason, actor)?;
        closed.push(id.clone());
    }

    if json {
        let issues: Vec<_> = closed.iter()
            .filter_map(|id| storage.get_issue(id).ok().flatten())
            .collect();
        println!("{}", serde_json::to_string_pretty(&issues)?);
    } else {
        use colored::Colorize;
        for id in closed {
            println!("✓ Closed issue {}", id.bold().cyan());
        }
    }

    Ok(())
}

