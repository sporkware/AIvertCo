#!/bin/bash
# YOLO Mode: Check for pending development tasks

echo "🔍 Checking for pending tasks..."

# Check for TODO comments in code
echo "Checking for TODO comments..."
TODO_COUNT=$(grep -r "TODO\|FIXME\|XXX" src/ web/ --include="*.rs" --include="*.ts" --include="*.tsx" | wc -l)
echo "Found $TODO_COUNT TODO items"

# Check for failing tests
echo "Checking test status..."
if cargo test --quiet 2>/dev/null; then
    echo "✅ All tests passing"
else
    echo "❌ Tests failing - needs attention"
    exit 1
fi

# Check for linting issues
echo "Checking code quality..."
if cargo clippy --quiet && (cd web && npm run lint --silent); then
    echo "✅ Code quality checks passed"
else
    echo "❌ Code quality issues found"
    exit 1
fi

# Check for outdated dependencies
echo "Checking dependencies..."
OUTDATED=$(cargo outdated --quiet 2>/dev/null | wc -l)
if [ "$OUTDATED" -gt 0 ]; then
    echo "⚠️  $OUTDATED dependencies may need updating"
fi

# Generate task list
echo "Generating task recommendations..."
cat > .yolo_tasks << EOF
# YOLO Mode Task List - $(date)

## Code Quality Tasks
$(if [ "$TODO_COUNT" -gt 0 ]; then echo "- Address $TODO_COUNT TODO/FIXME items"; fi)
$(if ! cargo clippy --quiet 2>/dev/null; then echo "- Fix clippy warnings"; fi)
$(if ! (cd web && npm run lint --silent 2>/dev/null); then echo "- Fix ESLint issues"; fi)

## Development Tasks
- Run comprehensive test suite
- Update documentation
- Check for security vulnerabilities
- Optimize performance bottlenecks

## Deployment Tasks
- Prepare for next release
- Update deployment scripts
- Review system monitoring

EOF

echo "📝 Task list generated in .yolo_tasks"
echo "✅ Task check complete"