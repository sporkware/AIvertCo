//! InfoSec Department - Information Security & Cybersecurity
//!
//! This module implements the InfoSec department responsible for:
//! - Security vulnerability assessment and remediation
//! - Access control and authentication management
//! - Threat detection and incident response
//! - Compliance monitoring and reporting
//! - Security policy enforcement
//! - Data protection and encryption

use crate::agents::{Agent, AgentTrait, Department};
use crate::communication::{Message, MessageBus, MessagePriority};
use crate::projects::{Project, Task};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// InfoSec Agent specialized in cybersecurity and threat protection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfoSecAgent {
    /// Base agent properties
    pub agent: Agent,
    /// Cybersecurity expertise level
    pub security_skill: u8,
    /// Threat detection skill
    pub threat_detection_skill: u8,
    /// Incident response skill
    pub incident_response_skill: u8,
    /// Current security posture
    pub security_posture: SecurityPosture,
    /// Active security incidents
    pub active_incidents: HashMap<Uuid, SecurityIncident>,
    /// Security policies and compliance status
    pub compliance_status: ComplianceStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPosture {
    /// Overall security score (0-100)
    pub overall_score: u8,
    /// Vulnerability count by severity
    pub vulnerabilities: VulnerabilityCounts,
    /// Active security controls
    pub active_controls: Vec<SecurityControl>,
    /// Recent security events
    pub recent_events: Vec<SecurityEvent>,
    /// Last assessment date
    pub last_assessment: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VulnerabilityCounts {
    pub critical: u32,
    pub high: u32,
    pub medium: u32,
    pub low: u32,
    pub info: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityControl {
    pub id: String,
    pub name: String,
    pub control_type: ControlType,
    pub status: ControlStatus,
    pub last_check: chrono::DateTime<chrono::Utc>,
    pub effectiveness: u8, // 0-100
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ControlType {
    AccessControl,
    Encryption,
    NetworkSecurity,
    EndpointProtection,
    Monitoring,
    IncidentResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ControlStatus {
    Active,
    Inactive,
    Degraded,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    pub id: Uuid,
    pub event_type: EventType,
    pub severity: Severity,
    pub description: String,
    pub source: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub resolved: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EventType {
    UnauthorizedAccess,
    MalwareDetected,
    DDoSAttack,
    DataBreach,
    PolicyViolation,
    SuspiciousActivity,
    SystemCompromise,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityIncident {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub severity: Severity,
    pub status: IncidentStatus,
    pub assigned_to: Option<Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub resolution_steps: Vec<String>,
    pub affected_systems: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IncidentStatus {
    Open,
    Investigating,
    Mitigating,
    Resolved,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceStatus {
    /// GDPR compliance score
    pub gdpr_compliance: u8,
    /// SOC 2 compliance score
    pub soc2_compliance: u8,
    /// ISO 27001 compliance score
    pub iso27001_compliance: u8,
    /// Last compliance audit
    pub last_audit: chrono::DateTime<chrono::Utc>,
    /// Open compliance issues
    pub open_issues: Vec<ComplianceIssue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceIssue {
    pub id: String,
    pub standard: String,
    pub requirement: String,
    pub severity: Severity,
    pub status: String,
    pub due_date: chrono::DateTime<chrono::Utc>,
}

impl InfoSecAgent {
    /// Create a new InfoSec agent
    pub fn new(name: String, manager_id: Option<Uuid>) -> Self {
        Self {
            agent: Agent::new(name, Department::InfoSec, manager_id),
            security_skill: 95,
            threat_detection_skill: 90,
            incident_response_skill: 85,
            security_posture: SecurityPosture::default(),
            active_incidents: HashMap::new(),
            compliance_status: ComplianceStatus::default(),
        }
    }

    /// Perform security vulnerability scan
    pub async fn perform_vulnerability_scan(&mut self, target: &str) -> Result<ScanResults, InfoSecError> {
        println!("🔍 InfoSec: Starting vulnerability scan on {}", target);

        // Simulate vulnerability scanning
        let vulnerabilities = vec![
            Vulnerability {
                id: format!("CVE-2024-{}", rand::random::<u32>() % 10000),
                title: "Sample Vulnerability".to_string(),
                severity: Severity::Medium,
                cvss_score: 6.5,
                description: "Sample vulnerability description".to_string(),
                affected_system: target.to_string(),
                remediation: "Apply security patch".to_string(),
                discovered_at: chrono::Utc::now(),
            }
        ];

        let results = ScanResults {
            target: target.to_string(),
            scan_start: chrono::Utc::now(),
            scan_end: chrono::Utc::now(),
            vulnerabilities_found: vulnerabilities.len() as u32,
            vulnerabilities,
            scan_status: ScanStatus::Completed,
        };

        // Update security posture
        self.update_security_posture(&results).await?;

        println!("✅ InfoSec: Vulnerability scan completed for {}", target);
        Ok(results)
    }

    /// Handle security incident
    pub async fn handle_incident(&mut self, incident_report: IncidentReport) -> Result<Uuid, InfoSecError> {
        let incident_id = Uuid::new_v4();

        let incident = SecurityIncident {
            id: incident_id,
            title: incident_report.title,
            description: incident_report.description,
            severity: incident_report.severity,
            status: IncidentStatus::Open,
            assigned_to: Some(self.agent.id),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            resolution_steps: vec!["Initial assessment".to_string()],
            affected_systems: incident_report.affected_systems,
        };

        self.active_incidents.insert(incident_id, incident);

        // Escalate critical incidents
        if incident_report.severity == Severity::Critical {
            println!("🚨 CRITICAL: Security incident detected - {}", incident_report.title);
            // Send alerts to management
        } else {
            println!("⚠️ InfoSec: Security incident reported - {}", incident_report.title);
        }

        Ok(incident_id)
    }

    /// Update security controls
    pub async fn update_security_controls(&mut self) -> Result<(), InfoSecError> {
        println!("🔒 InfoSec: Updating security controls...");

        // Ensure all critical controls are active
        let required_controls = vec![
            ("access_control", "Multi-Factor Authentication", ControlType::AccessControl),
            ("encryption", "Data Encryption at Rest", ControlType::Encryption),
            ("firewall", "Network Firewall", ControlType::NetworkSecurity),
            ("monitoring", "Security Information and Event Management", ControlType::Monitoring),
        ];

        for (id, name, control_type) in required_controls {
            if !self.security_posture.active_controls.iter().any(|c| c.id == id) {
                let control = SecurityControl {
                    id: id.to_string(),
                    name: name.to_string(),
                    control_type,
                    status: ControlStatus::Active,
                    last_check: chrono::Utc::now(),
                    effectiveness: 85,
                };
                self.security_posture.active_controls.push(control);
            }
        }

        println!("✅ InfoSec: Security controls updated");
        Ok(())
    }

    /// Perform compliance audit
    pub async fn perform_compliance_audit(&mut self) -> Result<AuditResults, InfoSecError> {
        println!("📋 InfoSec: Performing compliance audit...");

        // Simulate compliance checking
        let gdpr_score = (rand::random::<f32>() * 20.0 + 80.0) as u8;
        let soc2_score = (rand::random::<f32>() * 15.0 + 85.0) as u8;
        let iso_score = (rand::random::<f32>() * 10.0 + 90.0) as u8;

        self.compliance_status.gdpr_compliance = gdpr_score;
        self.compliance_status.soc2_compliance = soc2_score;
        self.compliance_status.iso27001_compliance = iso_score;
        self.compliance_status.last_audit = chrono::Utc::now();

        let results = AuditResults {
            audit_date: chrono::Utc::now(),
            gdpr_compliance: gdpr_score,
            soc2_compliance: soc2_score,
            iso27001_compliance: iso_score,
            overall_compliance: (gdpr_score + soc2_score + iso_score) / 3,
            issues_found: vec![],
            recommendations: vec![
                "Review access control policies".to_string(),
                "Update encryption standards".to_string(),
                "Enhance monitoring capabilities".to_string(),
            ],
        };

        println!("✅ InfoSec: Compliance audit completed - Overall score: {}%", results.overall_compliance);
        Ok(results)
    }

    /// Monitor for threats in real-time
    pub async fn monitor_threats(&mut self) -> Result<Vec<SecurityEvent>, InfoSecError> {
        // Simulate threat detection
        let mut events = Vec::new();

        // Random threat generation for simulation
        if rand::random::<f32>() < 0.1 { // 10% chance of detecting something
            let event_types = vec![
                EventType::SuspiciousActivity,
                EventType::UnauthorizedAccess,
                EventType::MalwareDetected,
            ];

            let event_type = event_types[rand::random::<usize>() % event_types.len()];
            let severity = match rand::random::<f32>() {
                x if x < 0.1 => Severity::Critical,
                x if x < 0.3 => Severity::High,
                x if x < 0.6 => Severity::Medium,
                _ => Severity::Low,
            };

            let event = SecurityEvent {
                id: Uuid::new_v4(),
                event_type: event_type.clone(),
                severity: severity.clone(),
                description: format!("Detected {} with {} severity", format!("{:?}", event_type).to_lowercase(), format!("{:?}", severity).to_lowercase()),
                source: "threat_monitoring_system".to_string(),
                timestamp: chrono::Utc::now(),
                resolved: false,
            };

            events.push(event.clone());
            self.security_posture.recent_events.push(event);
        }

        Ok(events)
    }

    /// Update security posture based on scan results
    async fn update_security_posture(&mut self, scan_results: &ScanResults) -> Result<(), InfoSecError> {
        // Update vulnerability counts
        self.security_posture.vulnerabilities = VulnerabilityCounts {
            critical: scan_results.vulnerabilities.iter().filter(|v| v.severity == Severity::Critical).count() as u32,
            high: scan_results.vulnerabilities.iter().filter(|v| v.severity == Severity::High).count() as u32,
            medium: scan_results.vulnerabilities.iter().filter(|v| v.severity == Severity::Medium).count() as u32,
            low: scan_results.vulnerabilities.iter().filter(|v| v.severity == Severity::Low).count() as u32,
            info: scan_results.vulnerabilities.iter().filter(|v| v.severity == Severity::Info).count() as u32,
        };

        // Calculate overall security score
        let vuln_penalty = (self.security_posture.vulnerabilities.critical * 20 +
                           self.security_posture.vulnerabilities.high * 10 +
                           self.security_posture.vulnerabilities.medium * 5) as i32;

        self.security_posture.overall_score = (100i32 - vuln_penalty).max(0) as u8;
        self.security_posture.last_assessment = chrono::Utc::now();

        Ok(())
    }
}

#[async_trait]
impl AgentTrait for InfoSecAgent {
    async fn process_message(&mut self, message: Message) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        match message.message_type.as_str() {
            "vulnerability_scan" => {
                if let Some(target) = message.metadata.get("target") {
                    self.perform_vulnerability_scan(target).await?;
                }
            }
            "incident_report" => {
                // Parse incident from message content
                let incident_report = IncidentReport {
                    title: message.metadata.get("title").unwrap_or(&"Security Incident".to_string()).clone(),
                    description: message.content,
                    severity: Severity::High, // Default to high for reported incidents
                    affected_systems: vec!["unknown".to_string()], // Would parse from metadata
                };
                self.handle_incident(incident_report).await?;
            }
            "threat_check" => {
                let threats = self.monitor_threats().await?;
                for threat in threats {
                    println!("🚨 InfoSec: Threat detected - {} ({:?})", threat.description, threat.severity);
                }
            }
            "compliance_audit" => {
                self.perform_compliance_audit().await?;
            }
            "security_update" => {
                self.update_security_controls().await?;
            }
            _ => {
                println!("🤷 InfoSec: Unknown message type: {}", message.message_type);
            }
        }

        Ok(())
    }

    async fn perform_daily_tasks(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("🔒 InfoSec: Performing daily security tasks...");

        // Threat monitoring
        self.process_message(Message {
            id: Uuid::new_v4(),
            from_agent: self.agent.id,
            to_agent: self.agent.id,
            message_type: "threat_check".to_string(),
            content: "Daily threat monitoring".to_string(),
            priority: MessagePriority::Normal,
            timestamp: chrono::Utc::now(),
            metadata: HashMap::new(),
        }).await?;

        // Vulnerability scanning
        self.process_message(Message {
            id: Uuid::new_v4(),
            from_agent: self.agent.id,
            to_agent: self.agent.id,
            message_type: "vulnerability_scan".to_string(),
            content: "Daily vulnerability scan".to_string(),
            priority: MessagePriority::Normal,
            timestamp: chrono::Utc::now(),
            metadata: HashMap::new(),
        }).await?;

        // Security control updates
        self.process_message(Message {
            id: Uuid::new_v4(),
            from_agent: self.agent.id,
            to_agent: self.agent.id,
            message_type: "security_update".to_string(),
            content: "Daily security control update".to_string(),
            priority: MessagePriority::Normal,
            timestamp: chrono::Utc::now(),
            metadata: HashMap::new(),
        }).await?;

        // Weekly compliance audit (simplified to daily for demo)
        self.process_message(Message {
            id: Uuid::new_v4(),
            from_agent: self.agent.id,
            to_agent: self.agent.id,
            message_type: "compliance_audit".to_string(),
            content: "Regular compliance audit".to_string(),
            priority: MessagePriority::Normal,
            timestamp: chrono::Utc::now(),
            metadata: HashMap::new(),
        }).await?;

        Ok(())
    }

    fn get_agent(&self) -> &Agent {
        &self.agent
    }

    fn get_agent_mut(&mut self) -> &mut Agent {
        &mut self.agent
    }
}

impl Default for SecurityPosture {
    fn default() -> Self {
        Self {
            overall_score: 85,
            vulnerabilities: VulnerabilityCounts {
                critical: 0,
                high: 2,
                medium: 5,
                low: 12,
                info: 25,
            },
            active_controls: vec![],
            recent_events: vec![],
            last_assessment: chrono::Utc::now(),
        }
    }
}

impl Default for ComplianceStatus {
    fn default() -> Self {
        Self {
            gdpr_compliance: 85,
            soc2_compliance: 88,
            iso27001_compliance: 82,
            last_audit: chrono::Utc::now(),
            open_issues: vec![],
        }
    }
}

/// Results from a vulnerability scan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResults {
    pub target: String,
    pub scan_start: chrono::DateTime<chrono::Utc>,
    pub scan_end: chrono::DateTime<chrono::Utc>,
    pub vulnerabilities_found: u32,
    pub vulnerabilities: Vec<Vulnerability>,
    pub scan_status: ScanStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vulnerability {
    pub id: String,
    pub title: String,
    pub severity: Severity,
    pub cvss_score: f32,
    pub description: String,
    pub affected_system: String,
    pub remediation: String,
    pub discovered_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ScanStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

/// Incident report structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncidentReport {
    pub title: String,
    pub description: String,
    pub severity: Severity,
    pub affected_systems: Vec<String>,
}

/// Compliance audit results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditResults {
    pub audit_date: chrono::DateTime<chrono::Utc>,
    pub gdpr_compliance: u8,
    pub soc2_compliance: u8,
    pub iso27001_compliance: u8,
    pub overall_compliance: u8,
    pub issues_found: Vec<String>,
    pub recommendations: Vec<String>,
}

/// InfoSec-specific errors
#[derive(Debug, thiserror::Error)]
pub enum InfoSecError {
    #[error("Scan failed: {0}")]
    ScanFailed(String),

    #[error("Incident handling failed: {0}")]
    IncidentHandlingFailed(String),

    #[error("Compliance audit failed: {0}")]
    ComplianceAuditFailed(String),

    #[error("Security control error: {0}")]
    SecurityControlError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_infosec_agent_creation() {
        let agent = InfoSecAgent::new("Test InfoSec Agent".to_string(), None);
        assert_eq!(agent.agent.name, "Test InfoSec Agent");
        assert_eq!(agent.agent.department, Department::InfoSec);
        assert_eq!(agent.security_skill, 95);
        assert_eq!(agent.threat_detection_skill, 90);
    }

    #[tokio::test]
    async fn test_vulnerability_scan() {
        let mut agent = InfoSecAgent::new("Test Agent".to_string(), None);
        let result = agent.perform_vulnerability_scan("test-system").await;
        assert!(result.is_ok());

        let scan_results = result.unwrap();
        assert_eq!(scan_results.target, "test-system");
        assert_eq!(scan_results.scan_status, ScanStatus::Completed);
    }

    #[tokio::test]
    async fn test_incident_handling() {
        let mut agent = InfoSecAgent::new("Test Agent".to_string(), None);
        let incident_report = IncidentReport {
            title: "Test Security Incident".to_string(),
            description: "Test incident description".to_string(),
            severity: Severity::High,
            affected_systems: vec!["web-server".to_string()],
        };

        let result = agent.handle_incident(incident_report).await;
        assert!(result.is_ok());
        assert_eq!(agent.active_incidents.len(), 1);
    }

    #[tokio::test]
    async fn test_compliance_audit() {
        let mut agent = InfoSecAgent::new("Test Agent".to_string(), None);
        let result = agent.perform_compliance_audit().await;
        assert!(result.is_ok());

        let audit_results = result.unwrap();
        assert!(audit_results.overall_compliance >= 0);
        assert!(audit_results.overall_compliance <= 100);
    }
}