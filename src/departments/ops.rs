//! Operations Department - System Operations & Support
//!
//! This module implements the Operations department responsible for:
//! - System administration and maintenance
//! - Incident response and troubleshooting
//! - Customer support and ticket management
//! - Service level agreement (SLA) monitoring
//! - Change management and release coordination
//! - Capacity planning and resource management

use crate::agents::{Agent, AgentTrait, Department};
use crate::communication::{Message, MessageBus, MessagePriority};
use crate::projects::{Project, Task};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Operations Agent specialized in system operations and support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpsAgent {
    /// Base agent properties
    pub agent: Agent,
    /// System administration skill
    pub sysadmin_skill: u8,
    /// Customer support skill
    pub support_skill: u8,
    /// Incident response skill
    pub incident_skill: u8,
    /// Active support tickets
    pub support_tickets: HashMap<Uuid, SupportTicket>,
    /// System incidents
    pub incidents: HashMap<Uuid, Incident>,
    /// SLA tracking
    pub sla_tracking: SLATracking,
    /// Change management queue
    pub change_queue: Vec<ChangeRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportTicket {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub priority: Priority,
    pub status: TicketStatus,
    pub customer_id: Option<String>,
    pub assigned_to: Option<Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub resolution: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Priority {
    Low,
    Normal,
    High,
    Urgent,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TicketStatus {
    Open,
    InProgress,
    PendingCustomer,
    Resolved,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Incident {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub severity: Severity,
    pub status: IncidentStatus,
    pub affected_services: Vec<String>,
    pub root_cause: Option<String>,
    pub resolution: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub resolved_at: Option<chrono::DateTime<chrono::Utc>>,
    pub assigned_team: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Severity {
    Sev1, // Critical - service down
    Sev2, // High - major functionality impacted
    Sev3, // Medium - minor functionality impacted
    Sev4, // Low - cosmetic or informational
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IncidentStatus {
    Open,
    Investigating,
    Mitigating,
    Resolved,
    PostMortem,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SLATracking {
    /// Service Level Agreements
    pub slas: HashMap<String, SLA>,
    /// Current SLA compliance percentages
    pub compliance: HashMap<String, f32>,
    /// SLA violations this month
    pub violations: Vec<SLAViolation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SLA {
    pub service_name: String,
    pub uptime_target: f32, // percentage (e.g., 99.9)
    pub response_time_target: u32, // milliseconds
    pub resolution_time_target: u32, // hours for Sev1 incidents
    pub measurement_period: String, // e.g., "monthly"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SLAViolation {
    pub service: String,
    pub violation_type: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub impact: String,
    pub resolution: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeRequest {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub change_type: ChangeType,
    pub risk_level: RiskLevel,
    pub impact: String,
    pub rollback_plan: String,
    pub scheduled_time: chrono::DateTime<chrono::Utc>,
    pub status: ChangeStatus,
    pub requester: Uuid,
    pub approver: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChangeType {
    Standard,    // Routine, low-risk changes
    Normal,      // Standard changes requiring approval
    Emergency,   // Urgent changes to restore service
    Major,       // High-impact changes requiring CAB approval
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChangeStatus {
    Draft,
    PendingApproval,
    Approved,
    Scheduled,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

impl OpsAgent {
    /// Create a new Operations agent
    pub fn new(name: String, manager_id: Option<Uuid>) -> Self {
        Self {
            agent: Agent::new(name, Department::Ops, manager_id),
            sysadmin_skill: 88,
            support_skill: 85,
            incident_skill: 90,
            support_tickets: HashMap::new(),
            incidents: HashMap::new(),
            sla_tracking: SLATracking::default(),
            change_queue: vec![],
        }
    }

    /// Create a support ticket
    pub async fn create_ticket(&mut self, ticket_request: TicketRequest) -> Result<Uuid, OpsError> {
        let ticket_id = Uuid::new_v4();

        let ticket = SupportTicket {
            id: ticket_id,
            title: ticket_request.title,
            description: ticket_request.description,
            priority: ticket_request.priority,
            status: TicketStatus::Open,
            customer_id: ticket_request.customer_id,
            assigned_to: None, // Will be assigned by routing logic
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            resolution: None,
            tags: ticket_request.tags,
        };

        self.support_tickets.insert(ticket_id, ticket);

        // Auto-assign based on priority and workload
        self.assign_ticket(ticket_id).await?;

        println!("🎫 Ops: Created support ticket '{}' (Priority: {:?})", ticket_request.title, ticket_request.priority);
        Ok(ticket_id)
    }

    /// Declare system incident
    pub async fn declare_incident(&mut self, incident_report: IncidentReport) -> Result<Uuid, OpsError> {
        let incident_id = Uuid::new_v4();

        let incident = Incident {
            id: incident_id,
            title: incident_report.title,
            description: incident_report.description,
            severity: incident_report.severity,
            status: IncidentStatus::Open,
            affected_services: incident_report.affected_services,
            root_cause: None,
            resolution: None,
            created_at: chrono::Utc::now(),
            resolved_at: None,
            assigned_team: None,
        };

        self.incidents.insert(incident_id, incident);

        // Escalate based on severity
        match incident_report.severity {
            Severity::Sev1 => {
                println!("🚨 CRITICAL INCIDENT: {} - Immediate response required!", incident_report.title);
                // Trigger emergency response
            }
            Severity::Sev2 => {
                println!("⚠️ HIGH PRIORITY INCIDENT: {} - Response within 1 hour", incident_report.title);
            }
            _ => {
                println!("📋 INCIDENT: {} - Standard response time", incident_report.title);
            }
        }

        Ok(incident_id)
    }

    /// Update incident status
    pub async fn update_incident(&mut self, incident_id: Uuid, update: IncidentUpdate) -> Result<(), OpsError> {
        if let Some(incident) = self.incidents.get_mut(&incident_id) {
            incident.status = update.status;

            if let Some(root_cause) = update.root_cause {
                incident.root_cause = Some(root_cause);
            }

            if let Some(resolution) = update.resolution {
                incident.resolution = Some(resolution);
                incident.resolved_at = Some(chrono::Utc::now());
            }

            println!("📝 Ops: Updated incident {} - Status: {:?}", incident.title, incident.status);
            Ok(())
        } else {
            Err(OpsError::IncidentNotFound(incident_id))
        }
    }

    /// Submit change request
    pub async fn submit_change_request(&mut self, change_request: ChangeRequest) -> Result<Uuid, OpsError> {
        let change_id = change_request.id;
        self.change_queue.push(change_request);

        println!("📋 Ops: Submitted change request '{}'", self.change_queue.last().unwrap().title);
        Ok(change_id)
    }

    /// Approve change request
    pub async fn approve_change(&mut self, change_id: Uuid, approver: Uuid) -> Result<(), OpsError> {
        if let Some(change) = self.change_queue.iter_mut().find(|c| c.id == change_id) {
            change.status = ChangeStatus::Approved;
            change.approver = Some(approver);

            println!("✅ Ops: Approved change request '{}'", change.title);
            Ok(())
        } else {
            Err(OpsError::ChangeNotFound(change_id))
        }
    }

    /// Monitor SLA compliance
    pub async fn monitor_sla(&mut self) -> Result<(), OpsError> {
        for (service_name, sla) in &self.sla_tracking.slas.clone() {
            // Simulate SLA calculation
            let compliance = 99.0 + rand::random::<f32>() * 2.0; // 99.0-101.0%
            self.sla_tracking.compliance.insert(service_name.clone(), compliance);

            if compliance < sla.uptime_target {
                let violation = SLAViolation {
                    service: service_name.clone(),
                    violation_type: "Uptime Target".to_string(),
                    timestamp: chrono::Utc::now(),
                    impact: format!("Uptime {:.2}% below target {:.2}%", compliance, sla.uptime_target),
                    resolution: None,
                };
                self.sla_tracking.violations.push(violation);

                println!("⚠️ Ops: SLA violation for {} - {:.2}% uptime", service_name, compliance);
            }
        }

        Ok(())
    }

    /// Perform system maintenance
    pub async fn perform_maintenance(&mut self, maintenance_task: MaintenanceTask) -> Result<(), OpsError> {
        println!("🔧 Ops: Starting maintenance task '{}'", maintenance_task.title);

        // Simulate maintenance execution
        match maintenance_task.task_type {
            MaintenanceType::SecurityPatch => {
                println!("🔒 Ops: Applying security patches...");
            }
            MaintenanceType::DatabaseOptimization => {
                println!("🗄️ Ops: Optimizing database performance...");
            }
            MaintenanceType::BackupVerification => {
                println!("💾 Ops: Verifying backup integrity...");
            }
            MaintenanceType::LogRotation => {
                println!("📜 Ops: Rotating system logs...");
            }
        }

        println!("✅ Ops: Maintenance task '{}' completed", maintenance_task.title);
        Ok(())
    }

    /// Generate operations report
    pub async fn generate_report(&self) -> Result<OpsReport, OpsError> {
        let report = OpsReport {
            generated_at: chrono::Utc::now(),
            ticket_summary: TicketSummary {
                total_tickets: self.support_tickets.len() as u32,
                open_tickets: self.support_tickets.values().filter(|t| t.status == TicketStatus::Open).count() as u32,
                resolved_today: 0, // Would calculate from timestamps
                average_resolution_time: 4.2, // hours
            },
            incident_summary: IncidentSummary {
                total_incidents: self.incidents.len() as u32,
                active_incidents: self.incidents.values().filter(|i| i.status != IncidentStatus::Closed).count() as u32,
                sev1_incidents: self.incidents.values().filter(|i| i.severity == Severity::Sev1).count() as u32,
                mttr: 2.5, // hours
            },
            sla_compliance: self.sla_tracking.compliance.clone(),
            upcoming_changes: self.change_queue.iter()
                .filter(|c| c.status == ChangeStatus::Approved)
                .map(|c| c.title.clone())
                .collect(),
        };

        Ok(report)
    }

    /// Auto-assign ticket based on priority and agent workload
    async fn assign_ticket(&mut self, ticket_id: Uuid) -> Result<(), OpsError> {
        if let Some(ticket) = self.support_tickets.get_mut(&ticket_id) {
            // Simple assignment logic - in real system would consider agent skills and workload
            ticket.assigned_to = Some(self.agent.id);
            ticket.status = TicketStatus::InProgress;
        }
        Ok(())
    }
}

#[async_trait]
impl AgentTrait for OpsAgent {
    async fn process_message(&mut self, message: Message) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        match message.message_type.as_str() {
            "create_ticket" => {
                let ticket_request = TicketRequest {
                    title: message.metadata.get("title").unwrap_or(&"Support Request".to_string()).clone(),
                    description: message.content,
                    priority: Priority::Normal,
                    customer_id: message.metadata.get("customer_id").cloned(),
                    tags: vec![],
                };
                self.create_ticket(ticket_request).await?;
            }
            "declare_incident" => {
                let incident_report = IncidentReport {
                    title: message.metadata.get("title").unwrap_or(&"System Incident".to_string()).clone(),
                    description: message.content,
                    severity: Severity::Sev3,
                    affected_services: vec!["unknown".to_string()],
                };
                self.declare_incident(incident_report).await?;
            }
            "sla_check" => {
                self.monitor_sla().await?;
            }
            "maintenance_task" => {
                let maintenance_task = MaintenanceTask {
                    title: message.metadata.get("title").unwrap_or(&"System Maintenance".to_string()).clone(),
                    task_type: MaintenanceType::SecurityPatch,
                    scheduled_time: chrono::Utc::now(),
                    estimated_duration: 30, // minutes
                };
                self.perform_maintenance(maintenance_task).await?;
            }
            "generate_report" => {
                let report = self.generate_report().await?;
                println!("📊 Ops: Generated operations report - {} tickets, {} incidents",
                        report.ticket_summary.total_tickets, report.incident_summary.total_incidents);
            }
            _ => {
                println!("🤷 Ops: Unknown message type: {}", message.message_type);
            }
        }

        Ok(())
    }

    async fn perform_daily_tasks(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("🔧 Ops: Performing daily operations tasks...");

        // SLA monitoring
        self.process_message(Message {
            id: Uuid::new_v4(),
            from_agent: self.agent.id,
            to_agent: self.agent.id,
            message_type: "sla_check".to_string(),
            content: "Daily SLA monitoring".to_string(),
            priority: MessagePriority::Normal,
            timestamp: chrono::Utc::now(),
            metadata: HashMap::new(),
        }).await?;

        // System maintenance
        self.process_message(Message {
            id: Uuid::new_v4(),
            from_agent: self.agent.id,
            to_agent: self.agent.id,
            message_type: "maintenance_task".to_string(),
            content: "Daily system maintenance".to_string(),
            priority: MessagePriority::Normal,
            timestamp: chrono::Utc::now(),
            metadata: HashMap::new(),
        }).await?;

        // Generate daily report
        self.process_message(Message {
            id: Uuid::new_v4(),
            from_agent: self.agent.id,
            to_agent: self.agent.id,
            message_type: "generate_report".to_string(),
            content: "Daily operations report".to_string(),
            priority: MessagePriority::Normal,
            timestamp: chrono::Utc::now(),
            metadata: HashMap::new(),
        }).await?;

        // Close old tickets (simulate automated closure)
        let old_tickets: Vec<Uuid> = self.support_tickets.iter()
            .filter(|(_, ticket)| {
                ticket.status == TicketStatus::Resolved &&
                chrono::Utc::now().signed_duration_since(ticket.updated_at).num_days() > 7
            })
            .map(|(id, _)| *id)
            .collect();

        for ticket_id in old_tickets {
            if let Some(ticket) = self.support_tickets.get_mut(&ticket_id) {
                ticket.status = TicketStatus::Closed;
                println!("🔒 Ops: Auto-closed old ticket '{}'", ticket.title);
            }
        }

        Ok(())
    }

    fn get_agent(&self) -> &Agent {
        &self.agent
    }

    fn get_agent_mut(&mut self) -> &mut Agent {
        &mut self.agent
    }
}

impl Default for SLATracking {
    fn default() -> Self {
        let mut slas = HashMap::new();
        slas.insert("web-service".to_string(), SLA {
            service_name: "web-service".to_string(),
            uptime_target: 99.9,
            response_time_target: 500,
            resolution_time_target: 4,
            measurement_period: "monthly".to_string(),
        });

        Self {
            slas,
            compliance: HashMap::new(),
            violations: vec![],
        }
    }
}

/// Support ticket creation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketRequest {
    pub title: String,
    pub description: String,
    pub priority: Priority,
    pub customer_id: Option<String>,
    pub tags: Vec<String>,
}

/// Incident declaration report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncidentReport {
    pub title: String,
    pub description: String,
    pub severity: Severity,
    pub affected_services: Vec<String>,
}

/// Incident status update
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncidentUpdate {
    pub status: IncidentStatus,
    pub root_cause: Option<String>,
    pub resolution: Option<String>,
}

/// Maintenance task definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceTask {
    pub title: String,
    pub task_type: MaintenanceType,
    pub scheduled_time: chrono::DateTime<chrono::Utc>,
    pub estimated_duration: u32, // minutes
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MaintenanceType {
    SecurityPatch,
    DatabaseOptimization,
    BackupVerification,
    LogRotation,
    SystemUpdate,
}

/// Operations report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpsReport {
    pub generated_at: chrono::DateTime<chrono::Utc>,
    pub ticket_summary: TicketSummary,
    pub incident_summary: IncidentSummary,
    pub sla_compliance: HashMap<String, f32>,
    pub upcoming_changes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketSummary {
    pub total_tickets: u32,
    pub open_tickets: u32,
    pub resolved_today: u32,
    pub average_resolution_time: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncidentSummary {
    pub total_incidents: u32,
    pub active_incidents: u32,
    pub sev1_incidents: u32,
    pub mttr: f32, // Mean Time To Resolution in hours
}

/// Operations-specific errors
#[derive(Debug, thiserror::Error)]
pub enum OpsError {
    #[error("Ticket not found: {0}")]
    TicketNotFound(Uuid),

    #[error("Incident not found: {0}")]
    IncidentNotFound(Uuid),

    #[error("Change request not found: {0}")]
    ChangeNotFound(Uuid),

    #[error("SLA calculation error: {0}")]
    SLACalculationError(String),

    #[error("Maintenance task failed: {0}")]
    MaintenanceFailed(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ops_agent_creation() {
        let agent = OpsAgent::new("Test Ops Agent".to_string(), None);
        assert_eq!(agent.agent.name, "Test Ops Agent");
        assert_eq!(agent.agent.department, Department::Ops);
        assert_eq!(agent.sysadmin_skill, 88);
        assert_eq!(agent.support_skill, 85);
    }

    #[tokio::test]
    async fn test_ticket_creation() {
        let mut agent = OpsAgent::new("Test Agent".to_string(), None);
        let ticket_request = TicketRequest {
            title: "Test Support Ticket".to_string(),
            description: "Test ticket description".to_string(),
            priority: Priority::Normal,
            customer_id: Some("customer123".to_string()),
            tags: vec!["test".to_string()],
        };

        let result = agent.create_ticket(ticket_request).await;
        assert!(result.is_ok());
        assert_eq!(agent.support_tickets.len(), 1);
    }

    #[tokio::test]
    async fn test_incident_declaration() {
        let mut agent = OpsAgent::new("Test Agent".to_string(), None);
        let incident_report = IncidentReport {
            title: "Test System Incident".to_string(),
            description: "Test incident description".to_string(),
            severity: Severity::Sev2,
            affected_services: vec!["web-service".to_string()],
        };

        let result = agent.declare_incident(incident_report).await;
        assert!(result.is_ok());
        assert_eq!(agent.incidents.len(), 1);
    }

    #[tokio::test]
    async fn test_sla_monitoring() {
        let mut agent = OpsAgent::new("Test Agent".to_string(), None);
        let result = agent.monitor_sla().await;
        assert!(result.is_ok());
        assert!(!agent.sla_tracking.compliance.is_empty());
    }

    #[tokio::test]
    async fn test_operations_report() {
        let agent = OpsAgent::new("Test Agent".to_string(), None);
        let result = agent.generate_report().await;
        assert!(result.is_ok());

        let report = result.unwrap();
        assert!(report.generated_at <= chrono::Utc::now());
    }
}