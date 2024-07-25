module Storage {

    resource struct Project {
        name: vector<u8>,
        description: vector<u8>,
        content: vector<u8>,
    }

    public fun upload_project(account: &signer, name: vector<u8>, description: vector<u8>, content: vector<u8>) {
        let project = Project {
            name,
            description,
            content,
        };
        move_to(account, project);
    }

    public fun list_projects(): vector<vector<u8>> {
        // Simuler la récupération des projets
        let projects: vector<vector<u8>> = vector::empty();
        projects
    }
}
