use anyhow::{Context, Result};
use clap::Args;
use tracer::storage::Storage;
use chrono::{DateTime, Utc};

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

        // Show recent comments (always visible, not just with --full)
        let events = storage.get_events(&args.id, 20)?;
        let comments: Vec<_> = events.iter()
            .filter(|e| e.event_type == tracer::types::EventType::Commented)
            .collect();
        
        if !comments.is_empty() {
            use colored::Colorize;
            println!("\n  Recent comments:");
            for event in comments.iter().take(5) {
                let time_ago = format_time_ago(&event.created_at);
                println!("    {} {}: \"{}\"", 
                    event.actor.cyan(),
                    format!("({})", time_ago).dimmed(),
                    event.comment.as_ref().unwrap_or(&String::new())
                );
            }
        }

        // Show all events if --full
        if args.full {
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

fn format_time_ago(dt: &DateTime<Utc>) -> String {
    let now = Utc::now();
    let duration = now.signed_duration_since(*dt);
    
    if duration.num_days() > 0 {
        format!("{} days ago", duration.num_days())
    } else if duration.num_hours() > 0 {
        format!("{} hours ago", duration.num_hours())
    } else if duration.num_minutes() > 0 {
        format!("{} min ago", duration.num_minutes())
    } else {
        "just now".to_string()
    }
}

