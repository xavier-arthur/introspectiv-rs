use yew_router::prelude::*;
use yew::prelude::*;

pub mod about;
pub mod index;
pub mod gol;


use about::About;
use index::Index;
use gol::Gol;

use crate::layout::Layout;


#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Index,

    #[at("/about")]
    About,

    #[at("/project/gol")]
    Gol,

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

            Route::Gol => html! {
                <Layout>
                    <Gol />
                </Layout>
            },

            Route::NotFound => html! {
                { "404, not found" }
            }
        }
    }
}