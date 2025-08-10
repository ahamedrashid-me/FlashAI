use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

pub mod task_manager;
pub mod config;

pub use task_manager::TaskManager;
pub use config::Config;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStatus {
    pub health: String,
    pub active_tasks: u32,
    pub proxy_count: u32,
    pub stealth_active: bool,
    pub last_activity: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub description: String,
    pub status: TaskStatus,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub progress: f32,
    pub results_count: u32,
    pub output_path: Option<String>,
    pub stealth_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Planning,
    Executing,
    Completed,
    Failed,
    Paused,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrapingResult {
    pub task_id: String,
    pub data: serde_json::Value,
    pub source_url: String,
    pub extracted_at: DateTime<Utc>,
    pub quality_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyInfo {
    pub id: u32,
    pub url: String,
    pub is_active: bool,
    pub last_used: Option<DateTime<Utc>>,
    pub success_rate: f32,
    pub response_time_ms: Option<u32>,
}

/// Core Flash AI Engine
pub struct FlashEngine {
    config: Config,
    active_tasks: RwLock<HashMap<String, Task>>,
    proxies: RwLock<Vec<ProxyInfo>>,
    // Add other core components here
}

impl FlashEngine {
    pub async fn new(config: Config) -> Result<Self> {
        Ok(Self {
            config,
            active_tasks: RwLock::new(HashMap::new()),
            proxies: RwLock::new(Vec::new()),
        })
    }

    pub async fn get_status(&self) -> Result<SystemStatus> {
        let tasks = self.active_tasks.read().await;
        let proxies = self.proxies.read().await;
        
        Ok(SystemStatus {
            health: "Healthy".to_string(),
            active_tasks: tasks.len() as u32,
            proxy_count: proxies.len() as u32,
            stealth_active: self.config.stealth.enabled,
            last_activity: Utc::now(),
        })
    }

    pub async fn create_task(&self, description: String, stealth: bool) -> Result<String> {
        let task_id = uuid::Uuid::new_v4().to_string();
        let task = Task {
            id: task_id.clone(),
            description,
            status: TaskStatus::Pending,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
            progress: 0.0,
            results_count: 0,
            output_path: None,
            stealth_enabled: stealth,
        };

        let mut tasks = self.active_tasks.write().await;
        tasks.insert(task_id.clone(), task);
        
        Ok(task_id)
    }
}
