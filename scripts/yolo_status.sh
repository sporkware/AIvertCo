#!/bin/bash
# Check YOLO Mode status

echo "🤖 YOLO Mode Status"
echo "==================="

if [ -f ".yolo_active" ]; then
    echo "✅ Status: ACTIVE"
    if [ -f ".yolo_pause" ]; then
        echo "⏸️  State: PAUSED (human override)"
    else
        echo "▶️  State: RUNNING"
    fi
else
    echo "⏹️  Status: INACTIVE"
fi

echo ""

# Show configuration summary
if [ -f "yolo.toml" ]; then
    echo "⚙️  Configuration:"
    grep -E "^(level|working_hours|auto_deploy)" yolo.toml | sed 's/^/  /'
else
    echo "⚙️  Configuration: Not found (run setup_yolo.sh)"
fi

echo ""

# Show recent activity
if [ -f "logs/yolo.log" ]; then
    echo "📊 Recent Activity:"
    tail -n 5 logs/yolo.log | sed 's/^/  /'
else
    echo "📊 Recent Activity: No log file"
fi

echo ""

# Show pending tasks
if [ -f ".yolo_tasks" ]; then
    echo "📝 Pending Tasks:"
    head -n 10 .yolo_tasks | sed 's/^/  /'
    TASK_COUNT=$(wc -l < .yolo_tasks)
    echo "  (... $TASK_COUNT total tasks)"
else
    echo "📝 Pending Tasks: None"
fi

echo ""

# Show system health
echo "🏥 System Health:"
if [ -f "test_report.txt" ]; then
    if grep -q "Deployment Ready: YES" test_report.txt; then
        echo "  ✅ Tests: Passing"
    else
        echo "  ❌ Tests: Failing"
    fi
else
    echo "  ❓ Tests: Not run recently"
fi

if cargo check --quiet 2>/dev/null; then
    echo "  ✅ Build: OK"
else
    echo "  ❌ Build: Failing"
fi

echo ""

# Show next run time
if [ -f ".yolo_active" ] && [ ! -f ".yolo_pause" ]; then
    CURRENT_HOUR=$(date +%H)
    if [ "$CURRENT_HOUR" -ge 9 ] && [ "$CURRENT_HOUR" -le 18 ]; then
        echo "⏰ Next Cycle: Within next 30 minutes"
    else
        echo "⏰ Next Cycle: Tomorrow at 9 AM"
    fi
else
    echo "⏰ Next Cycle: Manual start required"
fi