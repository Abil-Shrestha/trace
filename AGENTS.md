# Tracer for AI Agents

This document guides AI agents (Claude, GPT, Cursor, etc.) on how to use Tracer effectively.

## Quick Start

```bash
# Initialize tracer in your project
tracer init

# Create your first issue
tracer create "Implement user authentication" -p 1 -t feature

# See what's ready to work on
tracer ready --json
```

## Core Workflow for AI Agents

### 1. Find Ready Work

At the start of any session, check for ready work:

```bash
# Get unblocked work in JSON format
tracer ready --json

# Filter by priority
tracer ready --priority 1 --json

# Limit results
tracer ready --limit 5 --json
```

### 2. Claim and Start Work

```bash
# Update issue to in_progress
tracer update <issue-id> --status in_progress --json

# Example:
tracer update bd-1 --status in_progress
```

### 3. Create Issues During Work

As you discover new work, file it immediately:

```bash
# Create a bug you found
tracer create "Fix edge case in validation" -t bug -p 0 --json

# Link it back to parent work with discovered-from
tracer dep add <new-issue-id> <parent-issue-id> --type discovered-from
```

### 4. Add Dependencies

When you realize work blocks other work:

```bash
# bd-2 is blocked by bd-1
tracer dep add bd-2 bd-1 --type blocks

# For parent-child relationships (epics)
tracer dep add bd-3 bd-1 --type parent-child
```

### 5. Complete Work

```bash
# Close the issue
tracer close <issue-id> --reason "Implemented and tested"

# Example with multiple issues
tracer close bd-1 bd-2 bd-3 --reason "All completed"
```

### 6. Check Status

```bash
# View issue details
tracer show <issue-id> --json

# See overall statistics
tracer stats --json

# Check what's blocked
tracer blocked --json
```

## JSON Output Format

All commands support `--json` for programmatic parsing:

### Ready Work Response

```json
[
  {
    "id": "bd-1",
    "title": "Implement feature X",
    "status": "open",
    "priority": 1,
    "issue_type": "feature",
    "created_at": "2025-10-15T10:00:00Z",
    "updated_at": "2025-10-15T10:00:00Z"
  }
]
```

### Issue Creation Response

```json
{
  "id": "bd-42",
  "title": "New issue",
  "status": "open",
  "priority": 2,
  "issue_type": "task",
  "created_at": "2025-10-15T10:30:00Z",
  "updated_at": "2025-10-15T10:30:00Z"
}
```

## Best Practices for AI Agents

### 1. Always Check Ready Work First

Before starting any new work, check what's unblocked:

```bash
WORK=$(tracer ready --limit 1 --json)
if [ "$(echo $WORK | jq length)" -gt 0 ]; then
  ISSUE_ID=$(echo $WORK | jq -r '.[0].id')
  echo "Working on: $ISSUE_ID"
fi
```

### 2. File Issues as You Discover Them

Don't let discovered work get lost:

```bash
# Discovered a bug while working on bd-5
NEW_ID=$(tracer create "Fix null pointer in parser" -t bug -p 0 --json | jq -r '.id')
tracer dep add $NEW_ID bd-5 --type discovered-from
```

### 3. Use Appropriate Dependency Types

- **blocks**: "bd-2 cannot start until bd-1 is done"
- **parent-child**: "bd-2 is a subtask of epic bd-1"
- **discovered-from**: "bd-2 was found while working on bd-1"
- **related**: "bd-2 and bd-1 are connected but don't block"

### 4. Update Status Regularly

Keep the tracker in sync with reality:

```bash
# Starting work
tracer update bd-1 --status in_progress

# Hit a blocker
tracer update bd-1 --status blocked

# Back to work
tracer update bd-1 --status in_progress

# Done
tracer close bd-1 --reason "Completed successfully"
```

### 5. Create Epics for Large Features

Break down complex work:

```bash
# Create epic
EPIC=$(tracer create "User authentication system" -t epic -p 0 --json | jq -r '.id')

# Create child tasks
T1=$(tracer create "Design auth schema" -t task -p 1 --json | jq -r '.id')
tracer dep add $T1 $EPIC --type parent-child

T2=$(tracer create "Implement login endpoint" -t task -p 1 --json | jq -r '.id')
tracer dep add $T2 $EPIC --type parent-child
tracer dep add $T2 $T1 --type blocks  # T2 blocked by T1

T3=$(tracer create "Add session management" -t task -p 1 --json | jq -r '.id')
tracer dep add $T3 $EPIC --type parent-child
tracer dep add $T3 $T1 --type blocks  # T3 blocked by T1
```

## Advanced Usage

### Query and Filter

