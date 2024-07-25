mod handlers;
mod model;
mod services;

use warp::Filter;

#[tokio::main]
async fn main() {
    let upload_project = warp::path("upload_project")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handlers::upload_project_handler);

    let list_projects = warp::path("list_projects")
        .and(warp::get())
        .and_then(handlers::list_projects_handler);

    let routes = upload_project.or(list_projects);

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}