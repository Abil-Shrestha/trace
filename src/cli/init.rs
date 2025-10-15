use anyhow::Result;
use clap::Args;
use std::path::PathBuf;
use trace::Storage;

#[derive(Args)]
pub struct InitArgs {
    /// ID prefix for issues (default: bd)
    #[arg(long, default_value = "bd")]
    pub prefix: String,

    /// Database path (default: .trace/<prefix>.db)
    #[arg(long)]
    pub path: Option<PathBuf>,
}

pub fn execute(args: InitArgs) -> Result<()> {
    let db_path = if let Some(path) = args.path {
        path
    } else {
        let current = std::env::current_dir()?;
        let trace_dir = current.join(".trace");
        std::fs::create_dir_all(&trace_dir)?;
        trace_dir.join(format!("{}.db", args.prefix))
    };

    // Create the database (schema is auto-initialized)
    let mut storage = trace::storage::sqlite::SqliteStorage::new(&db_path)?;
    
    // Set the prefix in config
    storage.set_config("prefix", &args.prefix)?;

    println!("✓ Initialized trace database at {}", db_path.display());
    println!("  Prefix: {}", args.prefix);
    println!("  JSONL: {}", trace::utils::find_jsonl_path(&db_path).display());
    println!();
    println!("Next steps:");
    println!("  1. Create your first issue: trace create \"My first task\"");
    println!("  2. See ready work: trace ready");
    println!("  3. Add to git: git add .trace/issues.jsonl");

    Ok(())
}

