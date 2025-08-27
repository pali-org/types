//! # Pali Types
//!
//! Shared data types for the Pali todo management system.
//! 
//! This crate contains all the common data structures used by both the Pali server 
//! and CLI client, ensuring consistency and type safety across the entire system.
//!
//! ## Todo Management
//! 
//! - [`Todo`] - Core todo item structure
//! - [`CreateTodoRequest`] - Request to create a new todo
//! - [`UpdateTodoRequest`] - Request to update an existing todo
//!
//! ## API Key Management
//! 
//! - [`KeyType`] - Admin or Client key types  
//! - [`CreateApiKeyRequest`] - Request to create an API key
//! - [`ApiKeyResponse`] - Response containing new API key
//! - [`ApiKeyInfo`] - Public API key information (without key)
//!
//! ## API Responses
//!
//! - [`ApiResponse`] - Standard API response wrapper

use serde::{Deserialize, Serialize};

/// Priority level constants
pub mod priority {
    /// Low priority
    pub const LOW: i32 = 1;
    /// Medium priority (default)
    pub const MEDIUM: i32 = 2;
    /// High priority
    pub const HIGH: i32 = 3;
    
    /// Minimum valid priority value
    pub const MIN: i32 = LOW;
    /// Maximum valid priority value
    pub const MAX: i32 = HIGH;
    
    /// Check if a priority value is valid
    pub fn is_valid(priority: i32) -> bool {
        (MIN..=MAX).contains(&priority)
    }
    
    /// Clamp a priority to the valid range
    pub fn clamp(priority: i32) -> i32 {
        priority.clamp(MIN, MAX)
    }
}

/// A todo item in the Pali system
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    /// Unique identifier for the todo
    pub id: String,
    /// The todo title
    pub title: String,
    /// Optional description
    pub description: Option<String>,
    /// Whether the todo is completed
    pub completed: bool,
    /// Priority level (1=low, 2=medium, 3=high)
    pub priority: i32,
    /// Optional due date as Unix timestamp
    pub due_date: Option<i64>,
    /// Creation timestamp
    pub created_at: i64,
    /// Last update timestamp
    pub updated_at: i64,
}

/// Request to create a new todo
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateTodoRequest {
    /// The todo title
    pub title: String,
    /// Optional description
    pub description: Option<String>,
    /// Priority level (1=low, 2=medium, 3=high), defaults to 2
    pub priority: Option<i32>,
    /// Optional due date as Unix timestamp
    pub due_date: Option<i64>,
}

/// Request to update an existing todo
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct UpdateTodoRequest {
    /// Update the title
    pub title: Option<String>,
    /// Update the description
    pub description: Option<String>,
    /// Update completion status
    pub completed: Option<bool>,
    /// Update priority level
    pub priority: Option<i32>,
    /// Update due date
    pub due_date: Option<i64>,
}

/// API key permission types
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum KeyType {
    /// Administrative privileges
    Admin,
    /// Standard client access
    Client,
}

/// Request to create a new API key
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateApiKeyRequest {
    /// Friendly name for the client/application
    pub client_name: String,
    /// Permission level for the key
    pub key_type: KeyType,
}

/// Response containing a newly created API key
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiKeyResponse {
    /// Unique key identifier
    pub id: String,
    /// Client name
    pub client_name: String,
    /// Permission level
    pub key_type: KeyType,
    /// The actual API key (only returned once)
    pub api_key: String,
    /// Creation timestamp
    pub created_at: i64,
}

/// Public API key information (without the actual key)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiKeyInfo {
    /// Unique key identifier
    pub id: String,
    /// Client name
    pub client_name: String,
    /// Permission level
    pub key_type: KeyType,
    /// Last time this key was used
    pub last_used: Option<i64>,
    /// Creation timestamp
    pub created_at: i64,
    /// Whether the key is active
    pub active: bool,
}

