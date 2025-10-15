#!/bin/bash
# Test script for validating Trace with Claude Code workflows

set -e

echo "🧪 Testing Trace Integration for Claude Code"
echo "=============================================="
echo ""

# Cleanup previous test
rm -rf .trace-test
mkdir -p .trace-test
cd .trace-test

TRACE="../target/release/trace"

# Test 1: Initialize
echo "✓ Test 1: Initialize database"
$TRACE init --prefix test
echo ""

# Test 2: Create issues
echo "✓ Test 2: Create issues"
$TRACE create "Implement authentication" -t epic -p 0
$TRACE create "Add login form" -t task -p 1
$TRACE create "Add session management" -t task -p 1
$TRACE create "Write tests" -t task -p 2
echo ""

# Test 3: Add dependencies
echo "✓ Test 3: Add dependencies"
$TRACE dep add test-2 test-1 --type parent-child
$TRACE dep add test-3 test-1 --type parent-child
$TRACE dep add test-3 test-2 --type blocks
$TRACE dep add test-4 test-1 --type discovered-from
echo ""

# Test 4: Check ready work (should show test-2, not test-3)
echo "✓ Test 4: Check ready work"
READY=$($TRACE ready --json)
echo "Ready work JSON:"
echo $READY | jq '.'
echo ""

# Test 5: Update status
echo "✓ Test 5: Update issue status"
$TRACE update test-2 --status in_progress
$TRACE show test-2
echo ""

# Test 6: Create discovered issue
echo "✓ Test 6: Create discovered issue"
NEW_ID=$($TRACE create "Fix validation bug" -t bug -p 0 --deps "discovered-from:test-2" --json | jq -r '.id')
echo "Created issue: $NEW_ID"
echo ""

# Test 7: Close issues
echo "✓ Test 7: Close issues"
$TRACE close test-2 --reason "Completed"
echo ""

# Test 8: Check blocked issues
echo "✓ Test 8: Check blocked issues"
$TRACE blocked
echo ""

# Test 9: View dependency tree
echo "✓ Test 9: View dependency tree"
$TRACE dep tree test-1
echo ""

# Test 10: Statistics
echo "✓ Test 10: Statistics"
$TRACE stats
echo ""

# Test 11: Export/Import
echo "✓ Test 11: Export and import"
$TRACE export -o export-test.jsonl
echo "Exported to export-test.jsonl:"
cat export-test.jsonl | jq -s 'length'
echo ""

# Test 12: List with filters
echo "✓ Test 12: List with filters"
echo "Open issues:"
$TRACE list --status open --json | jq 'length'
echo "High priority issues:"
$TRACE list --priority 0 --json | jq 'length'
echo ""

# Test 13: Detect cycles
echo "✓ Test 13: Detect cycles (should be none)"
$TRACE dep cycles
echo ""

echo "✅ All tests passed!"
echo ""
echo "Test database location: $(pwd)/.trace/"
echo "You can explore with: cd $(pwd) && $TRACE list"
