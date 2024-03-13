#![recursion_limit = "1024"]

use yew::prelude::*;
use yew_router::prelude::*;

mod pages;
mod sl;

use pages::{home::Home, study::StudyProps};
struct App;

#[derive(Switch, Debug, Clone, PartialEq)]
pub enum AppRoute {
    #[to = "/study"]
    Study,
    #[to = "/"]
    Index,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {

        html! {
            <>
            <div class="container">

                <main>
                <Router<AppRoute, ()>
                    render = Router::render(|switch: AppRoute| {
                        match switch {
                            AppRoute::Study => html!{<StudyProps/>},
                            AppRoute::Index => html!{<Home/>},
                        }
                    })
                />
                </main>
            </div>

            </>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
