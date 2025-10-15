use anyhow::Result;
use clap::Args;
use tracer::storage::Storage;
use tracer::types::*;

#[derive(Args)]
pub struct ListArgs {
    /// Filter by status
    #[arg(long, value_parser = clap::value_parser!(Status))]
    pub status: Option<Status>,

    /// Filter by priority
    #[arg(long)]
    pub priority: Option<i32>,

    /// Filter by issue type
    #[arg(long, value_parser = clap::value_parser!(IssueType))]
    pub issue_type: Option<IssueType>,

    /// Filter by assignee
    #[arg(long)]
    pub assignee: Option<String>,

    /// Filter by labels (comma-separated)
    #[arg(short, long, value_delimiter = ',')]
    pub labels: Vec<String>,

    /// Maximum number of results
    #[arg(long)]
    pub limit: Option<usize>,
}

pub fn execute(args: ListArgs, storage: &dyn Storage, json: bool) -> Result<()> {
    let filter = IssueFilter {
        status: args.status,
        priority: args.priority,
        issue_type: args.issue_type,
        assignee: args.assignee,
        labels: args.labels,
        limit: args.limit,
    };

    let issues = storage.search_issues("", &filter)?;

    if json {
        println!("{}", serde_json::to_string_pretty(&issues)?);
    } else {
        if issues.is_empty() {
            println!("No issues found");
            return Ok(());
        }

        println!("Found {} issue(s):\n", issues.len());
        for issue in issues {
            print!("{}", tracer::utils::format_issue(&issue, false));
            println!();
        }
    }

    Ok(())
}

