use std::fmt;

/// The current lifecycle state of a task.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

impl TaskStatus {
    pub fn next(self) -> Self {
        match self {
            Self::Todo => Self::InProgress,
            Self::InProgress => Self::Done,
            Self::Done => Self::Todo,
        }
    }

    pub fn symbol(self) -> &'static str {
        match self {
            Self::Todo => "[ ]",
            Self::InProgress => "[~]",
            Self::Done => "[x]",
        }
    }
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Self::Todo => "Todo",
            Self::InProgress => "In Progress",
            Self::Done => "Done",
        };
        f.write_str(label)
    }
}

/// How important a task is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Priority {
    Low,
    Normal,
    High,
    Urgent,
}

impl Priority {
    pub fn next(self) -> Self {
        match self {
            Self::Low => Self::Normal,
            Self::Normal => Self::High,
            Self::High => Self::Urgent,
            Self::Urgent => Self::Low,
        }
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Self::Low => "Low",
            Self::Normal => "Normal",
            Self::High => "High",
            Self::Urgent => "Urgent",
        };
        f.write_str(label)
    }
}

/// A task shown in the dashboard.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub project: Option<String>,
    pub status: TaskStatus,
    pub priority: Priority,
}

impl Task {
    pub fn new(
        id: u32,
        title: impl Into<String>,
        description: impl Into<String>,
        project: Option<String>,
        status: TaskStatus,
        priority: Priority,
    ) -> Self {
        Self {
            id,
            title: title.into(),
            description: description.into(),
            project,
            status,
            priority,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Priority, TaskStatus};

    #[test]
    fn task_status_cycles_in_order() {
        assert_eq!(TaskStatus::Todo.next(), TaskStatus::InProgress);
        assert_eq!(TaskStatus::InProgress.next(), TaskStatus::Done);
        assert_eq!(TaskStatus::Done.next(), TaskStatus::Todo);
    }

    #[test]
    fn priority_cycles_in_order() {
        assert_eq!(Priority::Low.next(), Priority::Normal);
        assert_eq!(Priority::Normal.next(), Priority::High);
        assert_eq!(Priority::High.next(), Priority::Urgent);
        assert_eq!(Priority::Urgent.next(), Priority::Low);
    }
}
