use yew_router::prelude::*;
use yew::prelude::*;

use about::About;
use index::Index;
use crate::layout::Layout;

pub mod about;
pub mod index;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Index,

    #[at("/about")]
    About,

    #[not_found]
    #[at("/404")]
    NotFound
}

impl Route {
    pub fn switch(route: Self) -> Html {
        match route {
            Route::Index => html! {
                <Layout>
                    <Index />
                </Layout>
            },

            Route::About => html! {
                <Layout>
                    <About />
                </Layout>
            },

            Route::NotFound => html! {
                { "404, not found" }
            }
        }
    }
}