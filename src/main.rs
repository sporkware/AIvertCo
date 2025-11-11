//! AI Company Simulation - Main Entry Point
//!
//! This is the main entry point for the AI Company simulation system.
//! It orchestrates all departments and agents to create a fully autonomous
//! company simulation where AI agents work together to deliver projects,
//! maintain infrastructure, ensure security, and provide customer support.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

mod agents;
mod communication;
mod departments;
mod projects;

use agents::{Agent, AgentTrait, Department};
use communication::{Message, MessageBus, MessagePriority};
use departments::devops::DevOpsAgent;
use departments::infosec::InfoSecAgent;
use departments::networking::NetworkingAgent;
use departments::ops::OpsAgent;

/// Main simulation orchestrator
#[derive(Debug)]
struct CompanySimulation {
    /// All agents in the company
    agents: HashMap<Uuid, Box<dyn AgentTrait>>,
    /// Message bus for inter-agent communication
    message_bus: Arc<MessageBus>,
    /// Active projects
    projects: HashMap<Uuid, projects::Project>,
    /// Simulation configuration
    config: SimulationConfig,
}

#[derive(Debug)]
struct SimulationConfig {
    /// Simulation speed multiplier (1.0 = real-time)
    speed_multiplier: f32,
    /// Enable autonomous operation
    autonomous_mode: bool,
    /// Working hours (start, end)
    working_hours: (u8, u8),
    /// Maximum simulation steps
    max_steps: Option<u64>,
}

impl CompanySimulation {
    /// Create a new company simulation
    async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let message_bus = Arc::new(MessageBus::new().await?);

        let mut simulation = Self {
            agents: HashMap::new(),
            message_bus: message_bus.clone(),
            projects: HashMap::new(),
            config: SimulationConfig {
                speed_multiplier: 1.0,
                autonomous_mode: true,
                working_hours: (9, 18), // 9 AM to 6 PM
                max_steps: None,
            },
        };

        // Initialize all departments
        simulation.initialize_departments().await?;

