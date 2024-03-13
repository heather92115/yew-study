use yew::prelude::*;

pub struct Home;
impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="tile is-ancestor is-vertical">
                <div class="tile is-child">
                    <figure class="image is-3by1">
                        <img src="logo.jpg" class="logo" />
                    </figure>
                    <h1>{ "Welcome to Grow My Vocab!" }</h1>
                    <p>{ "Expand your vocabulary with fun and engaging exercises every day." }</p>
                </div>
            </div>
        }
    }
}
