#!/bin/bash
# YOLO Mode: Automated deployment

echo "🚀 Starting YOLO deployment process..."

# Check deployment readiness
echo "🔍 Checking deployment readiness..."

# Verify tests passed
if [ ! -f "test_report.txt" ] || ! grep -q "Deployment Ready: YES" test_report.txt; then
    echo "❌ Tests not passed - cannot deploy"
    echo "Run ./scripts/yolo_test.sh first"
    exit 1
fi

# Check for human override
if [ -f ".no_deploy" ]; then
    echo "⏸️  Deployment blocked by human override"
    echo "Remove .no_deploy to enable deployment"
    exit 1
fi

# Check current branch
CURRENT_BRANCH=$(git branch --show-current)
if [ "$CURRENT_BRANCH" != "main" ]; then
    echo "⚠️  Not on main branch (currently on: $CURRENT_BRANCH)"
    echo "Switching to main for deployment..."
    git checkout main
    git pull origin main
fi

# Create deployment tag
VERSION="v$(date +%Y.%m.%d.%H%M%S)"
echo "🏷️  Creating deployment tag: $VERSION"
git tag "$VERSION"
git push origin "$VERSION"

# Build production artifacts
echo "🔨 Building production artifacts..."

# Backend build
echo "Building Rust backend..."
cargo build --release

# Frontend build
echo "Building frontend..."
cd web
npm run build
cd ..

# Create deployment package
echo "📦 Creating deployment package..."
DEPLOY_DIR="deploy_$VERSION"
mkdir -p "$DEPLOY_DIR"

# Copy backend
cp target/release/virtco "$DEPLOY_DIR/" 2>/dev/null || echo "Backend binary not found"

# Copy frontend
cp -r web/build "$DEPLOY_DIR/web" 2>/dev/null || cp -r web/dist "$DEPLOY_DIR/web" 2>/dev/null || echo "Frontend build not found"

# Copy configuration
cp -r config "$DEPLOY_DIR/" 2>/dev/null || echo "No config directory"

# Copy deployment scripts
cp scripts/deploy_* "$DEPLOY_DIR/" 2>/dev/null || echo "No deployment scripts"

# Create deployment archive
echo "📁 Creating deployment archive..."
tar -czf "${DEPLOY_DIR}.tar.gz" "$DEPLOY_DIR"
rm -rf "$DEPLOY_DIR"

# Deploy based on environment
if [ "$DEPLOY_ENV" = "staging" ]; then
    echo "🧪 Deploying to staging environment..."
    ./scripts/deploy_staging.sh "${DEPLOY_DIR}.tar.gz"
elif [ "$DEPLOY_ENV" = "production" ]; then
    echo "🌟 Deploying to production environment..."
    ./scripts/deploy_production.sh "${DEPLOY_DIR}.tar.gz"
else
    echo "🤔 No deployment environment specified"
    echo "Set DEPLOY_ENV=staging or DEPLOY_ENV=production"
    echo "Deployment package created: ${DEPLOY_DIR}.tar.gz"
fi

# Run post-deployment tests
echo "🧪 Running post-deployment verification..."
if ./scripts/smoke_tests.sh; then
    echo "✅ Deployment successful!"

    # Create deployment report
    cat > deployment_report.txt << EOF
YOLO Deployment Report - $(date)
================================

✅ Deployment Successful

Version: $VERSION
Environment: ${DEPLOY_ENV:-local}
Package: ${DEPLOY_DIR}.tar.gz

Post-deployment Tests: PASSED

Next Steps:
- Monitor system health
- Check application logs
- Verify user access
- Plan next development cycle

EOF

else
    echo "❌ Post-deployment tests failed!"
    echo "🔄 Rolling back deployment..."

    # Trigger rollback
    ./scripts/rollback.sh "$VERSION"

    # Create failure report
    cat > deployment_report.txt << EOF
YOLO Deployment Report - $(date)
================================

❌ Deployment Failed

Version: $VERSION
Environment: ${DEPLOY_ENV:-local}

Post-deployment Tests: FAILED

Action Taken:
- Automatic rollback initiated
- System restored to previous version
- Manual investigation required

EOF

    exit 1
fi

echo "🎊 Deployment process complete!"