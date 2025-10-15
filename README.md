# Trace ⚡

> **Blazing-fast issue tracker for AI agents**

Trace is a lightweight, dependency-aware issue tracker designed specifically for AI coding agents. Written in Rust for maximum performance, it helps agents maintain context across long sessions, track complex dependencies, and discover ready work automatically.

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)

## Why Trace?

Traditional issue trackers are built for humans clicking through web UIs. Trace is built for **AI agents**:

- 🤖 **Agent-first design** - JSON output, CLI-friendly, programmatic workflows
- ⚡ **Blazing fast** - 2-3x faster than Go implementation, optimized Rust + SQLite
- 🔗 **Smart dependencies** - Four dependency types (blocks, related, parent-child, discovered-from)
- 📋 **Ready work detection** - Automatically finds unblocked tasks
- 📦 **Git-native** - JSONL storage synced via git, no server needed
- 🌍 **Distributed** - Multiple agents on different machines share one logical database
- 💾 **Full audit trail** - Every change is logged with context

## Features

- ✨ **Zero setup** - `trace init` creates project-local database
- 🔗 **Dependency tracking** - Four types: blocks, related, parent-child, discovered-from
- 📋 **Ready work detection** - Finds issues with no open blockers
- 🤖 **Agent-friendly** - `--json` output for all commands
- 📦 **Git-versioned** - JSONL records in git, synced across machines
- 🌍 **Distributed** - Multiple agents share one database via git
- 🏗️ **Extensible** - SQLite database you can extend
- 🔍 **Multi-project** - Each project isolated with auto-discovery
- 🌲 **Dependency trees** - Visualize complex relationships
- 🎨 **Beautiful CLI** - Colored output for humans, JSON for bots
- 💾 **Full audit trail** - Every change tracked with timestamps

## Installation

### From Source (Recommended)

```bash
# Clone the repository
git clone https://github.com/Abil-Shrestha/trace.git
cd trace

# Build release version
cargo build --release

# Install to system
cargo install --path .
```

The binary will be available as `trace`.

### Using Cargo

```bash
cargo install trace-tracker
```

## Quick Start

```bash
# Initialize in your project
trace init

# Create an issue
trace create "Fix authentication bug" -p 1 -t bug

# See ready work
trace ready

# Show issue details
trace show test-1

# Add dependencies
trace dep add test-2 test-1  # test-2 depends on test-1

# Close an issue
trace close test-1 --reason "Fixed"
```

## Commands

### Core Commands

```bash
trace init [--prefix bd]           # Initialize project database
trace create <title> [OPTIONS]     # Create new issue
trace list [OPTIONS]                # List issues with filters
trace show <id>                     # Show issue details
trace update <id> [OPTIONS]         # Update issue fields
trace close <id>... [--reason]      # Close one or more issues
trace ready [OPTIONS]               # Find ready work (no blockers)
trace blocked                       # Show blocked issues
trace stats                         # Show statistics
```

### Dependency Management

```bash
trace dep add <issue-id> <depends-on-id> [--type TYPE]
trace dep remove <issue-id> <depends-on-id>
trace dep tree <issue-id> [--max-depth 50]
trace dep cycles
```

Dependency types:

- `blocks` - Hard blocker (affects ready work)
- `related` - Soft relationship
- `parent-child` - Epic/subtask hierarchy
- `discovered-from` - Issue discovered during work

### Data Management

```bash
trace export [-o file.jsonl]        # Export to JSONL
trace import [-i file.jsonl]        # Import from JSONL
```

## JSON Output

All commands support `--json` for programmatic use:

```bash
# Get ready work
trace ready --json | jq '.[0]'

# Create issue and capture ID
ISSUE_ID=$(trace create "Task" --json | jq -r '.id')

# List with filtering
trace list --status open --json | jq 'length'
```

## Agent Integration Example

```bash
# Agent workflow
WORK=$(trace ready --limit 1 --json)
ISSUE_ID=$(echo $WORK | jq -r '.[0].id')

# Claim work
trace update $ISSUE_ID --status in_progress

# Discover new work during execution
NEW_ID=$(trace create "Fix edge case" -t bug --json | jq -r '.id')
trace dep add $NEW_ID $ISSUE_ID --type discovered-from

# Complete work
trace close $ISSUE_ID --reason "Implemented and tested"
```

## Database Discovery

Trace finds your database automatically:

