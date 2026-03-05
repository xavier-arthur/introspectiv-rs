use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct CellProps {
    pub alive: bool
}

#[component(GolCell)]
pub fn gol_cell(props: &CellProps) -> Html {
    html! {
        <div class="aspect-square border"></div>
    }
}