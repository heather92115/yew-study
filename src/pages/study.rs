use std::vec::IntoIter;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen_futures::spawn_local;
use web_sys::{Event, FocusEvent, HtmlInputElement, InputEvent, KeyboardEvent, MouseEvent};

use yew::{html, Component, Context, Html, NodeRef};
use crate::sl::study::{fetch_vocab_study_list, Challenge, check_vocab_answer};

/// Enumeration of messages that drive the component logic in the study session.
///
/// This enum represents the various types of actions that can occur in the study session,
/// such as updating the list of challenges, submitting an answer, moving to the next challenge,
/// or handling errors. It is used within the `update` function to react appropriately to user
/// interactions or asynchronous operation results.
///
/// ## Variants:
/// - `UpdateList(Vec<Challenge>)`: Updates the internal list of challenges with a new set.
///   This typically happens after fetching a new set of vocabulary challenges from the server.
/// - `UpdateAnswer(String)`: Updates the current answer based on user input. This allows
///   for real-time feedback or validation of the user's response.
/// - `CheckAnswer`: Triggers the validation of the user's answer against the correct response.
///   The outcome of this check may result in updating the UI to show whether the answer was correct or not.
/// - `ShowAnswerResponse(String)`: Updates the UI to display the result of the answer check.
///   This could be a success message, a correction, or a hint for the user.
/// - `NextChallenge`: Advances to the next challenge in the list, updating the UI to reflect
///   the new challenge to be solved.
/// - `FetchError(String)`: Displays an error message in the UI, typically used to indicate
///   problems fetching challenges or submitting answers.
///
/// ## Usage:
/// These messages are central to the reactive nature of the Yew framework, enabling the component
/// to handle a variety of actions and update its state and presentation accordingly.
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

/// Represents the state and behavior of a study session in a vocabulary learning application.
///
/// This struct is a key component of a web assembly application built using the Yew framework,
/// facilitating interactive study sessions where users are challenged to translate words or phrases
/// and receive feedback on their responses.
///
/// ## Fields:
/// - `study_mode`: An enumeration of the different modes the study session can be in,
///    including presenting a new challenge (`Challenge`), showing the outcome of a user's response (`Outcome`),
///    or displaying an error message (`Error`).
/// - `iterator`: An iterator over a collection of `Challenge` items. This allows the application
///    to sequentially present vocabulary challenges to the user.
/// - `challenge`: The current vocabulary challenge being presented to the user. It holds details
///    like the vocabulary ID, study ID, and the prompt for the user.
/// - `prompt`: The textual prompt derived from the current `challenge`, displayed to the user
///    to solicit their response.
/// - `answer`: The user's response to the current `prompt`. This is updated based on user input.
/// - `err_msg`: An error message to be displayed to the user in case of a problem,
///    such as an issue fetching a new challenge or submitting a response.
/// = `button_ref`: Attaches to html button to allow direct programmatic access
///
/// ## Usage:
/// The `Study` struct is instantiated as part of the Yew component lifecycle and is pivotal
/// in managing the flow of a study session. It responds to user input, fetches new challenges,
/// and updates the UI to reflect the current state of the study session.
pub struct Study {
    study_mode: StudyMode,
    iterator: IntoIter<Challenge>,
    challenge: Challenge,
    prompt: String,
    answer: String,
    err_msg: String,
    button_ref: NodeRef,
}

/// `Study` represents a study session within a vocabulary learning web application. This component
/// orchestrates the interaction flow of a study session, handling user responses, checking answers, and navigating
/// between different states of the study process.
///
/// ## Implementation Details:
/// - Utilizes asynchronous requests to interact with the backend for fetching challenges and validating responses.
/// - Employs the `Component` trait from the Yew framework to implement reactive UI updates
/// based on user interactions and asynchronous operations.
impl Study {

    /// Asynchronously fetches the next set of vocabulary study challenges.
    ///
    /// This function initiates a request to fetch a list of vocabulary study challenges for
    /// a specified `awesome_id` and up to a given `limit` of items. The function uses
    /// `spawn_local` to run the fetching process asynchronously, allowing the Yew component to
    /// remain responsive during the data fetching operation.
    ///
    /// Upon successful fetching, it sends a `Msg::UpdateList` message with the retrieved
    /// list of challenges to the component, which triggers the update lifecycle to incorporate
    /// the new challenges into the component's state. If the fetching results in an error,
    /// it sends a `Msg::FetchError` message with the error message, allowing the component
    /// to handle and display the error appropriately.
    ///
    /// ## Parameters:
    /// - `link`: The component's `html::Scope<Self>` link, used to send messages back to the component.
    /// - `awesome_id`: The ID of the `AwesomePerson` for whom the vocabulary list is fetched.
    /// - `limit`: The maximum number of vocabulary challenges to fetch.
    ///
    /// This function demonstrates handling asynchronous operations within a Yew component,
    /// using `spawn_local` for non-blocking network requests and message passing to update
    /// the component's state based on the results of those requests.
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

