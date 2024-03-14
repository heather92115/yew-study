use yew::prelude::*;
use yew_router::prelude::Link;
use crate::Route;

pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <section>
                <div>
                    <div>
                        <figure>
                            <img src="logo.jpg" class="logo" />
                        </figure>
                        <h1>{ "Welcome to Grow My Vocab!" }</h1>
                        <p>{ "Expand your vocabulary with fun and engaging exercises every day." }</p>
                    </div>
                </div>
                <Link<Route> classes={classes!("navbar-item")} to={Route::Study}>
                            { "Study" }
                </Link<Route>>
            </section>
        }
    }
}