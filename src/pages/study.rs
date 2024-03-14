use std::vec::IntoIter;
use wasm_bindgen_futures::spawn_local;

use yew::{Component, Context, html, Html};

use crate::sl::study::{Challenge, fetch_vocab_study_list};

pub enum Msg {
    UpdateList(Vec<Challenge>),
    CheckAnswer(String),
    FetchError(String)
}

pub struct Study {
    iterator: IntoIter<Challenge>,
    challenge: Challenge,
    answer: String,
    err_msg: String,
}

impl Component for Study {
    type Message = Msg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            iterator: Vec::new().into_iter(),
            challenge: Challenge::default(),
            answer: "".to_string(),
            err_msg: "".to_string(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateList(res) => {
                self.iterator = res.into_iter();
                self.challenge = self.iterator.next().unwrap_or_default();
            },
            Msg::CheckAnswer(answer) => {
                self.answer = answer;
            },
            Msg::FetchError(_err) => {

            }
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        //let cb = ctx.link().callback(Msg::CheckAnswer);

        html! {
            <section>
                <div>
                <div>
                    <div>
                        <h2>{ "Let's Do This" }</h2>
                        <p> { self.challenge.prompt.clone() } </p>
                        <p><input id="challenge_taken" /></p>
                        <button onclick={ctx.link().callback(|_| Msg::CheckAnswer("".to_string()))}>{ "Check" }</button>
                    </div>
                </div>
                </div>
            </section>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {

        let link = ctx.link().clone();
        if first_render {
            spawn_local(async move {
                let res = fetch_vocab_study_list(1, 3).await;
                if res.is_err() {
                    let err_msg = res.err().clone().unwrap().to_string();
                    link.send_message(Msg::FetchError(err_msg.clone()));
                } else {
                    let list = res.unwrap_or_default();
                    link.send_message(Msg::UpdateList(list.clone()));
                }
            });
        }
    }
}

