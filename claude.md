# Claude AI Development Guide for UTCP Skill System

This guide provides Claude AI with comprehensive context and best practices for developing with the Universal Tool Calling Protocol (UTCP) skill system.

## System Overview

UTCP is a production-ready, security-first skill system that replaces MCP with better security, performance, and reliability. The system is built in Rust and follows enterprise-grade patterns.

### Key Architecture Components

- **Core Types**: Strongly typed interfaces with comprehensive error handling
- **Security Layer**: Multi-layered security with command injection prevention, SSRF protection, and file access control
- **Version Management**: Semantic versioning with dependency resolution and Bill of Materials (BOM) support
- **State Management**: Multiple strategies including stateless, externalized, actor-based, and event-sourced
- **Resilience Patterns**: Retry policies, circuit breakers, idempotency, bulkheads, and timeouts
- **Protocol Handlers**: Extensible protocol support (HTTP, WebSocket, CLI, etc.)
- **Skill System**: Dynamic skill loading, registry, and execution management

## Development Guidelines

### 1. Code Structure

```rust
// Follow the established module structure
src/
├── core/          // Type definitions, errors, traits
├── security/      // Security validation, sandbox, audit
├── versioning/    // Semantic versioning, dependency resolution
├── state/         // State management strategies
├── resilience/    // Error recovery patterns
├── protocols/     // Protocol handlers
├── skills/        // Skill management system
└── utils/         // Monitoring, utilities
```

### 2. Type Safety Patterns

```rust
// Use strongly typed parameters
pub struct ToolParameters {
    #[serde(flatten)]
    pub params: HashMap<String, serde_json::Value>,
}

// Implement safe getters
impl ToolParameters {
    pub fn get<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Result<Option<T>, serde_json::Error> {
        match self.params.get(key) {
            Some(value) => Ok(Some(serde_json::from_value(value.clone())?)),
            None => Ok(None),
        }
    }
}
```

### 3. Security-First Development

```rust
// Always validate inputs
SecurityValidator::validate_command(command, &context)?;
SecurityValidator::validate_url(url, &context)?;
SecurityValidator::validate_file_path(path, &operation, &context)?;

// Use security contexts
let security_context = SecurityContext {
    allowed_commands: hashset!["ls", "cat", "grep"],
    command_sanitization: SanitizationPolicy::Strict,
    allowed_domains: vec!["api.example.com".to_string()],
    private_ip_access: PrivateIpPolicy::Block,
    // ...
};
```

### 4. Error Handling

```rust
// Use structured error types
#[derive(Error, Debug)]
pub enum SkillError {
    #[error("Skill not found: {0}")]
    SkillNotFound(String),
    
    #[error("Security violation: {0}")]
    SecurityViolation(String),
    
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
}

// Check for retryable errors
impl SkillError {
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            SkillError::NetworkError(_) | SkillError::Timeout | SkillError::ExecutionError(_)
        )
    }
}
```

### 5. Async/Await Patterns

```rust
// Use async-trait for trait methods
#[async_trait]
pub trait UtcpSkill: Send + Sync {
    async fn execute_tool(
        &self,
        tool_name: &str,
        parameters: ToolParameters,
        context: &ExecutionContext,
    ) -> Result<ToolResult>;
}

// Handle timeouts properly
let result = tokio::time::timeout(duration, operation).await??;
```

### 6. State Management

```rust
// Choose appropriate state strategy
let strategy = match requirements {
    Requirements::Stateless => StateManagementStrategy::Stateless,
    Requirements::Persistent => StateManagementStrategy::Externalized(ExternalStateConfig {
        storage_type: StorageType::Redis { /* ... */ },
        consistency_model: ConsistencyModel::StrongConsistency,
        ttl: Some(Duration::from_secs(3600)),
    }),
};
```

### 7. Resilience Implementation

```rust
// Combine multiple resilience patterns
let result = self
    .idempotency_manager
    .execute_idempotent(&idempotency_key, || async {
        bulkhead.execute(|| async {
            circuit_breaker.call(|| async {
                retry_policy.execute(|| async {
                    skill.execute_tool(tool_name, parameters, context).await
                }).await
            }).await
        }).await
    })
    .await?;
```

## Best Practices for Claude AI

### 1. Code Generation

- **Always start with type definitions** - Define structs and enums before implementation
- **Include comprehensive error handling** - Every function should return Result<T, SkillError>
- **Add proper documentation** - Use rustdoc comments for all public interfaces
- **Follow Rust conventions** - Use snake_case, proper borrowing, and idiomatic patterns

### 2. Security Considerations

