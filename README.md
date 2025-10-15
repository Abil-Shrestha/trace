# Tracer ⚡

> **Blazing-fast issue tracker for AI agents**

A lightweight, dependency-aware issue tracker designed for AI coding agents. Track work, manage dependencies, and discover ready tasks—all from the CLI.

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)

## 🤔 What is Tracer?

Imagine you're working on a project with multiple tasks, some blocking others. **Tracer helps you:**

- 📝 **Track all your tasks** in one place (no more scattered TODOs)
- 🔗 **Link dependencies** - "Task B can't start until Task A is done"
- ✅ **See what's ready** - Instantly find work you can start right now
- 🤖 **Perfect for AI agents** - They can track their own work across sessions
- 📦 **Git-friendly** - Everything stored as simple JSON, syncs via git

**Real Example:** You're building a login system. You need to design the database schema first, then implement the API, then build the UI. Tracer tracks these dependencies and always shows you what's actually ready to work on.

## 🚀 Quick Install

```bash
cargo install --git https://github.com/Abil-Shrestha/tracer
```

**Prerequisites:** [Rust toolchain](https://rustup.rs/) (installs in ~30 seconds)

**💡 Tip:** Use `tr` as a shorthand - both `tracer` and `tr` work identically!

## ⚡ Quick Start (2 minutes)

```bash
# 1. Initialize (do this once per project)
tracer init

# 2. Create your first task
tracer create "Build login page"
# ✓ Created issue test-1

# 3. See all your tasks
tracer list
# test-1  Build login page  [P2, task, open]

# 4. Mark it as in progress
tracer update test-1 --status in_progress

# 5. When done, close it
tracer close test-1 --reason "Login page complete"
```

**That's it!** You're now tracking work with Tracer. 🎉

### Want to Learn More?

```bash
tracer learn  # Interactive tutorial showing all features
```

**👉 [Full documentation](./QUICK_START.md) | [AI agent guide](./CLAUDE.md)**

## 🎯 Example Workflow (with Dependencies)

Once you're comfortable with basics, try managing dependencies:

```bash
# Create an epic (big feature)
tracer create "User authentication system" -t epic

# Create subtasks
tracer create "Design database schema" -t task
tracer create "Build login API" -t task
tracer create "Create login UI" -t task

# Link them: API depends on schema being done
tracer dep add test-3 test-2 --type blocks

# See what you can work on RIGHT NOW
tracer ready
# → Shows test-2 (Design database schema)
# → Hides test-3 (blocked by test-2)

# Start working on what's ready
tracer update test-2 --status in_progress
```

**The power:** Tracer automatically figures out what's ready to work on based on your dependencies!

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
tracer create "Title" [-p priority] [-t type]
tracer list [--status open] [--priority 1]
tracer show <id>
tracer update <id> --status in_progress
tracer close <id> --reason "Done"

# Dependencies
tracer dep add <from> <to> --type blocks
tracer dep tree <id>

# Find work
tracer ready [--limit 5]
tracer blocked

# Data management
tracer export [-o file.jsonl]
tracer stats
```

**💡 Tip:** Add `--json` to any command for programmatic parsing. Use `tr` instead of `tracer` for faster typing!

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
git clone https://github.com/Abil-Shrestha/tracer.git
cd tracer
cargo install --path .

# Verify installation
tracer --version
tr --version  # Both commands work!

# Using cargo (once published)
cargo install tracer
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

**[⭐ Star on GitHub](https://github.com/Abil-Shrestha/tracer)** • **[Report Issues](https://github.com/Abil-Shrestha/tracer/issues)** • Built with Rust 🦀
