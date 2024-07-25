use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct UploadProjectRequest {
    pub name: String,
    pub description: String,
    pub content: String, // Représentation du contenu du projet
}

#[derive(Serialize)]
pub struct UploadProjectResponse {
    pub status: String,
}

#[derive(Serialize)]
pub struct Project {
    pub name: String,
    pub description: String,
}

#[derive(Serialize)]
pub struct ProjectListResponse {
    pub projects: Vec<Project>,
}

