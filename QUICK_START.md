# Quick Start Guide

## Installation

```bash
# From the trace directory
cargo install --path .
```

Verify installation:
```bash
trace --version
trace --help
```

## Your First Session

```bash
# 1. Go to your project
cd ~/myproject

# 2. Initialize trace
trace init

# 3. Create some issues
trace create "Implement user authentication" -t epic -p 0
trace create "Add login form" -t task -p 1
trace create "Add logout button" -t task -p 1

# 4. Add dependencies
trace dep add test-2 test-1 --type parent-child
trace dep add test-3 test-1 --type parent-child

# 5. See what's ready
trace ready

# 6. Start working
trace update test-2 --status in_progress

# 7. Complete work
trace close test-2 --reason "Implemented and tested"

# 8. Check stats
trace stats
```

## Using with Claude Code

Just tell me (Claude) to use `trace` for tracking work! For example:

> "Hey Claude, check what work is ready in trace"

I'll automatically run:
```bash
trace ready --json
```

> "Create an issue for fixing the authentication bug"

I'll run:
```bash
trace create "Fix authentication bug" -t bug -p 0
```

See [CLAUDE.md](CLAUDE.md) for full integration details.

## Quick Reference

```bash
# Create
trace create "Title" [-p priority] [-t type]

# List & Show
trace list [--status open] [--priority 1]
trace show <id>

# Update
trace update <id> --status in_progress
trace close <id> --reason "Done"

# Dependencies
trace dep add <from> <to> [-t type]
trace dep tree <id>

# Ready Work
trace ready [--limit 5]
trace blocked

# Export/Import
trace export [-o file.jsonl]
trace import [-i file.jsonl]

# Stats
trace stats
```

## JSON Output

Add `--json` to any command for programmatic parsing:

```bash
trace ready --json | jq '.[0]'
trace list --json | jq 'length'
trace stats --json
```

## Tips

1. **Initialize once per project**: `trace init` in your project root
2. **Commit the JSONL**: `git add .trace/issues.jsonl`
3. **Let AI agents use it**: They'll track work automatically
4. **Check ready work first**: Always start with `trace ready`
5. **Use JSON for scripts**: `--json` flag on all commands

## Examples

### Create Epic with Subtasks
```bash
EPIC=$(trace create "New Feature" -t epic -p 0 --json | jq -r '.id')
trace create "Subtask 1" -t task -p 1 --deps "parent-child:$EPIC"
trace create "Subtask 2" -t task -p 1 --deps "parent-child:$EPIC"
```

### Find and Work on Highest Priority
```bash
WORK=$(trace ready --limit 1 --json)
ID=$(echo $WORK | jq -r '.[0].id')
trace update $ID --status in_progress
# ... do work ...
trace close $ID --reason "Completed"
```

### Export for Backup
```bash
trace export -o backup-$(date +%Y%m%d).jsonl
```

## Next Steps

- Read [README.md](README.md) for full documentation
- See [CLAUDE.md](CLAUDE.md) for AI agent integration
- Check [CONTRIBUTING.md](CONTRIBUTING.md) to contribute

Happy tracking! 🚀