```bash
# List all open bugs
tracer list --status open --type bug --json

# Find issues by priority
tracer list --priority 0 --json

# Search by assignee
tracer list --assignee "agent-1" --json
```

### Dependency Management

```bash
# View dependency tree
tracer dep tree bd-1 --max-depth 10

# Detect cycles (issues blocking each other)
tracer dep cycles

# Remove a dependency
tracer dep remove bd-2 bd-1
```

### Batch Operations

```bash
# Close multiple issues
tracer close bd-1 bd-2 bd-3 --reason "Sprint complete"

# Create issues with dependencies inline
tracer create "Task B" -p 1 --deps "blocks:bd-1"
```

## Integration Example

Complete agent workflow:

```bash
#!/bin/bash

# 1. Check for ready work
WORK=$(tracer ready --limit 1 --json)

if [ "$(echo $WORK | jq length)" -eq 0 ]; then
  echo "No ready work found"
  exit 0
fi

# 2. Get issue details
ISSUE_ID=$(echo $WORK | jq -r '.[0].id')
ISSUE_TITLE=$(echo $WORK | jq -r '.[0].title')

echo "Working on: $ISSUE_ID - $ISSUE_TITLE"

# 3. Claim the work
tracer update $ISSUE_ID --status in_progress

# 4. Do the work...
# (your implementation here)

# 5. Discover new work during execution
if [ "$FOUND_BUG" = "true" ]; then
  BUG_ID=$(tracer create "Fix discovered issue" -t bug -p 0 --json | jq -r '.id')
  tracer dep add $BUG_ID $ISSUE_ID --type discovered-from
fi

# 6. Complete the work
tracer close $ISSUE_ID --reason "Implemented and tested"

# 7. Show stats
tracer stats
```

## Why Use Trace?

### For Long-Horizon Tasks

Trace helps you maintain context across sessions:

- Your work list persists between conversations
- Dependencies prevent you from forgetting blockers
- Discovered work gets tracked, not lost

### For Multi-Agent Coordination

Multiple agents can work on the same project:

- Git syncs the database across machines
- Ready work detection prevents conflicts
- Audit trail shows who did what

### For Better Organization

Stop using markdown TODOs:

- Proper epics and subtasks
- Dependency tracking
- Priority management
- Status updates

## Troubleshooting

### Database Not Found

```bash
# Ensure tracer is initialized
tracer init

# Or specify database path
tracer --db .trace/myapp.db ready
```

### No Ready Work

```bash
# Check what's blocked
tracer blocked

# View dependency tree to find blockers
tracer dep tree <issue-id>

# Remove incorrect dependencies if needed
tracer dep remove <from-id> <to-id>
```

### Git Conflicts

If `.trace/issues.jsonl` has conflicts after git pull:

1. Resolve the conflict (keep both changes usually)
2. Import: `tracer import -i .trace/issues.jsonl`

## Configuration

### Set Actor Name

```bash
# Via environment variable
export TRACE_ACTOR="agent-name"
tracer create "Task"

# Or via flag
tracer --actor "agent-name" create "Task"
```

### Custom Database Location

```bash
# Via environment variable
export TRACE_DB=/path/to/custom.db
tracer ready

# Or via flag
tracer --db /path/to/custom.db ready
```

## Tips for Claude Code

1. **Start every session with `tracer ready`** to orient yourself
2. **File issues liberally** - better to have too many than lose work
3. **Use JSON output** for parsing: `tracer ready --json | jq`
4. **Update status** so humans can track your progress
5. **Link discovered work** back to parents with `discovered-from`
6. **Check `tracer stats`** periodically to see the big picture

## Example Session

```bash
# Session start - what's ready?
$ tracer ready --limit 3

✓ Ready work: 2 issue(s)

bd-5 Implement user profile page [P1, feature]
  Status: open
  Created: 2025-10-15 10:00 | Updated: 2025-10-15 10:00

bd-8 Fix validation bug [P0, bug]
  Status: open
  Created: 2025-10-15 11:00 | Updated: 2025-10-15 11:00

# Pick the highest priority
$ tracer update bd-8 --status in_progress

# Work on it, discover an issue
$ tracer create "Add tests for validation" -t task -p 1 --deps "discovered-from:bd-8"
✓ Created issue bd-12 Add tests for validation

# Complete the work
$ tracer close bd-8 --reason "Fixed validation and added tests"

# Check stats
$ tracer stats

Issue Statistics

  Total Issues:      12
  Open:              8
  In Progress:       1
  Blocked:           0
  Closed:            3

  Ready to Work:     7

  Avg Lead Time:     2.3 hours
```

Ready to track work like a pro!
