# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2025-10-15

### Added

- Initial release of Trace
- Core issue tracking functionality
  - Create, list, show, update, close commands
  - Priority levels (0-4)
  - Issue types (bug, feature, task, epic, chore)
  - Labels and assignees
  - Status tracking (open, in_progress, blocked, closed)
- Dependency management
  - Four dependency types: blocks, related, parent-child, discovered-from
  - Dependency tree visualization
  - Cycle detection
  - Ready work detection (issues with no open blockers)
- JSONL export/import
  - Git-friendly one-issue-per-line format
  - Auto-export after changes
  - Auto-import after git pull (hash-based)
  - JSONL format for git-friendly storage
- Database features
  - SQLite storage with automatic migrations
  - Database auto-discovery (.trace/\*.db in current dir or ancestors)
  - Project isolation (each project gets own database)
  - Full audit trail via events table
- CLI features
  - JSON output for all commands (--json flag)
  - Colored terminal output
  - Global flags (--db, --actor)
  - Comprehensive help text
- Performance optimizations
  - Prepared statement caching
  - Efficient dependency graph queries
  - Optimized for speed with Rust
- Complete test coverage for core functionality

### Performance

- Create issue: ~5ms
- List 1000 issues: ~15ms
- Ready work query: ~10ms
- Dependency tree: ~25ms

[Unreleased]: https://github.com/Abil-Shrestha/tracer/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/Abil-Shrestha/trace/releases/tag/v0.1.0