        Ok(simulation)
    }

    /// Initialize all company departments and agents
    async fn initialize_departments(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🏢 Initializing AI Company Departments...");

        // Create department managers
        let engineering_manager = self.create_agent(Department::Engineering, "Sarah Chen", None).await?;
        let sales_manager = self.create_agent(Department::Sales, "Mike Rodriguez", None).await?;
        let devops_manager = self.create_agent(Department::DevOps, "Jordan Smith", None).await?;
        let infosec_manager = self.create_agent(Department::InfoSec, "Alex Thompson", None).await?;
        let networking_manager = self.create_agent(Department::Networking, "Lisa Park", None).await?;
        let ops_manager = self.create_agent(Department::Ops, "David Wilson", None).await?;

        // Create specialized agents for each department
        self.create_department_agents(Department::DevOps, devops_manager, 3).await?;
        self.create_department_agents(Department::InfoSec, infosec_manager, 2).await?;
        self.create_department_agents(Department::Networking, networking_manager, 2).await?;
        self.create_department_agents(Department::Ops, ops_manager, 3).await?;

        println!("✅ All departments initialized with {} agents", self.agents.len());
        Ok(())
    }

    /// Create an agent for a specific department
    async fn create_agent(&mut self, department: Department, name: &str, manager_id: Option<Uuid>) -> Result<Uuid, Box<dyn std::error::Error>> {
        let agent_id = Uuid::new_v4();

        let agent: Box<dyn AgentTrait> = match department {
            Department::DevOps => Box::new(DevOpsAgent::new(name.to_string(), manager_id)),
            Department::InfoSec => Box::new(InfoSecAgent::new(name.to_string(), manager_id)),
            Department::Networking => Box::new(NetworkingAgent::new(name.to_string(), manager_id)),
            Department::Ops => Box::new(OpsAgent::new(name.to_string(), manager_id)),
            _ => {
                // For other departments, create a basic agent (would be expanded)
                Box::new(DevOpsAgent::new(name.to_string(), manager_id)) // Placeholder
            }
        };

        self.agents.insert(agent_id, agent);
        println!("👤 Created {} agent: {}", department.as_str(), name);

        Ok(agent_id)
    }

    /// Create multiple agents for a department
    async fn create_department_agents(&mut self, department: Department, manager_id: Uuid, count: usize) -> Result<(), Box<dyn std::error::Error>> {
        for i in 1..=count {
            let name = format!("{} Agent {}", department.as_str(), i);
            self.create_agent(department, &name, Some(manager_id)).await?;
        }
        Ok(())
    }

    /// Run the company simulation
    async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🚀 Starting AI Company Simulation...");
        println!("📊 {} agents across {} departments", self.agents.len(), 6);
        println!("⚙️  Simulation speed: {:.1}x", self.config.speed_multiplier);
        println!("🤖 Autonomous mode: {}", if self.config.autonomous_mode { "ENABLED" } else { "DISABLED" });

        let mut step_count = 0u64;

        loop {
            step_count += 1;
            println!("\n--- Simulation Step {} ---", step_count);

            // Check if we've reached max steps
            if let Some(max) = self.config.max_steps {
                if step_count >= max {
                    println!("🏁 Reached maximum simulation steps ({})", max);
                    break;
                }
            }

            // Check working hours
            let current_hour = chrono::Utc::now().hour() as u8;
            let (start_hour, end_hour) = self.config.working_hours;

            if current_hour < start_hour || current_hour >= end_hour {
                println!("😴 Outside working hours ({}-{}). Agents resting...", start_hour, end_hour);
                tokio::time::sleep(tokio::time::Duration::from_secs(300)).await; // Sleep 5 minutes
                continue;
            }

            // Run simulation step
            self.run_simulation_step().await?;

            // Sleep between steps (scaled by speed multiplier)
            let sleep_duration = (60.0 / self.config.speed_multiplier) as u64; // Base 1 minute
            tokio::time::sleep(tokio::time::Duration::from_secs(sleep_duration)).await;
        }

        println!("🏁 Simulation completed after {} steps", step_count);
        Ok(())
    }

    /// Execute one simulation step
    async fn run_simulation_step(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Process agent activities
        self.process_agent_activities().await?;

        // Handle inter-agent communication
        self.process_messages().await?;

        // Check for new projects or tasks
        self.generate_company_activities().await?;

        // Monitor system health
        self.monitor_system_health().await?;

        Ok(())
    }

    /// Process activities for all agents
    async fn process_agent_activities(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let agent_ids: Vec<Uuid> = self.agents.keys().cloned().collect();

        for agent_id in agent_ids {
            if let Some(agent) = self.agents.get_mut(&agent_id) {
                let agent_name = agent.get_agent().name.clone();
                let department = agent.get_agent().department.as_str();

                // Simulate agent activity
                match agent.get_agent().department {
                    Department::DevOps => {
                        // DevOps agents perform infrastructure tasks
                        if rand::random::<f32>() < 0.3 { // 30% chance
                            println!("🔧 {} (DevOps): Performing infrastructure maintenance", agent_name);
                        }
                    }
                    Department::InfoSec => {
                        // InfoSec agents monitor security
                        if rand::random::<f32>() < 0.2 { // 20% chance
                            println!("🔒 {} (InfoSec): Conducting security scan", agent_name);
                        }
                    }
                    Department::Networking => {
                        // Networking agents optimize network
                        if rand::random::<f32>() < 0.25 { // 25% chance
                            println!("🌐 {} (Networking): Optimizing network performance", agent_name);
                        }
                    }
                    Department::Ops => {
                        // Ops agents handle support
                        if rand::random::<f32>() < 0.4 { // 40% chance
                            println!("🎫 {} (Ops): Processing support tickets", agent_name);
                        }
                    }
                    _ => {}
                }

                // Run daily tasks (simplified - would run less frequently in real system)
                if rand::random::<f32>() < 0.1 { // 10% chance per step
                    agent.perform_daily_tasks().await?;
                }
            }
        }

        Ok(())
    }

    /// Process inter-agent messages
    async fn process_messages(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Simulate occasional inter-agent communication
        if rand::random::<f32>() < 0.15 { // 15% chance per step
            let agent_ids: Vec<Uuid> = self.agents.keys().cloned().collect();
            if agent_ids.len() >= 2 {
                let sender_idx = rand::random::<usize>() % agent_ids.len();
                let mut receiver_idx = rand::random::<usize>() % agent_ids.len();
                while receiver_idx == sender_idx {
                    receiver_idx = rand::random::<usize>() % agent_ids.len();
                }

                let sender_id = agent_ids[sender_idx];
                let receiver_id = agent_ids[receiver_idx];

                if let Some(sender) = self.agents.get(&sender_id) {
                    let message_types = vec![
                        "status_update",
                        "collaboration_request",
                        "issue_report",
                        "resource_request",
                    ];

                    let message_type = message_types[rand::random::<usize>() % message_types.len()];

                    let message = Message {
                        id: Uuid::new_v4(),
                        from_agent: sender_id,
                        to_agent: receiver_id,
                        message_type: message_type.to_string(),
                        content: format!("Automated {} from {} department",
                                       message_type.replace("_", " "),
                                       sender.get_agent().department.as_str()),
                        priority: MessagePriority::Normal,
                        timestamp: chrono::Utc::now(),
                        metadata: HashMap::new(),
                    };

                    // Send message through bus
                    self.message_bus.send_message(message.clone()).await?;

                    println!("💬 {} → {}: {}",
                           sender.get_agent().department.as_str(),
                           self.agents.get(&receiver_id).unwrap().get_agent().department.as_str(),
                           message.content);
                }
            }
        }

        Ok(())
    }

    /// Generate company activities (projects, incidents, etc.)
    async fn generate_company_activities(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Simulate random company events
        let event_roll = rand::random::<f32>();

        if event_roll < 0.05 { // 5% chance - new customer project
            let project_id = Uuid::new_v4();
            println!("📋 New customer project received: {}", project_id.simple());

            // Assign to engineering and ops
            self.assign_project_task(project_id, Department::Engineering).await?;
            self.assign_project_task(project_id, Department::Ops).await?;

        } else if event_roll < 0.08 { // 3% chance - security incident
            println!("🚨 Security incident detected!");
            self.handle_security_incident().await?;

        } else if event_roll < 0.12 { // 4% chance - infrastructure issue
            println!("⚠️ Infrastructure issue detected!");
            self.handle_infrastructure_issue().await?;

        } else if event_roll < 0.18 { // 6% chance - customer support request
            println!("🎫 Customer support request received!");
            self.handle_customer_request().await?;
        }

        Ok(())
    }

    /// Assign project task to department
    async fn assign_project_task(&mut self, project_id: Uuid, department: Department) -> Result<(), Box<dyn std::error::Error>> {
        // Find an agent in the department
        for agent in self.agents.values() {
            if agent.get_agent().department == department {
                let message = Message {
                    id: Uuid::new_v4(),
                    from_agent: Uuid::nil(), // System message
                    to_agent: agent.get_agent().id,
                    message_type: "project_assignment".to_string(),
                    content: format!("Assigned to project {}", project_id.simple()),
                    priority: MessagePriority::Normal,
                    timestamp: chrono::Utc::now(),
                    metadata: HashMap::from([
                        ("project_id".to_string(), project_id.to_string()),
                    ]),
                };

                self.message_bus.send_message(message).await?;
                break;
            }
        }

        Ok(())
    }

    /// Handle security incident
    async fn handle_security_incident(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Notify InfoSec agents
        for agent in self.agents.values() {
            if agent.get_agent().department == Department::InfoSec {
                let message = Message {
                    id: Uuid::new_v4(),
                    from_agent: Uuid::nil(),
                    to_agent: agent.get_agent().id,
                    message_type: "declare_incident".to_string(),
                    content: "Security incident: Suspicious activity detected on customer portal",
                    priority: MessagePriority::High,
                    timestamp: chrono::Utc::now(),
                    metadata: HashMap::from([
                        ("title".to_string(), "Security Incident - Suspicious Activity".to_string()),
                        ("severity".to_string(), "Sev2".to_string()),
                    ]),
                };

                self.message_bus.send_message(message).await?;
                break; // Notify first InfoSec agent
            }
        }

        Ok(())
    }

    /// Handle infrastructure issue
    async fn handle_infrastructure_issue(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Notify DevOps agents
        for agent in self.agents.values() {
            if agent.get_agent().department == Department::DevOps {
                let message = Message {
                    id: Uuid::new_v4(),
                    from_agent: Uuid::nil(),
                    to_agent: agent.get_agent().id,
                    message_type: "infrastructure_alert".to_string(),
                    content: "High CPU usage detected on web servers",
                    priority: MessagePriority::High,
                    timestamp: chrono::Utc::now(),
                    metadata: HashMap::new(),
                };

                self.message_bus.send_message(message).await?;
                break; // Notify first DevOps agent
            }
        }

        Ok(())
    }

    /// Handle customer request
    async fn handle_customer_request(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Notify Ops agents
        for agent in self.agents.values() {
            if agent.get_agent().department == Department::Ops {
                let message = Message {
                    id: Uuid::new_v4(),
                    from_agent: Uuid::nil(),
                    to_agent: agent.get_agent().id,
                    message_type: "create_ticket".to_string(),
                    content: "Customer reports website loading slowly",
                    priority: MessagePriority::Normal,
                    timestamp: chrono::Utc::now(),
                    metadata: HashMap::from([
                        ("title".to_string(), "Website Performance Issue".to_string()),
                        ("priority".to_string(), "Normal".to_string()),
                        ("customer_id".to_string(), format!("cust-{}", rand::random::<u32>())),
                    ]),
                };

                self.message_bus.send_message(message).await?;
                break; // Notify first Ops agent
            }
        }

        Ok(())
    }

    /// Monitor overall system health
    async fn monitor_system_health(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Periodic health check
        if rand::random::<f32>() < 0.1 { // 10% chance per step
            let total_agents = self.agents.len();
            let active_projects = self.projects.len();

            println!("🏥 System Health Check:");
            println!("   👥 Total Agents: {}", total_agents);
            println!("   📋 Active Projects: {}", active_projects);
            println!("   ✅ All systems operational");

            // Check agent status
            let mut department_counts = HashMap::new();
            for agent in self.agents.values() {
                let dept = agent.get_agent().department.as_str();
                *department_counts.entry(dept).or_insert(0) += 1;
            }

            println!("   📊 Department Distribution:");
            for (dept, count) in department_counts {
                println!("      {}: {} agents", dept, count);
            }
        }

        Ok(())
    }
}

