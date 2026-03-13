use serde::Deserialize;
use sqlx::SqlitePool;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
}

#[derive(Debug, sqlx::FromRow)]
#[allow(dead_code)]
pub struct Task {
    pub id: i64,
    pub name: String,
    pub status: String,
    pub priority: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl Task {
    /// "InProgress" → "In Progress"
    pub fn status_display(&self) -> &str {
        match self.status.as_str() {
            "InProgress" => "In Progress",
            other => other,
        }
    }

    /// "InProgress" → "progress", "Todo" → "todo", "Done" → "done"
    pub fn status_class(&self) -> &str {
        match self.status.as_str() {
            "InProgress" => "progress",
            "Todo" => "todo",
            "Done" => "done",
            _ => "todo",
        }
    }

    /// "High" → "high" etc.
    pub fn priority_class(&self) -> &str {
        match self.priority.as_str() {
            "High" => "high",
            "Medium" => "medium",
            "Low" => "low",
            _ => "medium",
        }
    }
}

#[derive(Debug, Deserialize, Default)]
pub struct TaskFilter {
    pub search: Option<String>,
    pub status: Option<String>,
    pub priority: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateTask {
    pub name: String,
    pub status: String,
    pub priority: String,
}

pub struct DashboardStats {
    pub total: i64,
    pub in_progress: i64,
    pub done: i64,
    pub todo: i64,
}
