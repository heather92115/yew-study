use yew::{Html, html};
use yew_router::prelude::*;

use crate::pages::{home::Home, study::Study, page_not_found::PageNotFound};

/// Enum representing the routes in the application, used with `yew_router`.
///
/// Each variant corresponds to a different route in the web application, mapped to a specific URL path.
/// This enum is integral to the routing system, facilitating navigation between different components
/// based on the URL.
///
/// ## Variants:
/// - `Home`: The root path (`"/"`), corresponding to the application's home page.
/// - `Study`: The study page (`"/study"`), dedicated to study-related content.
/// - `NotFound`: A catch-all route (`"/404"`) used when a requested route is not found.
///
/// ## Derived Attributes:
/// - `Routable`: Enables integration with `yew_router`, making the enum usable for routing.
/// - `PartialEq`, `Eq`: Allow equality checks between routes, necessary for route comparisons.
/// - `Clone`: Enables cloning of route instances, useful in contexts where route needs to be duplicated.
/// - `Debug`: Facilitates debugging by allowing the route instances to be formatted using the `{:?}` specifier.
///
/// ## Note on Derive:
/// The `derive` macro automatically implements the specified traits for the enum, simplifying the code
/// by avoiding manual implementations. It leverages Rust's metaprogramming capabilities to automatically
/// generate code based on the traits specified, enhancing code maintainability and readability.
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

/// Routes the application to the correct page component based on the current URL.
///
/// This function is the core of the application's routing logic, utilizing `yew_router`.
/// It matches the current route against defined `Route` enum variants and renders
/// the associated component.
///
/// ## Arguments:
/// - `routes`: The current `Route` derived from the URL.
///
/// ## Returns:
/// - `Html`: The Yew `Html` content of the matched route's component.
///
/// ## Supported Routes:
/// - `Route::Study`: Renders the `Study` component.
/// - `Route::Home`: Renders the `Home` component as the landing page.
/// - `Route::NotFound`: Renders the `PageNotFound` component for unmatched routes.
///
/// ## Example:
/// Given a URL path that matches `/study`, the `switch` function will render the `Study` component.
pub fn switch(routes: Route) -> Html {
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
