#!/bin/bash
# YOLO Mode: Comprehensive testing

echo "🧪 Running comprehensive YOLO testing suite..."

FAILED=0

# Function to run test and track failures
run_test() {
    local test_name="$1"
    local command="$2"

    echo "Running $test_name..."
    if eval "$command"; then
        echo "✅ $test_name passed"
        return 0
    else
        echo "❌ $test_name failed"
        FAILED=$((FAILED + 1))
        return 1
    fi
}

# Unit tests
run_test "Rust unit tests" "cargo test --lib --quiet"

# Integration tests
run_test "Rust integration tests" "cargo test --test integration --quiet"

# Documentation tests
run_test "Rust documentation tests" "cargo test --doc --quiet"

# Frontend tests
cd web
run_test "Frontend unit tests" "npm test -- --watchAll=false --passWithNoTests"
cd ..

# Type checking
cd web
run_test "TypeScript type check" "npm run typecheck"
cd ..

# Linting
run_test "Rust linting" "cargo clippy --quiet"
cd web
run_test "JavaScript/TypeScript linting" "npm run lint --silent"
cd ..

# Security audit
run_test "Security audit" "cargo audit --quiet"

# Performance benchmarks
run_test "Performance benchmarks" "cargo bench --quiet"

# Build check
run_test "Full project build" "cargo build --release --quiet"

# Webpack build
cd web
run_test "Frontend build" "npm run build"
cd ..

echo ""
echo "🧪 Testing Summary:"
echo "=================="

if [ $FAILED -eq 0 ]; then
    echo "✅ All tests passed! System is healthy."
    echo "🚀 Ready for deployment."

    # Create test report
    cat > test_report.txt << EOF
YOLO Test Report - $(date)
==========================

✅ All $(( $(grep -c "run_test" "$0") )) test suites passed

System Status: HEALTHY
Deployment Ready: YES

Test Results:
- Unit Tests: PASSED
- Integration Tests: PASSED
- Documentation Tests: PASSED
- Frontend Tests: PASSED
- Type Checking: PASSED
- Linting: PASSED
- Security Audit: PASSED
- Performance: PASSED
- Build: PASSED

EOF

else
    echo "❌ $FAILED test suite(s) failed."
    echo "⚠️  Manual review required before deployment."

    # Create failure report
    cat > test_report.txt << EOF
YOLO Test Report - $(date)
==========================

❌ $FAILED test suite(s) failed

System Status: NEEDS ATTENTION
Deployment Ready: NO

Failed Tests: $FAILED
Total Tests: $(grep -c "run_test" "$0")

Action Required:
- Review test failures
- Fix any critical issues
- Re-run test suite
- Manual deployment approval needed

EOF

    exit 1
fi