use yew::prelude::*;

use crate::components::game_of_life::Grid;

pub struct Gol;

impl Component for Gol {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self { Self }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <Grid
                color={Option::<String>::None}
                reset_trigger={0}
                autoplay_interval={0}
            />
        }
    }
}