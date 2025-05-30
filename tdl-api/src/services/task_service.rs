use sqlx::PgPool;
use crate::models::task::{Priority, Task};
pub struct TaskService {
    pool: PgPool,
}

impl TaskService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_all_tasks(&self) -> Result<Vec<Task>, sqlx::Error> {
        let tasks = sqlx::query_as::<_, Task>("SELECT * FROM tasks ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await?;
        Ok(tasks)
    }

    pub async fn create_task(&self, name: &str, priority: &Priority) -> Result<Task, sqlx::Error> {
        let task = sqlx::query_as::<_, Task>(
            "INSERT INTO tasks (name, completed, priority, created_at) VALUES ($1, FALSE, $2, NOW()) RETURNING *",
        )
        .bind(name)
        .bind(priority)
        .fetch_one(&self.pool)
        .await?;
        Ok(task)
    }

    pub async fn get_task_by_id(&self, task_id: i64) -> Result<Task, sqlx::Error> {
        let task = sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE id = $1")
            .bind(task_id)
            .fetch_one(&self.pool)
            .await?;
        Ok(task)
    }

    pub async fn complete_task_by_id(&self, task_id: i64) -> Result<Task, sqlx::Error> {
        let task = sqlx::query_as::<_, Task>("UPDATE tasks SET completed = TRUE WHERE id = $1 RETURNING *")
            .bind(task_id)
            .fetch_one(&self.pool)
            .await?;
        Ok(task)
    }

    pub async fn set_priority_by_id(&self, task_id: i64, priority: &Priority) -> Result<Task, sqlx::Error> {
        let task = sqlx::query_as::<_, Task>("UPDATE tasks SET priority = $2 WHERE id = $1 RETURNING *")
            .bind(task_id)
            .bind(priority)
            .fetch_one(&self.pool)
            .await?;
        Ok(task)
    }

    pub async fn delete_task_by_id(&self, task_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM tasks WHERE id = $1")
            .bind(task_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
