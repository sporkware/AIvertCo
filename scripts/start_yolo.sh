#!/bin/bash
# Start YOLO Mode autonomous operation

echo "🚀 Starting YOLO Mode..."

# Check if already running
if [ -f ".yolo_active" ]; then
    echo "⚠️  YOLO Mode already active"
    echo "To restart: ./scripts/stop_yolo.sh && ./scripts/start_yolo.sh"
    exit 1
fi

# Check for pause override
if [ -f ".yolo_pause" ]; then
    echo "⏸️  YOLO Mode is paused"
    echo "To resume: ./scripts/yolo_resume.sh"
    exit 1
fi

# Validate configuration
if [ ! -f "yolo.toml" ]; then
    echo "❌ YOLO configuration not found (yolo.toml)"
    echo "Run ./scripts/setup_yolo.sh first"
    exit 1
fi

# Create active flag
touch .yolo_active

# Update status
cat > .yolo_status << EOF
YOLO Mode Status
================

Status: ACTIVE
Started: $(date)
Last Run: $(date)
Next Run: Immediate

Configuration: yolo.toml
PID: $$
EOF

echo "$(date): YOLO Mode activated" >> logs/yolo.log

# Start monitoring in background
echo "👀 Starting monitoring..."
./scripts/monitor_yolo.sh &
MONITOR_PID=$!
echo $MONITOR_PID > .yolo_monitor_pid

# Run initial development cycle
echo "🔄 Running initial development cycle..."
./scripts/yolo_develop.sh

# Setup continuous operation (run every 30 minutes during working hours)
echo "⏰ Setting up continuous operation..."
while [ -f ".yolo_active" ] && [ ! -f ".yolo_pause" ]; do
    # Check working hours (9 AM - 6 PM, adjust as needed)
    CURRENT_HOUR=$(date +%H)
    if [ "$CURRENT_HOUR" -ge 9 ] && [ "$CURRENT_HOUR" -le 18 ]; then
        echo "$(date): Running scheduled development cycle..." >> logs/yolo.log
        ./scripts/yolo_develop.sh >> logs/yolo.log 2>&1

        # Update status
        sed -i "s/Last Run:.*/Last Run: $(date)/" .yolo_status
    else
        echo "$(date): Outside working hours - sleeping..." >> logs/yolo.log
    fi

    # Wait 30 minutes before next cycle
    sleep 1800
done

# Cleanup when stopped
echo "$(date): YOLO Mode deactivated" >> logs/yolo.log
rm -f .yolo_active
kill $MONITOR_PID 2>/dev/null
rm -f .yolo_monitor_pid

echo "🛑 YOLO Mode stopped"