use anyhow::{Context, Result};
use clap::Args;
use tracer::storage::Storage;

#[derive(Args)]
pub struct ShowArgs {
    /// Issue ID
    pub id: String,

    /// Show full details including events
    #[arg(long)]
    pub full: bool,
}

pub fn execute(args: ShowArgs, storage: &dyn Storage, json: bool) -> Result<()> {
    let issue = storage.get_issue(&args.id)?
        .context(format!("Issue {} not found", args.id))?;

    if json {
        println!("{}", serde_json::to_string_pretty(&issue)?);
    } else {
        print!("{}", tracer::utils::format_issue(&issue, true));

        // Show labels
        let labels = storage.get_labels(&args.id)?;
        if !labels.is_empty() {
            println!("\n  Labels: {}", labels.join(", "));
        }

        // Show dependencies
        let deps = storage.get_dependency_records(&args.id)?;
        if !deps.is_empty() {
            println!("\n  Dependencies:");
            for dep in deps {
                println!("    {} → {} ({})", dep.issue_id, dep.depends_on_id, dep.dep_type);
            }
        }

        // Show events if --full
        if args.full {
            let events = storage.get_events(&args.id, 20)?;
            if !events.is_empty() {
                println!("\n  Recent Events:");
                for event in events {
                    println!("    [{}] {} by {}", 
                        event.created_at.format("%Y-%m-%d %H:%M"),
                        event.event_type,
                        event.actor
                    );
                    if let Some(comment) = &event.comment {
                        println!("      {}", comment);
                    }
                }
            }
        }
    }

    Ok(())
}

