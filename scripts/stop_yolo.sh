#!/bin/bash
# Stop YOLO Mode autonomous operation

echo "🛑 Stopping YOLO Mode..."

# Remove active flag
rm -f .yolo_active

# Kill monitoring process
if [ -f ".yolo_monitor_pid" ]; then
    MONITOR_PID=$(cat .yolo_monitor_pid)
    kill $MONITOR_PID 2>/dev/null
    rm -f .yolo_monitor_pid
    echo "👀 Monitoring stopped"
fi

# Update status
cat > .yolo_status << EOF
YOLO Mode Status
================

Status: STOPPED
Stopped: $(date)
Last Run: $(grep "Last Run:" .yolo_status | cut -d: -f2- || echo "Unknown")

Configuration: yolo.toml
EOF

echo "$(date): YOLO Mode stopped by user" >> logs/yolo.log
echo "✅ YOLO Mode stopped successfully"