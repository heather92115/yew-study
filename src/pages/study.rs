use wasm_bindgen_futures::spawn_local;
use yew::{Component, ComponentLink, html, Html, ShouldRender};

use crate::sl::study::fetch_vocab_study_list;

pub struct Study {
    list: String,
    link: ComponentLink<Self>,
}

pub enum Msg {
    UpdateList(String),
}

impl Component for Study {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            list: String::new(),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateList(res) => {
                self.list = res;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="tile is-ancestor is-vertical">
                <div class="tile is-child hero">
                    <div class="hero-body container pb-0">
                        <h1 class="title is-1">{ "Welcome..." }</h1>
                        <h2 class="subtitle">{ "...to the best yew content" }</h2>
                        <h3> { self.list.clone() } </h3>
                    </div>
                </div>
            </div>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        let link = self.link.clone();
        if first_render {
            spawn_local(async move {
                let res = fetch_vocab_study_list(1, 3).await;
                link.send_message(Msg::UpdateList(res.unwrap()))
            });
        }
    }
}