impl Department {
    /// Convert department to string
    fn as_str(&self) -> &'static str {
        match self {
            Department::Engineering => "Engineering",
            Department::Sales => "Sales",
            Department::DevOps => "DevOps",
            Department::InfoSec => "InfoSec",
            Department::Networking => "Networking",
            Department::Ops => "Ops",
            Department::Marketing => "Marketing",
            Department::Finance => "Finance",
            Department::HR => "HR",
            Department::Legal => "Legal",
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 AI Company Simulation v0.1.0");
    println!("==================================");

    // Initialize the company simulation
    let mut simulation = CompanySimulation::new().await?;

    // Run the simulation
    simulation.run().await?;

    println!("👋 Simulation ended. Thank you for running the AI Company!");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_simulation_initialization() {
        let simulation = CompanySimulation::new().await;
        assert!(simulation.is_ok());

        let sim = simulation.unwrap();
        assert!(!sim.agents.is_empty());
        assert!(sim.agents.len() >= 10); // At least managers + department agents
    }

    #[tokio::test]
    async fn test_department_creation() {
        let simulation = CompanySimulation::new().await.unwrap();

        // Check that we have agents from all departments
        let mut departments_found = std::collections::HashSet::new();
        for agent in simulation.agents.values() {
            departments_found.insert(agent.get_agent().department.as_str());
        }

        assert!(departments_found.contains("DevOps"));
        assert!(departments_found.contains("InfoSec"));
        assert!(departments_found.contains("Networking"));
        assert!(departments_found.contains("Ops"));
    }
}


