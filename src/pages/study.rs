use std::vec::IntoIter;
use wasm_bindgen_futures::spawn_local;
use yew::{Component, ComponentLink, html, Html, ShouldRender};
use yew_router::components::RouterAnchor;
use crate::AppRoute;

use crate::sl::study::{Challenge, fetch_vocab_study_list};

#[derive(Clone)]
pub struct StudyProps {
    iterator: IntoIter<Challenge>,
    challenge: Challenge,
    answer: String,
    link: ComponentLink<Self>,
}

pub enum Msg {
    UpdateList(Vec<Challenge>),
    CheckAnswer(String)
}

impl Component for StudyProps {
    type Message = Msg;

    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            iterator: Vec::new().into_iter(),
            challenge: Challenge::default(),
            answer: "".to_string(),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateList(res) => {
                self.iterator = res.into_iter();
                self.challenge = self.iterator.next().unwrap_or_default();
                true
            },
            Msg::CheckAnswer(answer) => {
                self.answer = answer;
                true
            }

        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {

        type Anchor = RouterAnchor<AppRoute>;

        html! {
            <div>
                <div>
                    <div>
                        <h2>{ "Let's Do This" }</h2>
                        <p> { self.challenge.prompt.clone() } </p>
                        <p><input id="challenge_taken" /></p>
                        <button>{ "Check" }</button>
                    </div>
                </div>
                    <Anchor classes="navbar-item" route=AppRoute::Index>
                      { "Done" }
                      </Anchor>
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
