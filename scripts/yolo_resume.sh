#!/bin/bash
# Resume YOLO Mode

echo "▶️  Resuming YOLO Mode..."

if [ ! -f ".yolo_active" ]; then
    echo "⚠️  YOLO Mode is not active (start it first)"
    echo "Run: ./scripts/start_yolo.sh"
    exit 1
fi

if [ ! -f ".yolo_pause" ]; then
    echo "⚠️  YOLO Mode is not paused"
    exit 1
fi

# Remove pause flag
rm -f .yolo_pause

# Update status
sed -i 's/State:.*/State: RUNNING/' .yolo_status

echo "$(date): YOLO Mode resumed by user" >> logs/yolo.log
echo "✅ YOLO Mode resumed. Autonomous operation active."