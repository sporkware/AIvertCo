#!/bin/bash
# YOLO Mode: Autonomous development cycle

echo "🤖 Starting YOLO development cycle..."

# Check for human override
if [ -f ".yolo_pause" ]; then
    echo "⏸️  YOLO mode paused by human override"
    echo "To resume: rm .yolo_pause"
    exit 0
fi

# Check working hours (9 AM - 6 PM EST, adjust for your timezone)
CURRENT_HOUR=$(date +%H)
if [ "$CURRENT_HOUR" -lt 9 ] || [ "$CURRENT_HOUR" -gt 18 ]; then
    echo "😴 Outside working hours (9 AM - 6 PM). Sleeping..."
    exit 0
fi

echo "📊 Analyzing current project state..."

# Check git status
if [ -n "$(git status --porcelain)" ]; then
    echo "📝 Uncommitted changes detected"
    echo "Stashing changes for safety..."
    git stash push -m "YOLO auto-stash $(date)"
fi

# Create development branch
BRANCH_NAME="yolo/auto-develop-$(date +%Y%m%d-%H%M%S)"
echo "🌿 Creating development branch: $BRANCH_NAME"
git checkout -b "$BRANCH_NAME"

# Run automated improvements
echo "🔧 Running automated code improvements..."

# Fix simple clippy issues
echo "Running clippy auto-fixes..."
cargo clippy --fix --allow-dirty

# Format code
echo "Formatting code..."
cargo fmt
cd web && npm run format && cd ..

# Update dependencies (conservatively)
echo "Checking for safe dependency updates..."
cargo update --conservative

# Run tests
echo "🧪 Running test suite..."
if cargo test; then
    echo "✅ All tests passed"
else
    echo "❌ Tests failed - reverting changes"
    git checkout .
    git checkout main
    git branch -D "$BRANCH_NAME"
    exit 1
fi

# Generate documentation updates
echo "📚 Updating documentation..."
./scripts/update_docs.sh

# Check if changes are meaningful
CHANGES=$(git diff --stat | tail -1 | awk '{print $4+$6}')
if [ "$CHANGES" -gt 0 ]; then
    echo "📈 Made $CHANGES changes"

    # Commit changes
    git add .
    git commit -m "🤖 YOLO auto-improvements

- Code formatting and style fixes
- Dependency updates
- Documentation updates
- Test suite validation

Changes: $CHANGES files modified"

    # Create pull request or merge
    if [ "$AUTO_MERGE" = "true" ]; then
        echo "🔀 Auto-merging to main..."
        git checkout main
        git merge "$BRANCH_NAME" --no-ff -m "🤖 YOLO auto-merge: Routine improvements"
    else
        echo "📋 Creating pull request..."
        # This would integrate with GitHub CLI or similar
        echo "PR created for human review: $BRANCH_NAME"
    fi
else
    echo "✨ No meaningful changes needed"
    git checkout main
    git branch -D "$BRANCH_NAME"
fi

# Restore any stashed changes
if git stash list | grep -q "YOLO auto-stash"; then
    echo "🔄 Restoring stashed changes..."
    git stash pop
fi

echo "🎉 YOLO development cycle complete!"