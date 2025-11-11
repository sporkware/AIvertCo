# Development Workflow - AI Company Simulation

## Git Workflow

### Branching Strategy
```
main (production-ready)
├── develop (integration branch)
│   ├── feature/agent-system
│   ├── feature/department-structure
│   ├── feature/communication-bus
│   └── feature/web-dashboard
```

### Commit Conventions
```
feat: add new agent personality system
fix: resolve message routing deadlock
docs: update API documentation
refactor: simplify agent state management
test: add integration tests for department interactions
```

### Pull Request Process
1. Create feature branch from `develop`
2. Implement changes with tests
3. Run full test suite locally
4. Create PR with detailed description
5. Code review by at least 2 team members
6. Automated CI checks pass
7. Merge to `develop` after approval
8. Regular merges from `develop` to `main`

## Development Environment

### Local Setup
```bash
# Clone repository
git clone https://github.com/sporkware/AIvertCo.git
cd virtco

# Start development environment
docker-compose up -d

# Run backend tests
cargo test

# Run frontend development server
cd web && npm run dev

# Run linting
npm run lint
cargo clippy
```

### IDE Configuration
- **VS Code** with Rust and TypeScript extensions
- **Rust Analyzer** for code intelligence
- **Prettier** for code formatting
- **ESLint** for JavaScript/TypeScript linting

## Coding Standards

### Rust Code Style
```rust
// Good: Clear naming and structure
pub struct AgentManager {
    agents: HashMap<Uuid, Agent>,
    message_bus: Arc<MessageBus>,
}

impl AgentManager {
    pub fn new() -> Self {
        Self {
            agents: HashMap::new(),
            message_bus: Arc::new(MessageBus::new()),
        }
    }

    pub async fn add_agent(&mut self, agent: Agent) -> Result<(), AgentError> {
        self.agents.insert(agent.id, agent);
        Ok(())
    }
}

// Bad: Unclear naming and poor structure
struct AM {
    a: HashMap<Uuid, Agent>,
    mb: Arc<MessageBus>,
}

impl AM {
    fn n() -> Self { Self { a: HashMap::new(), mb: Arc::new(MessageBus::new()) } }
    async fn aa(&mut self, a: Agent) -> Result<(), AgentError> { self.a.insert(a.id, a); Ok(()) }
}
```

### TypeScript Code Style
```typescript
// Good: Type safety and clear interfaces
interface Agent {
  readonly id: string;
  name: string;
  department: Department;
  personality: Personality;
}

interface AgentService {
  getAgent(id: string): Promise<Agent | null>;
  updateAgent(id: string, updates: Partial<Agent>): Promise<Agent>;
  listAgents(department?: Department): Promise<Agent[]>;
}

class AgentServiceImpl implements AgentService {
  constructor(private readonly db: Database) {}

  async getAgent(id: string): Promise<Agent | null> {
    return this.db.agents.findOne({ id });
  }

  async updateAgent(id: string, updates: Partial<Agent>): Promise<Agent> {
    const agent = await this.getAgent(id);
    if (!agent) throw new Error(`Agent ${id} not found`);

    const updated = { ...agent, ...updates };
    await this.db.agents.update(id, updated);
    return updated;
  }

  async listAgents(department?: Department): Promise<Agent[]> {
    const query = department ? { department } : {};
    return this.db.agents.find(query);
  }
}
```

## Testing Strategy

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_creation() {
        let agent = Agent::new("Test Agent", Department::Engineering);
        assert_eq!(agent.name, "Test Agent");
        assert_eq!(agent.department, Department::Engineering);
        assert!(agent.is_active());
    }

    #[tokio::test]
    async fn test_agent_message_handling() {
        let mut agent = create_test_agent().await;
        let message = create_test_message();

        let result = agent.handle_message(message).await;
        assert!(result.is_ok());
    }
}
```

### Integration Tests
```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    use testcontainers::clients::Cli;

    #[tokio::test]
    async fn test_agent_communication() {
        let docker = Cli::default();
        let postgres = docker.run(Postgres::default());
        let redis = docker.run(Redis::default());

        // Set up test environment
        let db_url = format!("postgres://postgres:password@localhost:{}/test",
                           postgres.get_host_port_ipv4(5432));

        // Run integration test
        let result = run_agent_communication_test(&db_url).await;
        assert!(result.is_ok());
    }
}
```

### End-to-End Tests
```typescript
// E2E test with Playwright
import { test, expect } from '@playwright/test';

