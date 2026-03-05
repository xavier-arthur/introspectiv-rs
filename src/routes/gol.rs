use yew::prelude::*;

use crate::components::game_of_life::Grid;

pub enum Msg {
    Reset
}

pub struct Gol {
    reset_trigger: u32
}

impl Component for Gol {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            reset_trigger: 0
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Reset => {
                self.reset_trigger += 1;

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let reset = ctx.link().callback(|_| Msg::Reset);

        html! {
            <>
                <div class="h-12 flex justify-between">
                    <div class="inline-flex items-center">
                        <input placeholder="Autoplay (ms)" type="text" class="px-2 rounded-l-lg border h-[calc(100%-1rem)]" />
                        <button onclick={reset} class="rounded-r-lg p-2 bg-dark-1 h-fit text-xs">{ "Clear" }</button>
                    </div>
                    <button class="rounded-xl p-2 bg-dark-1 h-fit">{ "Iterate" }</button>
                </div>

                <Grid
                    color={Option::<String>::None}
                    reset_trigger={self.reset_trigger}
                    autoplay_interval={0}
                />
            </>
        }
    }
}