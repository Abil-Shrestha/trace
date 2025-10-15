# Contributing to Trace

Thank you for your interest in contributing to Trace! This document provides guidelines and instructions for contributing.

## Code of Conduct

Be respectful, constructive, and collaborative. We're all here to make Trace better.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
git clone https://github.com/Abil-Shrestha/tracer.git
cd tracer
   ```
3. **Build the project**:
   ```bash
   cargo build
   ```
4. **Run tests**:
   ```bash
   cargo test
   ```

## Development Workflow

### Making Changes

1. **Create a branch** for your feature or bugfix:

   ```bash
   git checkout -b feature/my-amazing-feature
   ```

2. **Make your changes** following the coding standards below

3. **Test your changes**:

   ```bash
   cargo test
   cargo clippy
   cargo fmt --check
   ```

4. **Commit with clear messages**:
   ```bash
   git commit -m "feat: add amazing feature"
   ```

### Commit Message Format

We follow [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` - New features
- `fix:` - Bug fixes
- `docs:` - Documentation changes
- `test:` - Test additions or modifications
- `refactor:` - Code refactoring
- `perf:` - Performance improvements
- `chore:` - Maintenance tasks

Examples:

```
feat: add markdown import for bulk issue creation
fix: resolve database locking issue on concurrent access
docs: update README with performance benchmarks
```

### Pull Request Process

1. **Update documentation** if needed (README, inline docs)
2. **Add tests** for new features
3. **Ensure all tests pass**: `cargo test`
4. **Format code**: `cargo fmt`
5. **Lint code**: `cargo clippy`
6. **Push to your fork**:
   ```bash
   git push origin feature/my-amazing-feature
   ```
7. **Open a Pull Request** on GitHub

#### PR Checklist

- [ ] Code builds without errors
- [ ] All tests pass
- [ ] New tests added for new features
- [ ] Documentation updated
- [ ] Code formatted with `cargo fmt`
- [ ] No new clippy warnings
- [ ] Commit messages follow convention

## Coding Standards

### Rust Style

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for consistent formatting
- Fix all `cargo clippy` warnings
- Write clear, descriptive variable names
- Add inline documentation for public APIs

### Code Organization

```
src/
├── types.rs        # Core data structures
├── storage/        # Storage layer (trait + implementations)
├── cli/            # CLI commands (one file per command group)
├── utils.rs        # Helper functions
├── lib.rs          # Public library API
└── main.rs         # CLI entry point
```

### Documentation

- Add doc comments (`///`) for public functions and types
- Include examples in doc comments when helpful
- Update README.md for user-facing changes
- Add inline comments for complex logic

Example:

````rust
/// Creates a new issue in the database.
///
/// # Arguments
///
/// * `issue` - The issue to create
/// * `actor` - Name of the user/agent creating the issue
///
/// # Example
///
/// ```
/// let issue = Issue { ... };
/// storage.create_issue(&issue, "agent")?;
/// ```
pub fn create_issue(&mut self, issue: &Issue, actor: &str) -> Result<()> {
    // Implementation
}
````

### Testing

- Write unit tests for new functions
- Add integration tests for new commands
- Test edge cases and error conditions
- Use descriptive test names

Example:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_issue_generates_id() {
        let mut storage = SqliteStorage::new(":memory:").unwrap();
        let id = storage.generate_id("test").unwrap();
        assert_eq!(id, "test-1");
    }

    #[test]
    fn test_ready_work_excludes_blocked_issues() {
        // Test implementation
    }
}
```

## Areas for Contribution

### High Priority

- **Markdown import** - Bulk issue creation from markdown files
- **Performance optimization** - SQLite query improvements
- **Windows testing** - Ensure cross-platform compatibility
- **Error handling** - Better error messages and recovery

### Medium Priority

- **Query language** - Advanced filtering syntax
- **Custom fields** - User-defined issue fields
- **Export formats** - CSV, Markdown, etc.
- **Git hooks** - Example scripts for automation

### Low Priority / Future

- **Web UI** - Optional visualization layer
- **Plugin system** - Extensibility framework
- **GitHub integration** - Sync with GitHub Issues
- **TUI** - Terminal UI for interactive use

## Project Architecture

### Storage Layer

The storage layer is abstracted via the `Storage` trait in `src/storage/mod.rs`. Currently, we have:

- `SqliteStorage` - SQLite implementation (main)

Future storage backends could include PostgreSQL, MongoDB, etc.

### CLI Layer

Each command is implemented as a module in `src/cli/`:

- `init.rs` - Database initialization
- `create.rs` - Issue creation
- `list.rs` - List issues
- `show.rs` - Show details
- `update.rs` - Update/close issues
- `ready.rs` - Ready/blocked work
- `dep.rs` - Dependency management
- `export.rs` - Export/import JSONL
- `stats.rs` - Statistics

### Data Flow

1. **CLI** parses arguments via `clap`
2. **Storage** is initialized (auto-discovery or explicit path)
3. **Command** executes via storage trait
4. **Output** formatted as text or JSON
5. **Auto-sync** exports to JSONL if changes made

## Questions?

- Open an issue for questions
- Check existing issues and PRs
- Reach out to maintainers

## Thank You!

Every contribution, no matter how small, makes Trace better for everyone. Thank you for being part of this project!
