use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Priority {
    High,
    Medium,
    Low,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Status {
    Pending,
    Completed { completed_date: NaiveDate },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub title: String,
    pub description: String,
    pub category: String,
    pub date: NaiveDate,
    pub priority: Priority,
    pub status: Status,
}

impl Task {
    pub fn new(
        title: String,
        description: String,
        category: String,
        date: NaiveDate,
        priority: Priority,
    ) -> Self {
        Self {
            date,
            title,
            category,
            priority,
            description,
            status: Status::Pending,
        }
    }

    pub fn show(&self) -> String {
        let icon_status = match self.status {
            Status::Pending => "⏳",
            Status::Completed { .. } => "✅",
        };
        format!(
            "{} {} (Data Prevista: {}) ",
            icon_status,
            self.title,
            self.date.format("%d-%m-%Y")
        )
    }

    pub fn finish(&mut self) {
        self.status = Status::Completed {
            completed_date: Utc::now().date_naive(),
        };
    }
}
