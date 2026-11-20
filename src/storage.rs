use std::{
    collections::HashMap,
    path::Path,
};

use rusqlite::{Connection, TransactionBehavior, params};

use crate::{
    project::Project,
    task::{Priority, Task, TaskStatus},
};

/// SQLite-backed persistence for TaskFlow.
pub struct Database {
    connection: Connection,
}

impl Database {
    /// Open a database and apply any required schema migrations.
    pub fn open(path: &Path) -> rusqlite::Result<Self> {
        if path != Path::new(":memory:") {
            if let Some(parent) = path.parent().filter(|parent| !parent.as_os_str().is_empty()) {
                std::fs::create_dir_all(parent)
                    .map_err(|_| rusqlite::Error::InvalidPath(parent.to_path_buf()))?;
            }
        }

        let mut connection = Connection::open(path)?;
        connection.pragma_update(None, "foreign_keys", "ON")?;
        connection.busy_timeout(std::time::Duration::from_secs(5))?;
        Self::migrate(&mut connection)?;

        Ok(Self { connection })
    }

    fn migrate(connection: &mut Connection) -> rusqlite::Result<()> {
        let version: i64 =
            connection.pragma_query_value(None, "user_version", |row| row.get(0))?;

        if version > 1 {
            return Err(rusqlite::Error::InvalidQuery);
        }

        if version == 0 {
            let transaction =
                connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
            transaction.execute_batch(
                "
                CREATE TABLE projects (
                    id   INTEGER PRIMARY KEY,
                    name TEXT NOT NULL COLLATE NOCASE UNIQUE
                );

                CREATE TABLE tasks (
                    id          INTEGER PRIMARY KEY,
                    title       TEXT NOT NULL CHECK (length(trim(title)) > 0),
                    description TEXT NOT NULL DEFAULT '',
                    status      TEXT NOT NULL CHECK (
                        status IN ('Todo', 'In Progress', 'Done')
                    ),
                    priority    TEXT NOT NULL CHECK (
                        priority IN ('Low', 'Normal', 'High', 'Urgent')
                    ),
                    project_id  INTEGER REFERENCES projects(id) ON DELETE SET NULL
                );

                CREATE INDEX idx_tasks_project_id ON tasks(project_id);
                PRAGMA user_version = 1;
                ",
            )?;
            transaction.commit()?;
        }

        Ok(())
    }

