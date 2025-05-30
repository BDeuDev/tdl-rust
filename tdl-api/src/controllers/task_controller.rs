use actix_web::{Responder, HttpResponse,web::Data,web};
use serde::Deserialize;
use sqlx::PgPool;
use crate::{models::task::Priority, services::task_service::TaskService};

#[derive(Deserialize)]
pub struct TaskDTO {
    pub name: String,
    pub priority: Priority,
}
#[derive(Deserialize)]
pub struct UpdatePriorityDTO  {
    pub priority: Priority,
}

pub async fn create_task(pool: Data<PgPool>, task_dto: web::Json<TaskDTO>) -> impl Responder {
    let service = TaskService::new(pool.get_ref().clone());

    match service.create_task(&task_dto.name, &task_dto.priority).await {
        Ok(task) => {
            HttpResponse::Created()
                .json(task)
        
        },
        Err(e) => {
            HttpResponse::InternalServerError()
                .body(format!("Error creating task: {}", e))
        }
    }
}

pub async fn get_all_tasks(pool: Data<PgPool>) -> impl Responder {
    let service = TaskService::new(pool.get_ref().clone());

    match service.get_all_tasks().await {
        Ok(tasks) => {
            HttpResponse::Ok().json(tasks)
        },
        Err(e) => {
            HttpResponse::InternalServerError()
                .body(format!("Error fetching tasks: {}", e))
        }
    }
}

pub async fn get_task_by_id(pool: Data<PgPool>, task_id: web::Path<i64>) -> impl Responder {
    let service = TaskService::new(pool.get_ref().clone());

    match service.get_task_by_id(task_id.into_inner()).await {
        Ok(task) => {
            HttpResponse::Ok().json(task)
        },
        Err(e) => {
            HttpResponse::InternalServerError()
                .body(format!("Error fetching task: {}", e))
        }
    }
}

pub async fn complete_task_by_id(pool: Data<PgPool>, task_id: web::Path<i64>) -> impl Responder {
    let service = TaskService::new(pool.get_ref().clone());

    match service.complete_task_by_id(task_id.into_inner()).await {
        Ok(task) => {
            HttpResponse::Ok().json(task)
        },
        Err(e) => {
            HttpResponse::InternalServerError()
                .body(format!("Error completing task: {}", e))
        }
    }
}

pub async fn set_priority_by_id(
    pool: Data<PgPool>,
    task_id: web::Path<i64>,
    task: web::Json<UpdatePriorityDTO>,
) -> impl Responder {
    let service = TaskService::new(pool.get_ref().clone());

    match service.set_priority_by_id(task_id.into_inner(), &task.priority).await {
        Ok(task) => HttpResponse::Ok().json(task),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error setting priority: {}", e)),
    }
}

pub async fn delete_task_by_id(
    pool: web::Data<PgPool>,
    task_id: web::Path<i64>,
) -> impl Responder {
    let service = TaskService::new(pool.get_ref().clone());

    match service.delete_task_by_id(task_id.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error deleting task: {}", e)),
    }
}