test('complete agent workflow', async ({ page }) => {
  // Navigate to dashboard
  await page.goto('/');

  // Create new agent
  await page.click('[data-testid="create-agent"]');
  await page.fill('[data-testid="agent-name"]', 'Test Engineer');
  await page.selectOption('[data-testid="department"]', 'Engineering');
  await page.click('[data-testid="submit"]');

  // Verify agent appears in list
  await expect(page.locator('[data-testid="agent-list"]')).toContainText('Test Engineer');

  // Assign task
  await page.click('[data-testid="assign-task"]');
  await page.fill('[data-testid="task-title"]', 'Implement login feature');
  await page.click('[data-testid="assign"]');

  // Verify task assignment
  await expect(page.locator('[data-testid="task-status"]')).toHaveText('Assigned');
});
```

## Code Review Checklist

### For All Changes
- [ ] Tests pass locally
- [ ] Code follows style guidelines
- [ ] No linting errors
- [ ] Documentation updated
- [ ] Breaking changes documented

### Rust-Specific
- [ ] No unsafe code without justification
- [ ] Error handling is comprehensive
- [ ] Performance considerations addressed
- [ ] Clippy warnings resolved

### TypeScript-Specific
- [ ] Type safety maintained
- [ ] No any types without comments
- [ ] React hooks follow rules
- [ ] Bundle size impact considered

## Deployment Process

### Staging Deployment
```bash
# Deploy to staging
git checkout develop
git pull origin develop
docker-compose -f docker-compose.staging.yml up -d

# Run integration tests against staging
npm run test:e2e -- --env staging
```

### Production Deployment
```bash
# Tag release
git checkout main
git pull origin main
git tag v1.2.3
git push origin v1.2.3

# Deploy via CI/CD pipeline
# - Build Docker images
# - Run security scans
# - Deploy to production
# - Run smoke tests
# - Monitor for 30 minutes
```

## Monitoring & Observability

### Application Metrics
- Agent response times
- Message throughput
- Error rates by department
- Project completion rates

### Infrastructure Metrics
- CPU/Memory usage
- Database connection pools
- Message queue depth
- API response times

### Logging
```rust
// Structured logging
use tracing::{info, error, warn};

#[tracing::instrument]
pub async fn process_message(&self, message: Message) -> Result<(), AgentError> {
    info!("Processing message from agent {}", message.from_agent);

    match self.validate_message(&message).await {
        Ok(_) => {
            self.route_message(message).await?;
            info!("Message processed successfully");
            Ok(())
        }
        Err(e) => {
            error!("Message validation failed: {}", e);
            Err(e)
        }
    }
}
```

### Alerting
- Agent system down
- High error rates
- Database connection issues
- Message queue backlog

## Security Considerations

### Code Security
- Dependency vulnerability scanning
- Secrets management (no hardcoded credentials)
- Input validation and sanitization
- SQL injection prevention

### Infrastructure Security
- Network segmentation
- Access control (RBAC)
- Encryption in transit and at rest
- Regular security updates

### Agent Security
- Sandboxed agent execution
- Resource limits per agent
- Communication encryption
- Audit logging for all actions

## Performance Optimization

### Backend Optimization
- Async/await for I/O operations
- Connection pooling
- Caching strategies
- Database query optimization

### Frontend Optimization
- Code splitting
- Lazy loading
- Image optimization
- Bundle analysis

### Agent Optimization
- Efficient message serialization
- Memory pooling for agents
- Background task processing
- Rate limiting for API calls