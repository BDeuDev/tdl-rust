use serde::{Serialize, Deserialize};
use sqlx::{FromRow};



#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "priority", rename_all = "lowercase")]
pub enum Priority {
    Low,
    Medium,
    High,
}


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub completed: bool,
    pub priority: Option<Priority>,
    pub created_at: chrono::NaiveDateTime,
}