//! UTCP Skill System - Universal Tool Calling Protocol
//!
//! A production-ready, security-first skill system that replaces MCP
//! with better security, performance, and reliability.

pub mod core;
pub mod security;
pub mod versioning;
pub mod state;
pub mod resilience;
pub mod protocols;
pub mod skills;
pub mod utils;

// Re-export commonly used types
pub use crate::core::{
    errors::{Result, SkillError},
    types::{
        ExecutionContext, SkillId, ToolName, ToolParameters, ToolResult,
        TaskContext, ResourceConstraints, ProtocolType
    },
    traits::{UtcpSkill, ProtocolHandler, SkillRegistry, StateStorage}
};

pub use crate::security::SecurityContext;
pub use crate::versioning::{SemanticVersion, VersionConstraint};
pub use crate::state::{StateManager, StateManagementStrategy};
pub use crate::resilience::{RetryPolicy, CircuitBreaker, IdempotencyManager};
pub use crate::skills::SkillManager;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_instantiation() {
        let _context = ExecutionContext::new("test_agent".to_string());
        let _params = ToolParameters::new();
        // Basic smoke test to ensure types compile
    }
}