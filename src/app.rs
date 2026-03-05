use yew::prelude::*;
use crate::routes::Route;

use crate::routes::index::Index;
use crate::routes::about::About;

fn switch(routes: Route) -> Html {
    match routes {
        Route::Index => html! { <Index /> },
        Route::About => html! { <About /> },
    }
}

#[component]
pub fn App() -> Html {
    html! {
        <Index />
    }
}