use crate::model::{UploadProjectRequest, ProjectListResponse};
use crate::services::{upload_project, list_projects};
use warp::reply::Json;
use warp::Rejection;

pub async fn upload_project_handler(body: UploadProjectRequest) -> Result<Json, Rejection> {
    match upload_project(body).await {
        Ok(response) => Ok(warp::reply::json(&response)),
        Err(_) => Err(warp::reject::custom(warp::http::StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

pub async fn list_projects_handler() -> Result<Json, Rejection> {
    match list_projects().await {
        Ok(response) => Ok(warp::reply::json(&response)),
        Err(_) => Err(warp::reject::custom(warp::http::StatusCode::INTERNAL_SERVER_ERROR)),
    }
}