# Quick Start Guide

## Installation

```bash
# From the trace directory
cargo install --path .
```

Verify installation:
```bash
tracer --version
tracer --help
```

## Your First Session

```bash
# 1. Go to your project
cd ~/myproject

# 2. Initialize trace
tracer init

# 3. Create some issues
tracer create "Implement user authentication" -t epic -p 0
tracer create "Add login form" -t task -p 1
tracer create "Add logout button" -t task -p 1

# 4. Add dependencies
tracer dep add test-2 test-1 --type parent-child
tracer dep add test-3 test-1 --type parent-child

# 5. See what's ready
tracer ready

# 6. Start working
tracer update test-2 --status in_progress

# 7. Complete work
tracer close test-2 --reason "Implemented and tested"

# 8. Check stats
tracer stats
```

## Using with Claude Code

Just tell me (Claude) to use `trace` for tracking work! For example:

> "Hey Claude, check what work is ready in trace"

I'll automatically run:
```bash
tracer ready --json
```

> "Create an issue for fixing the authentication bug"

I'll run:
```bash
tracer create "Fix authentication bug" -t bug -p 0
```

See [AGENTS.md](AGENTS.md) for full integration details.

## Quick Reference

```bash
# Create
tracer create "Title" [-p priority] [-t type]

# List & Show
tracer list [--status open] [--priority 1]
tracer show <id>

# Update
tracer update <id> --status in_progress
tracer close <id> --reason "Done"

# Dependencies
tracer dep add <from> <to> [-t type]
tracer dep tree <id>

# Ready Work
tracer ready [--limit 5]
tracer blocked

# Export/Import
tracer export [-o file.jsonl]
tracer import [-i file.jsonl]

# Stats
tracer stats
```

## JSON Output

Add `--json` to any command for programmatic parsing:

```bash
tracer ready --json | jq '.[0]'
tracer list --json | jq 'length'
tracer stats --json
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
tracer create "Subtask 1" -t task -p 1 --deps "parent-child:$EPIC"
tracer create "Subtask 2" -t task -p 1 --deps "parent-child:$EPIC"
```

### Find and Work on Highest Priority
```bash
WORK=$(trace ready --limit 1 --json)
ID=$(echo $WORK | jq -r '.[0].id')
tracer update $ID --status in_progress
# ... do work ...
tracer close $ID --reason "Completed"
```

### Export for Backup
```bash
tracer export -o backup-$(date +%Y%m%d).jsonl
```

## Next Steps

- Read [README.md](README.md) for full documentation
- See [AGENTS.md](AGENTS.md) for AI agent integration
- Check [CONTRIBUTING.md](CONTRIBUTING.md) to contribute

Happy tracking! 🚀
