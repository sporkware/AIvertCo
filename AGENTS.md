# AGENTS.md - Development Guidelines for AI Company Simulation

## Build/Lint/Test Commands
- **Build**: `npm run build` (for web components) | `cargo build` (for Rust agents)
- **Lint**: `npm run lint` | `cargo clippy`
- **Test**: `npm test` | `cargo test`
- **Single Test**: `npm test -- --testNamePattern="test name"` | `cargo test test_name`
- **Type Check**: `npm run typecheck` | `cargo check`

## Code Style Guidelines

### General
- Use TypeScript for web components, Rust for core agents
- Follow existing patterns in each department's codebase
- Document all public APIs and agent interfaces

### Naming Conventions
- **Functions**: camelCase (JavaScript/TypeScript), snake_case (Rust)
- **Classes/Components**: PascalCase
- **Files**: kebab-case for components, snake_case for Rust modules
- **Constants**: SCREAMING_SNAKE_CASE

### Imports & Dependencies
- Group imports: std lib, external packages, internal modules
- Use absolute imports for internal modules
- Avoid wildcard imports (`import *`)

### Error Handling
- Use Result/Option types in Rust
- Throw descriptive errors in JavaScript/TypeScript
- Log errors with context, never expose sensitive data

### Formatting
- Use Prettier for JavaScript/TypeScript
- Use rustfmt for Rust code
- 2 spaces indentation for JS/TS, 4 spaces for Rust

### Types & Interfaces
- Define interfaces for all agent communications
- Use strict TypeScript settings
- Document complex types with JSDoc/TSDoc

### Testing
- Unit tests for all agent logic
- Integration tests for department interactions
- Mock external dependencies

### Security
- Never log sensitive data (API keys, credentials)
- Validate all agent inputs
- Use environment variables for configuration