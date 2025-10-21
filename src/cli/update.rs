use anyhow::{Context, Result};
use clap::Args;
use tracer::storage::{IssueUpdates, Storage};
use tracer::types::*;

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

#[derive(Args)]
pub struct CommentArgs {
    /// Issue ID
    pub id: String,

    /// Comment text
    pub comment: String,
}

pub fn execute_update(args: UpdateArgs, storage: &mut Box<dyn Storage>, actor: &str, json: bool) -> Result<()> {
    // Verify issue exists
    let issue = storage.get_issue(&args.id)?
        .context(format!("Issue {} not found", args.id))?;

    // Auto-set assignee if status changes to in_progress and no assignee specified
    let assignee = if args.assignee.is_some() {
        args.assignee
    } else if args.status == Some(Status::InProgress) && issue.assignee.is_empty() {
        Some(actor.to_string())
    } else {
        None
    };

    let updates = IssueUpdates {
        title: args.title,
        description: args.description,
        design: None,
        acceptance_criteria: None,
        notes: None,
        status: args.status,
        priority: args.priority,
        issue_type: args.issue_type,
        assignee,
        estimated_minutes: None,
        external_ref: None,
    };

    storage.update_issue(&args.id, &updates, actor)?;

    if json {
        let updated = storage.get_issue(&args.id)?.expect("Issue should exist after update");
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

pub fn execute_comment(args: CommentArgs, storage: &mut Box<dyn Storage>, actor: &str, json: bool) -> Result<()> {
    // Verify issue exists
    storage.get_issue(&args.id)?
        .context(format!("Issue {} not found", args.id))?;

    storage.add_comment(&args.id, actor, &args.comment)?;

    if json {
        let issue = storage.get_issue(&args.id)?.expect("Issue should exist");
        println!("{}", serde_json::to_string_pretty(&issue)?);
    } else {
        use colored::Colorize;
        println!("✓ Added comment to {}", args.id.bold().cyan());
    }

    Ok(())
}

