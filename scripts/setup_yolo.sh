#!/bin/bash
# Setup YOLO Mode for autonomous development

echo "🤖 Setting up YOLO Mode..."

# Make scripts executable
echo "🔧 Making scripts executable..."
chmod +x scripts/*.sh

# Create YOLO configuration
echo "⚙️  Creating YOLO configuration..."
cat > yolo.toml << EOF
# YOLO Mode Configuration

[autonomy]
level = "escalation"  # routine, escalation, human_approval
working_hours_start = "09:00"
working_hours_end = "18:00"
timezone = "America/New_York"

[risk_thresholds]
code_changes_max = 1000  # lines
database_changes = false
external_api_calls = false
security_sensitive = false

[notifications]
email_enabled = true
slack_enabled = false
sms_enabled = false

[monitoring]
health_check_interval = 30  # seconds
metrics_retention = 30      # days
alert_timeout = 300         # seconds

[deployment]
auto_deploy = false  # Set to true for full autonomy
staging_first = true
rollback_on_failure = true
EOF

# Create YOLO status file
cat > .yolo_status << EOF
YOLO Mode Status
================

Status: INACTIVE
Last Run: Never
Next Run: Setup required

Configuration: yolo.toml
Scripts: Ready
Monitoring: Not started

To start: ./scripts/start_yolo.sh
To stop: ./scripts/stop_yolo.sh
EOF

# Create necessary directories
echo "📁 Creating necessary directories..."
mkdir -p logs
mkdir -p backups
mkdir -p reports

# Setup logging
echo "📊 Setting up logging..."
cat > logs/yolo.log << EOF
YOLO Mode Log - $(date)
=======================

$(date): YOLO Mode setup completed
$(date): System ready for autonomous operation
EOF

# Setup monitoring
echo "👀 Setting up monitoring..."
cat > scripts/monitor_yolo.sh << 'EOF'
#!/bin/bash
# YOLO Mode monitoring script

while true; do
    # Check system health
    if [ -f ".yolo_pause" ]; then
        echo "$(date): YOLO paused" >> logs/yolo.log
    elif [ -f ".yolo_active" ]; then
        echo "$(date): YOLO active - monitoring..." >> logs/yolo.log

        # Check for errors in recent logs
        ERROR_COUNT=$(tail -n 100 logs/yolo.log | grep -c "ERROR\|FAILED\|❌")
        if [ "$ERROR_COUNT" -gt 5 ]; then
            echo "$(date): High error rate detected - pausing YOLO" >> logs/yolo.log
            touch .yolo_pause
            echo "High error rate detected. YOLO paused for safety." | mail -s "YOLO Alert" admin@example.com 2>/dev/null || true
        fi
    fi

    sleep 300  # Check every 5 minutes
done
EOF

chmod +x scripts/monitor_yolo.sh

# Setup cron job for automated runs (optional)
echo "⏰ Setting up automated scheduling..."
CRON_JOB="*/30 * * * 1-5 ./scripts/yolo_check_tasks.sh >> logs/yolo.log 2>&1"
echo "Suggested cron job (add to crontab -e):"
echo "$CRON_JOB"

# Create README for YOLO mode
cat > YOLO_README.md << EOF
# YOLO Mode - Autonomous Development

## Quick Start

1. **Start YOLO Mode:**
   \`\`\`bash
   ./scripts/start_yolo.sh
   \`\`\`

2. **Check Status:**
   \`\`\`bash
   ./scripts/yolo_status.sh
   \`\`\`

3. **Pause/Resume:**
   \`\`\`bash
   ./scripts/yolo_pause.sh   # Pause
   ./scripts/yolo_resume.sh  # Resume
   \`\`\`

## Configuration

Edit \`yolo.toml\` to configure:
- Autonomy level (routine/escalation/human_approval)
- Working hours
- Risk thresholds
- Notification preferences

## Safety Features

- **Automatic Pause:** On high error rates
- **Human Override:** Create \`.yolo_pause\` file
- **Gradual Autonomy:** Start with oversight, increase as trust builds
- **Audit Trail:** All actions logged in \`logs/yolo.log\`

## Monitoring

- View logs: \`tail -f logs/yolo.log\`
- Check status: \`cat .yolo_status\`
- View reports: \`ls reports/\`

## Emergency Stop

If something goes wrong:
\`\`\`bash
touch .yolo_pause          # Pause immediately
./scripts/stop_yolo.sh     # Full stop
rm .yolo_active           # Reset status
\`\`\`
EOF

echo "✅ YOLO Mode setup complete!"
echo ""
echo "📋 Next steps:"
echo "1. Review and edit yolo.toml configuration"
echo "2. Test manually: ./scripts/yolo_check_tasks.sh"
echo "3. Start autonomous mode: ./scripts/start_yolo.sh"
echo "4. Monitor progress: tail -f logs/yolo.log"
echo ""
echo "📖 See YOLO_README.md for detailed instructions"