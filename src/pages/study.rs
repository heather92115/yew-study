use std::vec::IntoIter;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen_futures::spawn_local;
use web_sys::{Event, FocusEvent, HtmlInputElement, InputEvent, KeyboardEvent, MouseEvent};

use yew::{html, Component, Context, Html};
use crate::sl::study::{fetch_vocab_study_list, Challenge};

pub enum Msg {
    UpdateList(Vec<Challenge>),
    UpdateAnswer(String),
    CheckAnswer,
    FetchError(String),
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
                true
            }
            Msg::UpdateAnswer(answer) => {
                self.answer = answer;

                web_sys::console::log_1(&format!("pages/study answer: {}", self.answer).into());

                true
            }
            Msg::CheckAnswer => {
                web_sys::console::log_1(&format!("pages/study CheckAnswer: {}", self.answer).into());
                true
            },
            Msg::FetchError(_err) => true,
        }
    }

    fn view(& self, ctx: &Context<Self>) -> Html {

        let link = ctx.link().clone();

        let oninput = link.callback(|e: InputEvent| {
            let event: Event = e.dyn_into().unwrap_throw();
            let event_target = event.target().unwrap_throw();
            let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
            Msg::UpdateAnswer(target.value())
        });

        let onblur = link.callback(|e: FocusEvent| {
            let event: Event = e.dyn_into().unwrap_throw();
            let event_target = event.target().unwrap_throw();
            let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
            Msg::UpdateAnswer(target.value())
        });

        let onkeypress = link.callback(|e: KeyboardEvent| {

            let event: Event = e.clone().dyn_into().unwrap_throw();
            let event_target = event.target().unwrap_throw();
            let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
            if e.key() == "Enter" {
                Msg::CheckAnswer
            } else {
                Msg::UpdateAnswer(target.value())
            }
        });

        let onmouseover = |e: MouseEvent| {
            let event: Event = e.dyn_into().unwrap_throw();
            let event_target = event.target().unwrap_throw();
            let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();

            target
                .focus()
                .unwrap_or_default();
        };

        html! {
            <section>
                <div>
                <div>
                    <div>
                        <h2>{ "Let's Do This" }</h2>
                        <p> { self.challenge.prompt.clone() } </p>
                        <p>
                            <input
                                id="challenge_taken"
                                type="text"
                                {onmouseover}
                                {onblur}
                                {onkeypress}
                                {oninput}
                            />
                        </p>

                        <button onclick={ctx.link().callback(|_| Msg::CheckAnswer)}>{ "Check" }</button>
                        <p> { self.err_msg.clone() } </p>
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
