use yew::prelude::*;
use yew_router::prelude::Link;
use crate::route::Route;

pub struct Home;

/// The `Home` component of the application, representing the homepage.
///
/// This component displays the welcome message, a logo, and a brief introduction to the application. It also
/// includes a link to navigate to the `Study` page. The component is stateless, with no message handling or properties.
///
/// ## Implementation Details:
/// - `create`: Initializes the component. As there are no properties or state, it simply returns an instance of `Self`.
/// - `view`: Defines the HTML structure of the homepage, including static text content, an image, and a navigation link.
///
/// ## Usage:
/// This component is intended to be rendered as the main content of the application's root route. It provides
/// users with an overview of the application and an entry point to the `Study` page.
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