/// Standard API response wrapper
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiResponse<T> {
    /// Whether the request succeeded
    pub success: bool,
    /// Response data (when successful)
    pub data: Option<T>,
    /// Error message (when failed)
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    /// Create a successful API response with data
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    /// Create an error API response with message
    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
        }
    }

    /// Check if the response is successful
    pub fn is_success(&self) -> bool {
        self.success
    }

    /// Check if the response is an error
    pub fn is_error(&self) -> bool {
        !self.success
    }
}

impl Todo {
    /// Check if the todo is overdue based on current time
    /// 
    /// Returns `false` if:
    /// - No due date is set
    /// - Todo is already completed  
    /// - System time is unavailable
    pub fn is_overdue(&self) -> bool {
        if let Some(due_date) = self.due_date {
            if self.completed {
                return false;
            }
            
            match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
                Ok(duration) => {
                    let now = duration.as_secs() as i64;
                    due_date < now
                }
                Err(_) => false, // Can't determine time, assume not overdue
            }
        } else {
            false
        }
    }

    /// Check if the todo is high priority (3)
    pub fn is_high_priority(&self) -> bool {
        self.priority == crate::priority::HIGH
    }

    /// Check if the todo is low priority (1)
    pub fn is_low_priority(&self) -> bool {
        self.priority == crate::priority::LOW
    }

    /// Check if the todo is medium priority (2)
    pub fn is_medium_priority(&self) -> bool {
        self.priority == crate::priority::MEDIUM
    }

    /// Get a human-readable priority string
    pub fn priority_str(&self) -> &'static str {
        match self.priority {
            crate::priority::LOW => "low",
            crate::priority::MEDIUM => "medium",
            crate::priority::HIGH => "high",
            p if p < crate::priority::LOW => "low", // Invalid values default to low
            p if p > crate::priority::HIGH => "high", // Invalid values default to high
            _ => "medium", // Fallback (shouldn't happen)
        }
    }

    /// Check if the todo has a valid priority
    pub fn has_valid_priority(&self) -> bool {
        crate::priority::is_valid(self.priority)
    }
}

