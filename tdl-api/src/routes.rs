use actix_web::web;

use crate::controllers::{hello_controller, echo_controller, task_controller};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(hello_controller::hello));
    cfg.route("/echo", web::post().to(echo_controller::echo));

    cfg.route("/tasks", web::post().to(task_controller::create_task));
    cfg.route("/tasks", web::get().to(task_controller::get_all_tasks));
    cfg.route("/tasks/{id}", web::get().to(task_controller::get_task_by_id));

    cfg.route("/tasks/{id}/complete", web::patch().to(task_controller::complete_task_by_id));
    cfg.route("/tasks/{id}/priority", web::patch().to(task_controller::set_priority_by_id));

    cfg.route("/tasks/{id}", web::delete().to(task_controller::delete_task_by_id));
}
