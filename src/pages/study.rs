use std::vec::IntoIter;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen_futures::spawn_local;
use web_sys::{Event, FocusEvent, HtmlInputElement, InputEvent, KeyboardEvent, MouseEvent};

use yew::{html, Component, Context, Html};
use crate::sl::study::{fetch_vocab_study_list, Challenge, check_vocab_answer};

pub enum Msg {
    UpdateList(Vec<Challenge>),
    UpdateAnswer(String),
    CheckAnswer,
    ShowAnswerResponse(String),
    NextChallenge,
    FetchError(String),
}

pub enum StudyMode {
    Challenge,
    Outcome,
    Error
}

pub struct Study {
    study_mode: StudyMode,
    iterator: IntoIter<Challenge>,
    challenge: Challenge,
    prompt: String,
    answer: String,
    err_msg: String,
}

impl Study {

    pub fn load_next_vocab_list(&self, link: html::Scope<Self>, awesome_id: i32, limit: i32) {
        spawn_local(async move {
            let res = fetch_vocab_study_list(awesome_id, limit).await;
            if res.is_err() {
                let err_msg = res.err().clone().unwrap().to_string();
                link.send_message(Msg::FetchError(err_msg.clone()));
            } else {
                let list = res.unwrap_or_default();
                link.send_message(Msg::UpdateList(list.clone()));
            }
        });
    }

    pub fn get_answer_checked(&self, link: html::Scope<Self>, answer: String, challenge: Challenge) {
        spawn_local(async move {
            let res = check_vocab_answer(answer, challenge).await;
            if res.is_err() {
                let err_msg = res.err().clone().unwrap().to_string();
                link.send_message(Msg::FetchError(err_msg.clone()));

            } else {
                let response_prompt = res.unwrap_or_default();
                web_sys::console::log_1(&format!("pages/study response_prompt: {}", response_prompt).into());
                link.send_message(Msg::ShowAnswerResponse(response_prompt));
            }
        });
    }
}

impl Component for Study {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            study_mode: StudyMode::Challenge,
            iterator: Vec::new().into_iter(),
            challenge: Challenge::default(),
            prompt: "".to_string(),
            answer: "".to_string(),
            err_msg: "".to_string(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateList(res) => {
                self.iterator = res.into_iter();
                self.challenge = self.iterator.next().unwrap_or_default();
                self.prompt = self.challenge.prompt.clone();
                self.err_msg = "".to_string();
                self.study_mode = StudyMode::Challenge;

                true
            }
            Msg::UpdateAnswer(answer) => {
                self.answer = answer;
                self.err_msg = "".to_string();
                true
            }
            Msg::CheckAnswer => {
                let link = ctx.link().clone();

                web_sys::console::log_1(&format!("pages/study CheckAnswer: {}", self.answer).into());
                self.get_answer_checked(link, self.answer.clone(), self.challenge.clone());
                self.err_msg = "".to_string();
                true
            },
            Msg::ShowAnswerResponse(prompt) => {
                web_sys::console::log_1(&format!("pages/study ShowAnswerResponse: {}", prompt).into());
                self.prompt = prompt;
                self.err_msg = "".to_string();
                self.study_mode = StudyMode::Outcome;

                true
            },
            Msg::NextChallenge => {
                let link = ctx.link().clone();

                if self.iterator.clone().count() == 0 {
                    self.load_next_vocab_list(link, 1, 5);
                } else {
                    self.challenge = self.iterator.next().unwrap_or_default();
                    self.prompt = self.challenge.prompt.clone();
                    self.err_msg = "".to_string();
                    self.study_mode = StudyMode::Challenge;
                }

                true
            }
            Msg::FetchError(err) => {
                self.err_msg = err;
                self.study_mode = StudyMode::Error;
                true
            },
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
                    {
                        match self.study_mode {
                            StudyMode::Challenge => html! {
                                <>
                                    <h2>{ "Let's Do This" }</h2>
                                    <p> { self.prompt.clone() } </p>
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
                                </>
                            },
                            StudyMode::Outcome => html! {
                                <>
                                    <h2>{ self.prompt.clone() }</h2>
                                    <button onclick={ctx.link().callback(|_| Msg::NextChallenge)}>{ "Next" }</button>
                                </>
                            },
                            StudyMode::Error => html! {
                                <p> { self.err_msg.clone() } </p>
                            },
                        }
                    }
                </div>
            </section>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        let link = ctx.link().clone();
        if first_render {
            self.load_next_vocab_list(link, 1, 5);
        }
    }
}
