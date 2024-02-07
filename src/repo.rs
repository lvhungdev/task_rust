use rusqlite::types::{FromSql, FromSqlResult, ToSql, ToSqlOutput, Value, ValueRef};
use rusqlite::Connection;

use crate::task::Priority;
use crate::{error::Result, task::Task};

pub struct Repo {
    conn_str: String,
}

// TODO Better error handling
impl Repo {
    pub fn new() -> Self {
        let conn_str: String = match directories::BaseDirs::new() {
            Some(base_dir) => match base_dir.data_local_dir().to_str() {
                Some(path) => format!("{}/task_db.sqlite", path),
                None => "./task_db.sqlite".to_string(),
            },
            None => "./task_db.sqlite".to_string(),
        };

        return Self { conn_str };
    }

    pub fn init(&self) -> Result<()> {
        let conn: Connection = Connection::open(&self.conn_str)?;

        conn.execute(
            "
            CREATE TABLE IF NOT EXISTS tasks
            (
                id             INTEGER PRIMARY KEY,
                description    TEXT NOT NULL,
                created_date   DATETIME NOT NULL,
                completed_date DATETIME,
                is_completed   INTEGER NOT NULL,
                due_date       DATETIME,
                priority       INTEGER
            )
            ",
            (),
        )?;

        return Ok(());
    }

    pub fn get(&self, is_completed: bool) -> Result<Vec<Task>> {
        let conn: Connection = Connection::open(&self.conn_str)?;

        let mut statement = conn.prepare(
            "
            SELECT
            id,
            description,
            created_date,
            completed_date,
            is_completed,
            due_date,
            priority
            FROM tasks
            WHERE is_completed = ?1
            ",
        )?;

        let tasks: Vec<Task> = statement
            .query_map([is_completed], |row| {
                Ok(Task {
                    id: row.get(0)?,
                    description: row.get(1)?,
                    created_date: row.get(2)?,
                    completed_date: row.get(3)?,
                    is_completed: row.get(4)?,
                    due_date: row.get(5)?,
                    priority: row.get(6)?,
                })
            })?
            .map(|m| m.unwrap())
            .collect();

        return Ok(tasks);
    }

    pub fn add(&self, task: &Task) -> Result<()> {
        let conn: Connection = Connection::open(&self.conn_str)?;

        conn.execute(
            "
            INSERT INTO tasks
            (
                description,
                created_date,
                completed_date,
                is_completed,
                due_date,
                priority
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            ",
            (
                &task.description,
                task.created_date,
                task.completed_date,
                task.is_completed,
                task.due_date,
                &task.priority,
            ),
        )?;

        return Ok(());
    }

    pub fn update(&self, task: &Task) -> Result<()> {
        let conn: Connection = Connection::open(&self.conn_str)?;

        conn.execute(
            "
            UPDATE tasks
            SET
            description = ?1,
            created_date = ?2,
            completed_date = ?3,
            is_completed = ?4,
            due_date = ?5,
            priority = ?6
            WHERE id = ?7
            ",
            (
                &task.description,
                task.created_date,
                task.completed_date,
                task.is_completed,
                task.due_date,
                &task.priority,
                task.id,
            ),
        )?;

        return Ok(());
    }
}

impl FromSql for Priority {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        return match value {
            ValueRef::Integer(0) => Ok(Priority::High),
            ValueRef::Integer(1) => Ok(Priority::Medium),
            ValueRef::Integer(2) => Ok(Priority::Low),
            _ => Err(rusqlite::types::FromSqlError::InvalidType),
        };
    }
}

impl ToSql for Priority {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        return match self {
            Priority::High => Ok(ToSqlOutput::Owned(Value::Integer(0))),
            Priority::Medium => Ok(ToSqlOutput::Owned(Value::Integer(1))),
            Priority::Low => Ok(ToSqlOutput::Owned(Value::Integer(2))),
        };
    }
}
