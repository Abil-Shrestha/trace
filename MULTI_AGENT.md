# Multi-Agent Coordination

Tracer now supports simple multi-agent coordination through comments and automatic assignee tracking.

## Features

### 1. Commenting

Agents can leave comments on issues to communicate:

```bash
tracer comment bd-1 "Started working on the API"
tracer comment bd-1 "Need help with authentication"
```

Comments are shown in `tracer show`:

```bash
tracer show bd-1

bd-1 Implement auth API
Status: in_progress
Assignee: claude-1

Recent comments:
  cursor-2 (5 min ago): "Need help with authentication"
  claude-1 (15 min ago): "Started working on the API"
```

### 2. Auto-Assign

When an agent changes an issue's status to `in_progress`, they are automatically assigned to it:

```bash
# Agent identifies via --actor flag or $USER
tracer --actor claude-1 update bd-1 --status in_progress

# Now claude-1 is automatically assigned to bd-1
```

### 3. Assignee Visibility

Assignees are shown in all issue listings:

```bash
tracer list --status in_progress

bd-1 Implement auth [in_progress, Assignee: claude-1]
bd-2 Write tests [in_progress, Assignee: cursor-2]
bd-3 Fix bug [in_progress, Assignee: gpt-4]
```

## Multi-Agent Workflow

### Agent 1 (Claude) starts work:
```bash
tracer --actor claude-1 update bd-1 --status in_progress
tracer --actor claude-1 comment bd-1 "Working on JWT implementation"
```

### Agent 2 (Cursor) sees the activity:
```bash
tracer show bd-1
# Sees claude-1 is working on it with recent comment

tracer comment bd-1 "I can help with frontend once API is ready"
```

### Agent 1 completes:
```bash
tracer comment bd-1 "API complete, ready for testing"
tracer close bd-1 --reason "Implementation done"
```

### Agent 2 picks up dependent work:
```bash
tracer ready
# bd-2 is now unblocked

tracer --actor cursor-2 update bd-2 --status in_progress
tracer comment bd-2 "Starting tests for the new API"
```

## Key Points

- **Simple**: No registration, no sessions, just comments and assignee field
- **Actor identification**: Via `--actor` flag or `$TRACE_ACTOR` or `$USER` env var
- **Auto-assign**: Happens automatically when status changes to `in_progress`
- **Communication**: Through comments visible in `tracer show`
- **Coordination**: Agents see who's working on what via assignee field

## Example Session

```bash
# Three agents working together
$ tracer list --status in_progress

bd-1 Implement auth [P1, in_progress, Assignee: claude-1]
bd-2 Write tests [P1, in_progress, Assignee: cursor-2]
bd-3 Fix bug [P0, in_progress, Assignee: gpt-4]

$ tracer show bd-1

bd-1 Implement authentication API
Status: in_progress
Assignee: claude-1

Recent comments:
  cursor-2 (10 min ago): "I can help with frontend once ready"
  claude-1 (20 min ago): "Working on JWT implementation"
  claude-1 (30 min ago): "Started on this issue"
```
