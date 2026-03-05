use yew::prelude::*;

#[component(Index)]
pub fn index() -> Html {
    html! {
        <h1 class="text-lg bold">{ "this is just some text on index page" }</h1>
    }
}