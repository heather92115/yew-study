use yew::prelude::*;

pub struct PageNotFound;

/// The `PageNotFound` component, representing a 404 error page.
///
/// This component is displayed when a user navigates to a route that does not exist within the application. It provides
/// a simple message indicating that the page could not be found. Like `Home`, this component is stateless and does not
/// handle any messages or properties.
///
/// ## Implementation Details:
/// - `create`: Initializes the component. Given its stateless nature, it merely returns an instance of `Self`.
/// - `view`: Renders the HTML content for the 404 page, including a title and a subtitle that inform the user
///   that the page does not exist.
///
/// ## Usage:
/// This component should be used as part of the application's routing logic, displayed when no other routes match
/// the current URL. It serves as a catch-all to provide feedback to users when they attempt to access a non-existent
/// page within the application.
impl Component for PageNotFound {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <section>
                <div>
                    <div class="container">
                        <h1 class="title">
                            { "Page not found" }
                        </h1>
                        <h2 class="subtitle">
                            { "Page page does not seem to exist" }
                        </h2>
                    </div>
                </div>
            </section>
        }
    }
}