impl CreateTodoRequest {
    /// Create a new todo request with just a title
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: None,
            priority: None,
            due_date: None,
        }
    }

    /// Set the description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the priority (1=low, 2=medium, 3=high)
    /// 
    /// Values outside the valid range will be clamped.
    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = Some(crate::priority::clamp(priority));
        self
    }

    /// Set the due date as Unix timestamp
    /// 
    /// Negative timestamps will be rejected (set to None).
    pub fn with_due_date(mut self, due_date: i64) -> Self {
        if due_date >= 0 {
            self.due_date = Some(due_date);
        }
        self
    }

    /// Set priority to low
    pub fn with_low_priority(self) -> Self {
        self.with_priority(crate::priority::LOW)
    }

    /// Set priority to medium (default)
    pub fn with_medium_priority(self) -> Self {
        self.with_priority(crate::priority::MEDIUM)
    }

    /// Set priority to high
    pub fn with_high_priority(self) -> Self {
        self.with_priority(crate::priority::HIGH)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_constants() {
        assert_eq!(priority::LOW, 1);
        assert_eq!(priority::MEDIUM, 2);
        assert_eq!(priority::HIGH, 3);
        assert_eq!(priority::MIN, 1);
        assert_eq!(priority::MAX, 3);
    }

    #[test]
    fn test_priority_validation() {
        assert!(priority::is_valid(1));
        assert!(priority::is_valid(2));
        assert!(priority::is_valid(3));
        assert!(!priority::is_valid(0));
        assert!(!priority::is_valid(4));
        assert!(!priority::is_valid(-1));
    }

    #[test]
    fn test_priority_clamping() {
        assert_eq!(priority::clamp(0), 1);
        assert_eq!(priority::clamp(1), 1);
        assert_eq!(priority::clamp(2), 2);
        assert_eq!(priority::clamp(3), 3);
        assert_eq!(priority::clamp(4), 3);
        assert_eq!(priority::clamp(-5), 1);
        assert_eq!(priority::clamp(100), 3);
    }

    #[test]
    fn test_todo_priority_methods() {
        let low_todo = Todo {
            id: "test".to_string(),
            title: "Test".to_string(),
            description: None,
            completed: false,
            priority: priority::LOW,
            due_date: None,
            created_at: 1640995200,
            updated_at: 1640995200,
        };

        let medium_todo = Todo { priority: priority::MEDIUM, ..low_todo.clone() };
        let high_todo = Todo { priority: priority::HIGH, ..low_todo.clone() };
        let invalid_todo = Todo { priority: 0, ..low_todo.clone() };

        assert!(low_todo.is_low_priority());
        assert!(!low_todo.is_medium_priority());
        assert!(!low_todo.is_high_priority());
        assert_eq!(low_todo.priority_str(), "low");
        assert!(low_todo.has_valid_priority());

        assert!(!medium_todo.is_low_priority());
        assert!(medium_todo.is_medium_priority());
        assert!(!medium_todo.is_high_priority());
        assert_eq!(medium_todo.priority_str(), "medium");
        assert!(medium_todo.has_valid_priority());

        assert!(!high_todo.is_low_priority());
        assert!(!high_todo.is_medium_priority());
        assert!(high_todo.is_high_priority());
        assert_eq!(high_todo.priority_str(), "high");
        assert!(high_todo.has_valid_priority());

        assert!(!invalid_todo.is_low_priority());
        assert!(!invalid_todo.is_medium_priority());
        assert!(!invalid_todo.is_high_priority());
        assert_eq!(invalid_todo.priority_str(), "low"); // Invalid value < LOW defaults to "low"
        assert!(!invalid_todo.has_valid_priority());
    }

    #[test]
    fn test_priority_str_edge_cases() {
        let very_low_todo = Todo {
            id: "test".to_string(),
            title: "Test".to_string(),
            description: None,
            completed: false,
            priority: -5, // Very low
            due_date: None,
            created_at: 1640995200,
            updated_at: 1640995200,
        };

        let very_high_todo = Todo { priority: 100, ..very_low_todo.clone() };

        assert_eq!(very_low_todo.priority_str(), "low");
        assert_eq!(very_high_todo.priority_str(), "high");
    }

    #[test]
    fn test_todo_overdue() {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let past_time = current_time - 3600; // 1 hour ago
        let future_time = current_time + 3600; // 1 hour from now

        let overdue_todo = Todo {
            id: "test".to_string(),
            title: "Test".to_string(),
            description: None,
            completed: false,
            priority: priority::MEDIUM,
            due_date: Some(past_time),
            created_at: current_time,
            updated_at: current_time,
        };

        let not_due_todo = Todo { due_date: Some(future_time), ..overdue_todo.clone() };
        let completed_todo = Todo { completed: true, ..overdue_todo.clone() };
        let no_due_date_todo = Todo { due_date: None, ..overdue_todo.clone() };

        assert!(overdue_todo.is_overdue());
        assert!(!not_due_todo.is_overdue());
        assert!(!completed_todo.is_overdue()); // Completed todos aren't overdue
        assert!(!no_due_date_todo.is_overdue());
    }

    #[test]
    fn test_create_todo_request_builder() {
        let request = CreateTodoRequest::new("Test Todo")
            .with_description("Test description")
            .with_high_priority()
            .with_due_date(1640995200);

        assert_eq!(request.title, "Test Todo");
        assert_eq!(request.description, Some("Test description".to_string()));
        assert_eq!(request.priority, Some(priority::HIGH));
        assert_eq!(request.due_date, Some(1640995200));
    }

    #[test]
    fn test_create_todo_request_priority_helpers() {
        let low = CreateTodoRequest::new("Test").with_low_priority();
        let medium = CreateTodoRequest::new("Test").with_medium_priority();
        let high = CreateTodoRequest::new("Test").with_high_priority();

        assert_eq!(low.priority, Some(priority::LOW));
        assert_eq!(medium.priority, Some(priority::MEDIUM));
        assert_eq!(high.priority, Some(priority::HIGH));
    }

    #[test]
    fn test_priority_clamping_in_builder() {
        let clamped_low = CreateTodoRequest::new("Test").with_priority(0);
        let clamped_high = CreateTodoRequest::new("Test").with_priority(10);
        let normal = CreateTodoRequest::new("Test").with_priority(2);

        assert_eq!(clamped_low.priority, Some(1));
        assert_eq!(clamped_high.priority, Some(3));
        assert_eq!(normal.priority, Some(2));
    }

    #[test]
    fn test_due_date_validation() {
        let valid = CreateTodoRequest::new("Test").with_due_date(1640995200);
        let invalid = CreateTodoRequest::new("Test").with_due_date(-1);
        let zero = CreateTodoRequest::new("Test").with_due_date(0);

        assert_eq!(valid.due_date, Some(1640995200));
        assert_eq!(invalid.due_date, None); // Negative timestamp rejected
        assert_eq!(zero.due_date, Some(0)); // Zero is valid (Unix epoch)
    }

    #[test]
    fn test_api_response_helpers() {
        let success: ApiResponse<String> = ApiResponse::success("test".to_string());
        let error: ApiResponse<String> = ApiResponse::error("error message".to_string());

        assert!(success.is_success());
        assert!(!success.is_error());
        assert_eq!(success.data, Some("test".to_string()));
        assert_eq!(success.error, None);

        assert!(!error.is_success());
        assert!(error.is_error());
        assert_eq!(error.data, None);
        assert_eq!(error.error, Some("error message".to_string()));
    }

    #[test]
    fn test_serialization() {
        let todo = Todo {
            id: "test-id".to_string(),
            title: "Test Todo".to_string(),
            description: Some("Description".to_string()),
            completed: false,
            priority: priority::HIGH,
            due_date: Some(1640995200),
            created_at: 1640995200,
            updated_at: 1640995200,
        };

        // Test serialization
        let json = serde_json::to_string(&todo).unwrap();
        assert!(json.contains("test-id"));
        assert!(json.contains("Test Todo"));

        // Test deserialization
        let deserialized: Todo = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, todo.id);
        assert_eq!(deserialized.title, todo.title);
        assert_eq!(deserialized.priority, todo.priority);
    }

    #[test]
    fn test_key_type_serialization() {
        let admin = KeyType::Admin;
        let client = KeyType::Client;

        let admin_json = serde_json::to_string(&admin).unwrap();
        let client_json = serde_json::to_string(&client).unwrap();

        assert_eq!(admin_json, "\"admin\"");
        assert_eq!(client_json, "\"client\"");

        let deserialized_admin: KeyType = serde_json::from_str(&admin_json).unwrap();
        let deserialized_client: KeyType = serde_json::from_str(&client_json).unwrap();

        assert_eq!(deserialized_admin, KeyType::Admin);
        assert_eq!(deserialized_client, KeyType::Client);
    }

    #[test]
    fn test_update_todo_request_default() {
        let default_request = UpdateTodoRequest::default();
        
        assert_eq!(default_request.title, None);
        assert_eq!(default_request.description, None);
        assert_eq!(default_request.completed, None);
        assert_eq!(default_request.priority, None);
        assert_eq!(default_request.due_date, None);
    }

    #[test]
    fn test_clone_implementations() {
        let todo = Todo {
            id: "test".to_string(),
            title: "Test".to_string(),
            description: None,
            completed: false,
            priority: priority::MEDIUM,
            due_date: None,
            created_at: 1640995200,
            updated_at: 1640995200,
        };

        let cloned = todo.clone();
        assert_eq!(todo.id, cloned.id);
        assert_eq!(todo.title, cloned.title);

        let request = CreateTodoRequest::new("Test");
        let cloned_request = request.clone();
        assert_eq!(request.title, cloned_request.title);
    }
}
