# Trace ⚡

> **Blazing-fast issue tracker for AI agents**

A lightweight, dependency-aware issue tracker designed for AI coding agents. Track work, manage dependencies, and discover ready tasks—all from the CLI.

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)

## 🚀 Quick Install

```bash
cargo install --git https://github.com/Abil-Shrestha/trace
```

**Prerequisites:** [Rust toolchain](https://rustup.rs/) (installs in ~30 seconds)

## ⚡ Quick Start

```bash
# Initialize in your project
cd ~/myproject
trace init

# Create issues
trace create "Fix authentication bug" -p 1 -t bug
trace create "Add login form" -p 1 -t task

# Add dependencies (task-2 blocked by task-1)
trace dep add test-2 test-1 --type blocks

# See what's ready to work on
trace ready

# Start working
trace update test-1 --status in_progress

# Complete work
trace close test-1 --reason "Fixed and tested"
```

**👉 [Full documentation](./QUICK_START.md) | [AI agent guide](./CLAUDE.md)**

## Why Trace?

- ⚡ **Fast** - ~5ms per operation, built in Rust
- 🤖 **AI-friendly** - JSON output, CLI-first, programmatic workflows  
- 🔗 **Smart dependencies** - Track what blocks what, discover ready work
- 📦 **Git-native** - JSONL storage, no server needed
- 🌍 **Distributed** - Share work across agents via git
- 💾 **Audit trail** - Every change logged with context

## Key Features

**Dependency Tracking**
- Four types: `blocks`, `parent-child`, `discovered-from`, `related`
- Automatic ready work detection (finds unblocked tasks)
- Dependency trees and cycle detection

**AI Agent Integration**
- `--json` flag on all commands for programmatic parsing
- Auto-export to JSONL after changes
- Auto-import after git pull (hash-based)

**Git-Based Storage**
- One JSON line per issue = clean git diffs
- Commit `.trace/issues.jsonl` to version control
- Clone repo = clone full database

**Multi-Project Support**
- Auto-discovers database (like git)
- Works from any subdirectory
- Project-local isolation

## Common Commands

```bash
# Core operations
trace create "Title" [-p priority] [-t type]
trace list [--status open] [--priority 1]
trace show <id>
trace update <id> --status in_progress
trace close <id> --reason "Done"

# Dependencies
trace dep add <from> <to> --type blocks
trace dep tree <id>

# Find work
trace ready [--limit 5]
trace blocked

# Data management
trace export [-o file.jsonl]
trace stats
```

**💡 Tip:** Add `--json` to any command for programmatic parsing

## Performance

Fast enough to never slow you down:

| Operation        | Time  |
| ---------------- | ----- |
| Create issue     | ~5ms  |
| List 1000 issues | ~15ms |
| Ready work query | ~10ms |

_Benchmarks on M1 MacBook Pro with 10,000 issues_

## Installation (Alternative Methods)

```bash
# From source
git clone https://github.com/Abil-Shrestha/trace.git
cd trace
cargo install --path .

# Using cargo (once published)
cargo install trace-tracker
```

## Documentation

- **[Quick Start Guide](./QUICK_START.md)** - Get started in 5 minutes
- **[AI Agent Guide](./CLAUDE.md)** - Integration with Claude and other LLMs
- **[Contributing](./CONTRIBUTING.md)** - Help improve Trace
- **[Installation](./INSTALL.md)** - Detailed installation instructions

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

## License

MIT License - see [LICENSE](LICENSE) for details.

---

**[⭐ Star on GitHub](https://github.com/Abil-Shrestha/trace)** • **[Report Issues](https://github.com/Abil-Shrestha/trace/issues)** • Built with Rust 🦀