    /// Submits the user's answer for a vocabulary challenge to be checked and handles the response.
    ///
    /// This function sends the user's answer along with the corresponding challenge to the
    /// server for verification. It processes the response asynchronously, leveraging `spawn_local`
    /// for non-blocking execution. Depending on the server's response, it either triggers an
    /// error message or updates the UI to show the result of the answer check.
    ///
    /// ## Parameters:
    /// - `link`: The `html::Scope<Self>` link for communicating with the Yew component.
    /// - `answer`: The user's answer submitted for the challenge.
    /// - `challenge`: The `Challenge` struct containing details about the current vocabulary item.
    ///
    /// Upon failure, a `Msg::FetchError` message with the error message is sent to the component,
    /// prompting error handling logic. On success, the correct or feedback message is displayed
    /// using a `Msg::ShowAnswerResponse` message, allowing the component to update accordingly.
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

/// The `Study` component manages the study session for vocabulary challenges.
///
/// This component handles the display of vocabulary challenges, checks user answers,
/// and navigates through the vocabulary study set. It operates in three modes: `Challenge`,
/// `Outcome`, and `Error`, controlled by the `study_mode` state.
///
/// ## Lifecycle Methods:
/// - `create`: Initializes the component with default values.
/// - `update`: Handles messages and updates the component's state accordingly.
/// - `view`: Renders the component based on the current state and `study_mode`.
/// - `rendered`: On the first render, loads the next vocabulary list.
///
/// This component demonstrates handling asynchronous operations, user input,
/// and state-driven view rendering in a Yew application.
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
            button_ref: NodeRef::default(),
        }
    }

    /// Handles component messages and updates the state accordingly.
    ///
    /// This function is responsible for reacting to different messages (`Msg`) by updating
    /// the component's state to reflect changes in the UI or to initiate asynchronous operations.
    ///
    /// ## Parameters:
    /// - `ctx`: The component's context, providing access to the component's props and link.
    /// - `msg`: The message received by the component, representing an action to handle.
    ///
    /// Returns `bool` Each case updates the component state and decides whether a re-render is necessary by returning `true`.
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

    /// Renders the component based on the current study mode.
    ///
    /// Depending on the current `study_mode`, this function generates HTML to display
    /// the appropriate UI elements for each study phase: Challenge, Outcome, or Error.
    /// It sets up event handlers for user interactions with the input field and buttons.
    ///
    /// ## Event Handlers:
    /// - `oninput`: Updates the answer as the user types.
    /// - `onblur`: Updates the answer when the input field loses focus.
    /// - `onkeypress`: Checks the answer if the Enter key is pressed, otherwise updates the answer.
    /// - `onmouseover`: Automatically focuses the input field when hovered over.
    ///
    /// ## Study Modes:
    /// - `StudyMode::Challenge`: Displays the current challenge, allowing the user to enter an answer.
    /// - `StudyMode::Outcome`: Displays the outcome after checking an answer, with a button to proceed to the next challenge.
    /// - `StudyMode::Error`: Displays an error message if an issue occurs during the process.
    ///
    /// ## Parameters:
    /// - `ctx`: The component's context, providing access to component's link for creating callbacks.
    ///
    /// Returns `Html` representing the UI of the component in its current state.
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
                                    <button
                                        ref={self.button_ref.clone()}
                                        onclick={ctx.link().callback(|_| Msg::NextChallenge)}>{ "Next" }</button>
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

    /// Called after the component is rendered, allowing for post-render operations.
    ///
    /// This method is invoked every time the component finishes rendering, which can be after
    /// initial mounting or subsequent updates. It provides an opportunity to perform actions that
    /// require the component to be already in the DOM, such as focusing an element or integrating
    /// with third-party libraries that need to manipulate the rendered output.
    ///
    /// ## Parameters:
    /// - `ctx`: The component's context, providing access to the component's props, state, and link.
    /// - `first_render`: A boolean indicating if this is the first render of the component. `true` for
    ///   the first render after component creation, `false` for all subsequent renders.
    ///
    /// ## Behavior:
    /// - On the first render (`first_render` is `true`), it initiates loading the next vocabulary list
    ///   by calling `load_next_vocab_list`
    /// .
    /// - Regardless of the render, if a button reference (`button_ref`) is set and points to a valid
    ///   and present HTML element, it attempts to set focus to that element. This allows the user
    ///  to stay in 'keyboard only' mode.
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        let link = ctx.link().clone();
        if first_render {
            self.load_next_vocab_list(link, 1, 5);
        }

        if let Some(button) = self.button_ref.cast::<web_sys::HtmlElement>() {
            button.focus().unwrap_throw();
        }
    }
}
