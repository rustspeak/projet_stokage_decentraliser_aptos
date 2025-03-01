use yew::prelude::*;
use crate::components::{UploadForm, ProjectList};

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        App
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn change(&mut self, _ctx: &Context<Self>, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{ "Système de Stockage Décentralisé de Projets sur Aptos" }</h1>
                <UploadForm />
                <ProjectList />
            </div>
        }
    }
}

