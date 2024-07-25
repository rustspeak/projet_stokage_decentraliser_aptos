use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use reqwest::Client;

#[function_component(UploadForm)]
pub fn upload_form() -> Html {
    let name = use_state(|| "".to_string());
    let description = use_state(|| "".to_string());
    let content = use_state(|| "".to_string());

    let oninput_name = {
        let name = name.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            name.set(input.value());
        })
    };

    let oninput_description = {
        let description = description.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            description.set(input.value());
        })
    };

    let oninput_content = {
        let content = content.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            content.set(input.value());
        })
    };

    let onclick = {
        let name = name.clone();
        let description = description.clone();
        let content = content.clone();
        Callback::from(move |_| {
            let name = name.clone();
            let description = description.clone();
            let content = content.clone();
            spawn_local(async move {
                let client = Client::new();
                let response = client.post("http://localhost:8000/upload_project")
                    .json(&serde_json::json!({
                        "name": (*name).clone(),
                        "description": (*description).clone(),
                        "content": (*content).clone()
                    }))
                    .send()
                    .await
                    .unwrap();
                let result: serde_json::Value = response.json().await.unwrap();
                web_sys::console::log_1(&format!("Résultat de l'upload: {}", result["status"]).into());
            });
        })
    };

    html! {
        <div>
            <input type="text" {oninput_name} placeholder="Nom du projet" />
            <input type="text" {oninput_description} placeholder="Description du projet" />
            <input type="text" {oninput_content

} placeholder="Contenu du projet" />
            <button {onclick}>{ "Téléverser le projet" }</button>
        </div>
    }
}