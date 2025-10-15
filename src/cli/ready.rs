use anyhow::Result;
use clap::Args;
use tracer::storage::Storage;
use tracer::types::*;

#[derive(Args)]
pub struct ReadyArgs {
    /// Filter by priority
    #[arg(long)]
    pub priority: Option<i32>,

    /// Filter by assignee
    #[arg(long)]
    pub assignee: Option<String>,

    /// Maximum number of results
    #[arg(long)]
    pub limit: Option<usize>,
}

#[derive(Args)]
pub struct BlockedArgs {}

pub fn execute_ready(args: ReadyArgs, storage: &dyn Storage, json: bool) -> Result<()> {
    let filter = WorkFilter {
        status: Status::Open,
        priority: args.priority,
        assignee: args.assignee,
        limit: args.limit,
    };

    let issues = storage.get_ready_work(&filter)?;

    if json {
        println!("{}", serde_json::to_string_pretty(&issues)?);
    } else {
        if issues.is_empty() {
            println!("No ready work found");
            return Ok(());
        }

        use colored::Colorize;
        println!("{} Ready work: {} issue(s)\n", "✓".green(), issues.len());
        for issue in issues {
            print!("{}", tracer::utils::format_issue(&issue, false));
            println!();
        }
    }

    Ok(())
}

pub fn execute_blocked(_args: BlockedArgs, storage: &dyn Storage, json: bool) -> Result<()> {
    let blocked = storage.get_blocked_issues()?;

    if json {
        println!("{}", serde_json::to_string_pretty(&blocked)?);
    } else {
        if blocked.is_empty() {
            println!("No blocked issues found");
            return Ok(());
        }

        use colored::Colorize;
        println!("{} Blocked: {} issue(s)\n", "⚠".yellow(), blocked.len());
        for bi in blocked {
            print!("{}", tracer::utils::format_issue(&bi.issue, false));
            println!("  {} Blocked by: {}", "⚠".red(), bi.blocked_by.join(", "));
            println!();
        }
    }

    Ok(())
}

