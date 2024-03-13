use yew::prelude::*;
use yew_router::components::RouterAnchor;
use crate::AppRoute;

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
        type Anchor = RouterAnchor<AppRoute>;

        html! {
            <div>
                <div>
                    <figure>
                        <img src="logo.jpg" class="logo" />
                    </figure>
                    <h1>{ "Welcome to Grow My Vocab!" }</h1>
                    <p>{ "Expand your vocabulary with fun and engaging exercises every day." }</p>
                </div>

                <Anchor route=AppRoute::Study>
                   { "Study" }
                </Anchor>
            </div>
        }
    }
}
