use yew::{Component, Context, Html, html, Properties};
use crate::debug_object;
use crate::sl::study::Challenge;

#[derive(Properties, PartialEq, Clone, Debug)]
pub struct PromptProps {
    pub challenge: Challenge
}

pub enum Msg {
    Help,
}
pub struct Prompt {
    props: PromptProps,
    available_hints: Vec<String>,
    display_hints: Vec<String>,
}

impl Prompt {

    pub fn determine_hints(&mut self, props: PromptProps) -> Vec<String> {
        let mut hints = Vec::new();

        let challenge = props.challenge;

        if !challenge.pos.is_empty() {
            hints.push(
                format!("    Part of Speech: {}", challenge.pos)
            );
        }

        if !challenge.infinitive.is_empty() {
            hints.push(
                format!("    Infinitive: {}", challenge.infinitive)
            );
        }

        if !challenge.hint.is_empty() {
            hints.push(
                format!("    Other Hints: {}", challenge.hint)
            );
        }

        if !challenge.user_notes.is_empty() {
            hints.push(
                format!("    Your Notes: {}", challenge.user_notes)
            );
        }

        hints
    }
}

impl Component for Prompt {
    type Message = Msg;
    type Properties = PromptProps;

    fn create(ctx: &Context<Self>) -> Self {

        Self {
            props: ctx.props().clone(),
            available_hints: Vec::new(),
            display_hints: Vec::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {

        match msg {
            Msg::Help  => {

                if let Some(hint) = self.available_hints.pop() {
                    self.display_hints.push(hint);
                }

                true
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        if self.props.challenge != ctx.props().challenge {
            // Update component's state based on the new challenge.
            self.props = ctx.props().clone();
            self.available_hints = self.determine_hints(self.props.clone());

            // Return true to indicate that the component needs to re-render with the new props.
            true
        } else {
            false
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let hints: Vec<Html> = self.display_hints.iter().map(|hint| html! { <p>{ hint }</p> }).collect();

        debug_object!(hints);

        html! {
            <div>
                <p> { format!("Translate: {}", self.props.challenge.first_lang.clone()) } </p>
                <p> { format!("    Words in phrase: {}", self.props.challenge.num_learning_words.clone()) } </p>
                { for hints }

                if !self.available_hints.is_empty() {
                    <a onclick={ctx.link().callback(|_| Msg::Help)}>{ "give me a hint" }</a>
                }
            </div>
        }
    }
}