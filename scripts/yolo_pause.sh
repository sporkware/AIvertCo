#!/bin/bash
# Pause YOLO Mode

echo "⏸️  Pausing YOLO Mode..."

if [ ! -f ".yolo_active" ]; then
    echo "⚠️  YOLO Mode is not currently active"
    exit 1
fi

# Create pause flag
touch .yolo_pause

# Update status
sed -i 's/State:.*/State: PAUSED (human override)/' .yolo_status

echo "$(date): YOLO Mode paused by user" >> logs/yolo.log
echo "✅ YOLO Mode paused. Autonomous operation suspended."
echo "To resume: ./scripts/yolo_resume.sh"