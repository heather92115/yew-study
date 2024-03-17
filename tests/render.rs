#[cfg(test)]
use wasm_bindgen_test::*;
use yew::prelude::*;
use yew_study::pages::{home::Home, page_not_found::PageNotFound, study::Study};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn page_not_found_component_loads() {
    let _app: Html = html! {
        <PageNotFound />
    };
    // The test passes if the component is created without panicking.
    // Note: This does not directly assert anything about the component's HTML output.
}

#[wasm_bindgen_test]
fn home_component_loads() {
    let _app: Html = html! {
        <Home />
    };
    // The test passes if the component is created without panicking.
}

#[wasm_bindgen_test]
fn study_component_loads() {
    let _app: Html = html! {
        <Study />
    };
    // The test passes if the component is created without panicking.
}
