use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt::Display};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Priority {
    High,
    Medium,
    Low,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Status {
    Pending,
    Completed { completed_date: NaiveDate },
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

    pub fn finish(&mut self) {
        self.status = Status::Completed {
            completed_date: Utc::now().date_naive(),
        };
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let icon_status = match self.status {
            Status::Pending => "⏳",
            Status::Completed { .. } => "✅",
        };

        write!(
            f,
            "{} {} (Data Prevista: {}) ",
            icon_status,
            self.title,
            self.date.format("%d-%m-%Y")
        )
    }
}

#[derive(Debug)]
pub enum TaskError {
    FileIoError(String),
    DeserializeError(String),
}

impl Display for TaskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskError::FileIoError(msg) => write!(f, "Error read or write file: {}", msg),
            TaskError::DeserializeError(msg) => write!(f, "Error deserializing file: {}", msg),
        }
    }
}

impl Error for TaskError {}

impl From<std::io::Error> for TaskError {
    fn from(err: std::io::Error) -> Self {
        TaskError::FileIoError(err.to_string())
    }
}

impl From<serde_json::Error> for TaskError {
    fn from(err: serde_json::Error) -> Self {
        TaskError::DeserializeError(err.to_string())
    }
}