- **Validate all inputs** - Use SecurityValidator for commands, URLs, and file paths
- **Apply principle of least privilege** - Start with restrictive permissions
- **Log security events** - Use AuditLogger for security-relevant operations
- **Sanitize outputs** - Prevent information leakage in error messages

### 3. Performance Optimization

- **Use async/await throughout** - All I/O operations should be async
- **Implement proper connection pooling** - For database and HTTP connections
- **Apply caching strategies** - Use idempotency for expensive operations
- **Monitor resource usage** - Implement proper bulkheads and circuit breakers

### 4. Testing Strategy

```rust
#[tokio::test]
async fn test_skill_execution() {
    let skill = MockSkill::new();
    let context = ExecutionContext::new("test_agent".to_string());
    let mut params = ToolParameters::new();
    params.insert("key".to_string(), "value")?;
    
    let result = skill.execute_tool("test_tool", params, &context).await;
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap().success, true);
}
```

### 5. Configuration Management

```rust
// Use structured configuration
#[derive(Deserialize)]
pub struct SkillConfig {
    pub security: SecurityContext,
    pub resilience: ResilienceConfig,
    pub state: StateManagementStrategy,
    pub protocols: Vec<ProtocolConfig>,
}

// Provide sensible defaults
impl Default for SkillConfig {
    fn default() -> Self {
        Self {
            security: SecurityContext::default(),
            resilience: ResilienceConfig::default(),
            state: StateManagementStrategy::Stateless,
            protocols: vec![ProtocolConfig::Http(HttpConfig::default())],
        }
    }
}
```

## Common Patterns

### 1. Skill Implementation Template

```rust
pub struct MySkill {
    skill_id: String,
    metadata: SkillMetadata,
    config: MySkillConfig,
}

#[async_trait]
impl UtcpSkill for MySkill {
    fn skill_id(&self) -> &str { &self.skill_id }
    
    fn metadata(&self) -> &SkillMetadata { &self.metadata }
    
    async fn discover_tools(&self) -> Result<Vec<ToolDefinition>> {
        // Return available tools
    }
    
    async fn execute_tool(
        &self,
        tool_name: &str,
        parameters: ToolParameters,
        context: &ExecutionContext,
    ) -> Result<ToolResult> {
        // Implement tool execution
    }
    
    fn validate_parameters(&self, tool_name: &str, parameters: &ToolParameters) -> Result<()> {
        // Validate input parameters
    }
    
    fn can_handle(&self, context: &TaskContext) -> bool {
        // Check if skill can handle the task
    }
    
    fn dependencies(&self) -> Vec<SkillDependency> {
        // Return skill dependencies
    }
    
    async fn health_check(&self) -> Result<HealthStatus> {
        // Implement health check
    }
}
```

### 2. Protocol Handler Template

```rust
pub struct MyProtocolHandler {
    client: MyClient,
    config: MyProtocolConfig,
}

#[async_trait]
impl ProtocolHandler for MyProtocolHandler {
    fn protocol_type(&self) -> ProtocolType {
        ProtocolType::Custom("my_protocol".to_string())
    }
    
    async fn execute_call(
        &self,
        call_template: &CallTemplate,
        parameters: &ToolParameters,
        context: &ExecutionContext,
    ) -> Result<ToolResult> {
        // Implement protocol-specific execution
    }
    
    fn validate_call_template(&self, template: &CallTemplate) -> Result<()> {
        // Validate call template
    }
}
```

## Integration Points

### 1. External Systems

- **Database Integration**: Use sqlx for PostgreSQL, redis crate for Redis
- **HTTP APIs**: Use reqwest with proper timeout and retry configuration
- **Message Queues**: Implement async message handling patterns
- **File Systems**: Use tokio::fs for async file operations with security validation

### 2. Monitoring and Observability

```rust
// Use structured logging
tracing::info!(
    skill_id = %skill_id,
    tool_name = %tool_name,
    execution_id = %context.execution_id,
    "Tool execution started"
);

// Emit metrics
metrics::counter!("skill_executions_total", 1, "skill_id" => skill_id);
metrics::histogram!("skill_execution_duration", duration.as_secs_f64());
```

### 3. Configuration and Deployment

- Use environment variables for sensitive configuration
- Support multiple deployment targets (Docker, Kubernetes, bare metal)
- Implement graceful shutdown handling
- Provide health check endpoints for load balancers

## Development Workflow

1. **Design Phase**: Define types and interfaces first
2. **Security Review**: Validate security implications of new features
3. **Implementation**: Follow established patterns and conventions
4. **Testing**: Write comprehensive unit and integration tests
5. **Documentation**: Update relevant documentation and examples
6. **Deployment**: Use provided Docker and deployment configurations

This guide ensures consistent, secure, and maintainable development practices when working with the UTCP skill system.