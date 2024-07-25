use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use reqwest::Client;

#[function_component(ProjectList)]
pub fn project_list() -> Html {
    let projects = use_state(|| vec![]);

    {
        let projects = projects.clone();
        use_effect_with_deps(move |_| {
            spawn_local(async move {
                let client = Client::new();
                let response = client.get("http://localhost:8000/list_projects")
                    .send()
                    .await
                    .unwrap();
                let project_list: serde_json::Value = response.json().await.unwrap();
                projects.set(project_list["projects"].as_array().unwrap().iter().map(|p| p.clone()).collect());
            });
            || ()
        }, ());
    }

    html! {
        <div>
            <h2>{ "Liste des Projets" }</h2>
            <ul>
                { for projects.iter().map(|project| html! {
                    <li>
                        <h3>{ project["name"].as_str().unwrap() }</h3>
                        <p>{ project["description"].as_str().unwrap() }</p>
                    </li>
                }) }
            </ul>
        </div>
    }
}