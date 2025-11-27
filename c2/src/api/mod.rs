use actix_web::web;

pub mod agents;
pub mod tasks;
pub mod ops;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(agents::list_agents)
            .service(agents::register_agent)
            .service(tasks::create_task)
            .service(tasks::list_tasks)
            .service(tasks::list_tasks_for_operation)
            .service(tasks::list_tasks_for_agent)
            .service(ops::start_operation)
            .service(ops::list_operations)
            .service(ops::get_operation)
            .service(ops::start_operation)
    );
}