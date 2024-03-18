#![recursion_limit = "1024"]

use yew::{function_component, html, Html};
use yew_router::prelude::*;
use yew_study::route::{switch, Route};

/// The `Main` component serving as the root of the Yew-based web application.
///
/// This component sets up client-side routing using `yew_router`, allowing the application
/// to navigate between different pages without reloading the web page. It acts as the
/// central routing hub, deciding which page component to render based on the current URL.
///
/// ## Features:
/// - **Client-Side Routing**: Uses `BrowserRouter` and `Switch` to manage routing. This enables
///   seamless navigation and dynamic page rendering based on the URL path.
/// - **Route Configuration**: Defines routes in the `Route` enum and associates them with
///   different page components. The `switch` function maps each route to its corresponding
///   component, ensuring the correct page is displayed.
/// - **Reusable Layout**: Encapsulates the `Switch` router within a `main` HTML element, providing
///   a consistent layout structure across different pages. This can be expanded to include
///   site-wide elements like navigation bars or footers.
/// - **I18N Support**: Language models are loaded and usable throughout the entire component set.
///
/// ## Usage:
/// The `Main` component is used as the entry point for rendering the application's UI.
/// It's invoked by the `main` function with `yew::Renderer::<Main>::new().render();`, which
/// mounts the component to the DOM and starts the Yew application.
///
/// ## Example Routes:
/// - `/`: Renders the `Home` component as the landing page.
/// - `/study`: Renders the `Study` page for vocabulary activities.
/// - `/404`: Renders the `PageNotFound` component for unmatched routes.
///
/// Note: To add or modify routes, adjust the `Route` enum and the `switch` function accordingly.
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

/// Entry point for the Yew-based web application "Grow Your Vocabulary".
///
/// This application uses the Yew framework to render a single-page application (SPA) with
/// client-side routing. It's built and served using Trunk, a WASM web application bundler for Rust.
///
/// ## Features:
/// - **Yew Framework**: Utilizes Yew for its component-based architecture, enabling a modular and
///   reactive UI development approach.
/// - **Client-Side Routing**: Employs `yew_router` for managing navigation and rendering different
///   pages without reloading the browser.
/// - **SCSS and CSS**: Styles are managed with SCSS and CSS, bundled and applied through Trunk.
/// - **Assets Management**: Static assets like favicon and logo are managed by Trunk, ensuring they're
///   correctly bundled and available in the final build.
/// - **Proxy Configuration**: For development, Trunk's proxy feature is configured to forward
///   `/gql` requests to a backend server, facilitating seamless frontend-backend integration.
///
/// ## Pages:
/// - **Home**: The landing page of the application, accessible at the root `/` path.
/// - **Study**: A study page to engage with vocabulary activities, accessible at `/study`.
/// - **NotFound**: A fallback page for unmatched routes, providing user feedback for broken or
///   incorrect URLs.
///
/// ## Running the Application:
/// Ensure that Trunk is installed and run `trunk serve` from the terminal in the project's root directory.
/// This will compile the Rust code to WASM, bundle the application, and serve it on `localhost` at the
/// specified port (default `3001` for this project). Proxy settings in `Trunk.toml` allow for API requests
/// to be forwarded to the backend server.
///
/// The application's structure and behavior are defined in the `Main` function component, which sets up
/// the routing and renders the corresponding page components based on the current URL.
fn main() {
    yew::Renderer::<Main>::new().render();
}
