use yew_router::prelude::*;
pub mod about;
pub mod index;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Index,

    #[at("/about")]
    About
}