1. `--db` flag: `trace --db /path/to/db.db list`
2. `$TRACE_DB` environment variable
3. `.trace/*.db` in current directory or ancestors (like git)
4. `~/.trace/default.db` as fallback

```bash
# Initialize per-project
cd ~/myproject
trace init --prefix myapp

# Works from any subdirectory
cd ~/myproject/src/components
trace create "Fix navbar"  # Uses ~/myproject/.trace/myapp.db
```

## Performance

Trace is built for speed:

| Operation        | Time  |
| ---------------- | ----- |
| Create issue     | ~5ms  |
| List 1000 issues | ~15ms |
| Ready work query | ~10ms |
| Dependency tree  | ~25ms |

_Benchmarks on M1 MacBook Pro with 10,000 issues_

## Why Git-Based?

Trace stores issues as JSONL (JSON Lines) in git:

- ✅ **No server required** - Works offline, no API limits
- ✅ **Git-friendly** - One line per issue = clean diffs
- ✅ **Distributed** - Clone repo = clone database
- ✅ **Versioned** - Full history via git
- ✅ **Mergeable** - AI can resolve conflicts
- ✅ **Scriptable** - Use jq, grep, standard tools

Auto-sync keeps SQLite and JSONL in sync:

- Auto-export after changes (immediate)
- Auto-import after git pull (hash-based)

## Development

```bash
# Run tests
cargo test

# Build debug version
cargo build

# Build release version
cargo build --release

# Run with debug output
TRACE_DEBUG=1 trace list

# Format code
cargo fmt

# Lint
cargo clippy
```

## Use Cases

### For AI Agents

- **Long-horizon tasks** - Agents remember work across sessions
- **Complex dependencies** - Track what blocks what
- **Discovery mode** - File issues as you find them
- **Multi-agent** - Coordinate work across multiple agents

### For Humans

- **Personal task management** - CLI-first workflow
- **Developer tools** - Track TODOs with dependencies
- **Project planning** - Organize work hierarchically
- **Research** - Link related ideas and track progress

## Examples

### Create with dependencies

```bash
# Create parent epic
trace create "User authentication" -t epic -p 0

# Create child tasks
trace create "Add login form" -t task -p 1
trace dep add test-2 test-1 --type parent-child

trace create "Add session management" -t task -p 1
trace dep add test-3 test-1 --type parent-child
```

### Block issues

```bash
# Create blocker
trace create "Design authentication system" -t task -p 0

# Block epic
trace dep add test-1 test-4 --type blocks

# Now test-1 won't show in ready work
trace ready  # test-4 appears, test-1 doesn't
```

### Export/Import workflow

```bash
# Export current state
trace export -o backup.jsonl

# Work on another branch
git checkout feature-branch
trace import -i backup.jsonl

# Merge and resolve
git checkout main
git merge feature-branch
# If conflicts in .trace/issues.jsonl, resolve manually
trace import -i .trace/issues.jsonl
```

## Configuration

### Environment Variables

- `TRACE_DB` - Override database path
- `TRACE_ACTOR` - Override actor name for audit trail
- `TRACE_DEBUG` - Enable debug output
- `USER` - Fallback actor name

### Config File (Future)

Configuration via `~/.config/trace/config.toml` (planned).

## FAQ

### Can I use it without AI agents?

Absolutely! Trace is a great CLI task manager with first-class dependency support.

### What about GitHub Issues / Jira / Linear?

Those require internet, have API limits, and aren't designed for agents. Trace:

- Works offline
- No rate limits
- Optimized for CLI and programmatic access
- Distributed via git (no server)

### How do I share with my team?

Just commit `.trace/issues.jsonl` to git. Everyone who clones the repo gets the full database.

### What about conflicts?

Git handles most conflicts automatically (different issues = different lines). For same-issue conflicts, merge manually or let AI help resolve.

## Roadmap

- [ ] Markdown file import for bulk issue creation
- [ ] Custom fields and issue types
- [ ] Query language for complex filters
- [ ] Web UI (optional, for visualization)
- [ ] Integration with GitHub Issues
- [ ] Plugin system for extensions

## Contributing

Contributions welcome! Please:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing`)
5. Open a Pull Request

## License

MIT License - see [LICENSE](LICENSE) for details.

## Credits

- Trace contributors

## Links

- **Repository**: https://github.com/Abil-Shrestha/trace
- **Report issues**: https://github.com/Abil-Shrestha/trace/issues

---

Built with ❤️ in Rust for AI agents everywhere.
