use yew::prelude::*;
use yew_router::{
    BrowserRouter,
    Switch,
};

use crate::routes::Route;

#[component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Route::switch} />
        </BrowserRouter>
    }
}