    /// Load projects and tasks, preserving their stable IDs.
    pub fn load(&self) -> rusqlite::Result<(Vec<Task>, Vec<Project>)> {
        let mut project_statement =
            self.connection.prepare("SELECT id, name FROM projects ORDER BY id")?;
        let projects = project_statement
            .query_map([], |row| {
                let id: i64 = row.get(0)?;
                Ok(Project::new(to_u32(id)?, row.get::<_, String>(1)?))
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;

        let mut task_statement = self.connection.prepare(
            "
            SELECT tasks.id, tasks.title, tasks.description, projects.name,
                   tasks.status, tasks.priority
            FROM tasks
            LEFT JOIN projects ON projects.id = tasks.project_id
            ORDER BY tasks.id
            ",
        )?;
        let tasks = task_statement
            .query_map([], |row| {
                let id: i64 = row.get(0)?;
                let status: String = row.get(4)?;
                let priority: String = row.get(5)?;

                Ok(Task::new(
                    to_u32(id)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get(3)?,
                    parse_status(&status)?,
                    parse_priority(&priority)?,
                ))
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;

        Ok((tasks, projects))
    }

    /// Atomically replace the stored snapshot with the current application state.
    ///
    /// The transaction ensures an interrupted save cannot leave a partially-written
    /// collection of tasks and projects.
    pub fn save(&mut self, tasks: &[Task], projects: &[Project]) -> rusqlite::Result<()> {
        let transaction =
            self.connection.transaction_with_behavior(TransactionBehavior::Immediate)?;

        transaction.execute("DELETE FROM tasks", [])?;
        transaction.execute("DELETE FROM projects", [])?;

        let mut project_ids = HashMap::with_capacity(projects.len());
        for project in projects {
            let id = i64::from(project.id);
            transaction.execute(
                "INSERT INTO projects (id, name) VALUES (?1, ?2)",
                params![id, project.name],
            )?;
            project_ids.insert(project.name.as_str(), id);
        }

        for task in tasks {
            let project_id = match task.project.as_deref() {
                Some(name) => Some(*project_ids.get(name).ok_or_else(|| {
                    rusqlite::Error::InvalidParameterName(format!(
                        "task {} references missing project {name}",
                        task.id
                    ))
                })?),
                None => None,
            };

            transaction.execute(
                "
                INSERT INTO tasks (id, title, description, status, priority, project_id)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                ",
                params![
                    i64::from(task.id),
                    task.title,
                    task.description,
                    task.status.to_string(),
                    task.priority.to_string(),
                    project_id
                ],
            )?;
        }

        transaction.commit()
    }
}

fn to_u32(value: i64) -> rusqlite::Result<u32> {
    u32::try_from(value)
        .map_err(|_| rusqlite::Error::IntegralValueOutOfRange(0, value))
}

fn parse_status(value: &str) -> rusqlite::Result<TaskStatus> {
    match value {
        "Todo" => Ok(TaskStatus::Todo),
        "In Progress" => Ok(TaskStatus::InProgress),
        "Done" => Ok(TaskStatus::Done),
        _ => Err(rusqlite::Error::InvalidQuery),
    }
}

fn parse_priority(value: &str) -> rusqlite::Result<Priority> {
    match value {
        "Low" => Ok(Priority::Low),
        "Normal" => Ok(Priority::Normal),
        "High" => Ok(Priority::High),
        "Urgent" => Ok(Priority::Urgent),
        _ => Err(rusqlite::Error::InvalidQuery),
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::Database;
    use crate::{
        project::Project,
        task::{Priority, Task, TaskStatus},
    };

    #[test]
    fn new_database_starts_empty_and_has_schema_version_one() {
        let database = Database::open(Path::new(":memory:")).unwrap();

        let (tasks, projects) = database.load().unwrap();
        assert!(tasks.is_empty());
        assert!(projects.is_empty());

        let version: i64 = database
            .connection
            .pragma_query_value(None, "user_version", |row| row.get(0))
            .unwrap();
        assert_eq!(version, 1);
    }

    #[test]
    fn saves_and_loads_tasks_and_projects() {
        let mut database = Database::open(Path::new(":memory:")).unwrap();
        let projects = vec![Project::new(7, "Learning")];
        let tasks = vec![
            Task::new(
                12,
                "Study SQLite",
                "Transactions and foreign keys",
                Some("Learning".to_string()),
                TaskStatus::InProgress,
                Priority::Urgent,
            ),
            Task::new(
                13,
                "Review notes",
                "",
                None,
                TaskStatus::Done,
                Priority::Low,
            ),
        ];

        database.save(&tasks, &projects).unwrap();
        let (loaded_tasks, loaded_projects) = database.load().unwrap();

        assert_eq!(loaded_projects, projects);
        assert_eq!(loaded_tasks.len(), 2);
        assert_eq!(loaded_tasks[0].id, 12);
        assert_eq!(loaded_tasks[0].title, "Study SQLite");
        assert_eq!(loaded_tasks[0].project.as_deref(), Some("Learning"));
        assert_eq!(loaded_tasks[0].status, TaskStatus::InProgress);
        assert_eq!(loaded_tasks[0].priority, Priority::Urgent);
        assert_eq!(loaded_tasks[1].project, None);
        assert_eq!(loaded_tasks[1].status, TaskStatus::Done);
    }

    #[test]
    fn data_survives_closing_and_reopening_the_database() {
        use std::time::{SystemTime, UNIX_EPOCH};

        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "taskflow-test-{}-{unique}.sqlite3",
            std::process::id()
        ));

        {
            let mut database = Database::open(&path).unwrap();
            let tasks = vec![Task::new(
                42,
                "Persist across restarts",
                "Stored on disk",
                None,
                TaskStatus::InProgress,
                Priority::High,
            )];
            database.save(&tasks, &[]).unwrap();
        }

        let database = Database::open(&path).unwrap();
        let (tasks, projects) = database.load().unwrap();
        assert!(projects.is_empty());
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].id, 42);
        assert_eq!(tasks[0].title, "Persist across restarts");

        drop(database);
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn saving_project_deletion_moves_tasks_to_inbox() {
        let mut database = Database::open(Path::new(":memory:")).unwrap();
        let projects = vec![Project::new(1, "Work")];
        let tasks = vec![Task::new(
            1,
            "Finish report",
            "",
            Some("Work".to_string()),
            TaskStatus::Todo,
            Priority::Normal,
        )];

        database.save(&tasks, &projects).unwrap();

        let mut inbox_task = tasks[0].clone();
        inbox_task.project = None;
        database.save(&[inbox_task], &[]).unwrap();

        let (loaded_tasks, loaded_projects) = database.load().unwrap();
        assert!(loaded_projects.is_empty());
        assert_eq!(loaded_tasks.len(), 1);
        assert_eq!(loaded_tasks[0].project, None);
    }

    #[test]
    fn failed_snapshot_does_not_partially_replace_saved_data() {
        let mut database = Database::open(Path::new(":memory:")).unwrap();
        let original = vec![Task::new(
            1,
            "Keep this task",
            "",
            None,
            TaskStatus::Todo,
            Priority::Normal,
        )];
        database.save(&original, &[]).unwrap();

        let invalid = vec![Task::new(
            2,
            "Invalid project reference",
            "",
            Some("Missing".to_string()),
            TaskStatus::Todo,
            Priority::Normal,
        )];
        assert!(database.save(&invalid, &[]).is_err());

        let (loaded_tasks, _) = database.load().unwrap();
        assert_eq!(loaded_tasks.len(), 1);
        assert_eq!(loaded_tasks[0].title, "Keep this task");
    }
}
