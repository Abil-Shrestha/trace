use anyhow::Result;
use clap::Args;
use trace::storage::Storage;

#[derive(Args)]
pub struct StatsArgs {}

pub fn execute(_args: StatsArgs, storage: &dyn Storage, json: bool) -> Result<()> {
    let stats = storage.get_statistics()?;

    if json {
        println!("{}", serde_json::to_string_pretty(&stats)?);
    } else {
        use colored::Colorize;
        
        println!("{}", "Issue Statistics".bold());
        println!();
        println!("  Total Issues:      {}", stats.total_issues);
        println!("  Open:              {}", stats.open_issues.to_string().green());
        println!("  In Progress:       {}", stats.in_progress_issues.to_string().blue());
        println!("  Blocked:           {}", stats.blocked_issues.to_string().red());
        println!("  Closed:            {}", stats.closed_issues.to_string().dimmed());
        println!();
        println!("  Ready to Work:     {}", stats.ready_issues.to_string().bold().green());
        println!();
        println!("  Avg Lead Time:     {:.1} hours", stats.average_lead_time_hours);
    }

    Ok(())
}

