#![recursion_limit = "1024"]

use yew::prelude::*;
use yew_router::prelude::*;

mod pages;
mod sl;

use pages::{home::Home, study::Study};
use crate::pages::page_not_found::PageNotFound;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/study")]
    Study,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Study { } => {
            html! { <Study  /> }
        }
        Route::Home => {
            html! { <Home /> }
        }
        Route::NotFound => {
            html! { <PageNotFound /> }
        }
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
                <main>
                    <Switch<Route> render={switch} />
                </main>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<Main>::new().render();
}
