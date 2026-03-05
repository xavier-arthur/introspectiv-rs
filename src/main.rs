use yew::prelude::*;

#[component]
fn App() -> Html {
    html! {
        <p>{ "It works"}</p>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
