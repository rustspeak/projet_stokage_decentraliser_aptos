use aptos_sdk::types::account_address::AccountAddress;
use aptos_sdk::rest_client::{Client, FaucetClient};
use crate::model::{UploadProjectRequest, UploadProjectResponse, ProjectListResponse, Project};
use std::error::Error;

pub async fn upload_project(req: UploadProjectRequest) -> Result<UploadProjectResponse, Box<dyn Error>> {
    // Simuler l'upload du projet
    // Vous pouvez implémenter la logique de stockage sur la blockchain Aptos ici
    Ok(UploadProjectResponse { status: "success".to_string() })
}

pub async fn list_projects() -> Result<ProjectListResponse, Box<dyn Error>> {
    // Simuler la récupération de la liste des projets
    let projects = vec![
        Project { name: "Projet 1".to_string(), description: "Description du Projet 1".to_string() },
        Project { name: "Projet 2".to_string(), description: "Description du Projet 2".to_string() },
    ];
    Ok(ProjectListResponse { projects })
}