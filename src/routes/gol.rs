use wasm_bindgen::JsCast;
use web_sys::window;
use yew::prelude::*;

use crate::components::game_of_life::Grid;

pub enum Msg {
    Reset,
    Advance,
    Randomize
}

pub struct Gol {
    reset_trigger: u32,
    advance_trigger: u32,
    randomize_trigger: u32,

    #[allow(dead_code)]
    key_listener: Option<gloo_events::EventListener> // this is just for storing
}

impl Component for Gol {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();

        let key_listener = gloo_events::EventListener::new(
            &window().unwrap(),
            "keyup",
            move |generic_event| {
                let e = generic_event.dyn_ref::<KeyboardEvent>().unwrap();

                if e.key() == " " {
                    link.send_message(Msg::Advance);
                }
            }
        );

        Self {
            randomize_trigger: 0,
            reset_trigger: 0,
            advance_trigger: 0,
            key_listener: Some(key_listener)
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Reset => {
                self.reset_trigger += 1;

                true
            },

            Msg::Randomize => {
                self.randomize_trigger += 1;

                true
            },

            Msg::Advance => {
                self.advance_trigger += 1;

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let reset = ctx.link().callback(|_| Msg::Reset);
        let advance = ctx.link().callback(|_| Msg::Advance);
        let randomize = ctx.link().callback(|_| Msg::Randomize);

        html! {
            <>
                <div class="h-12 flex justify-between">
                    <div class="inline-flex items-center">
                        <input placeholder="Autoplay (ms)" type="text" class="px-2 rounded-l-lg border h-[calc(100%-1rem)]" />
                        <button onclick={reset} class="rounded-r-lg p-2 bg-dark-1 h-fit text-xs">{ "Clear" }</button>
                    </div>

                    <div class="inline-flex items-center">
                        <button onclick={randomize} class="rounded-lg p-2 bg-dark-1 h-fit text-xs self-center mr-2">{ "Random" }</button>
                        <button onclick={advance} class="rounded-lg p-2 bg-dark-1 h-fit text-xs self-center mr-2">{ "Advance" }</button>
                    </div>
                </div>

                <div class="h-[calc(100%-3rem)]">
                    <Grid
                        color={Option::<String>::None}
                        reset_trigger={self.reset_trigger}
                        advance_trigger={self.advance_trigger}
                        randomize_trigger={self.randomize_trigger}
                        autoplay_interval={0}
                    />
                </div>
            </>
        }
    }
}