use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, window};
use yew::prelude::*;

use crate::components::game_of_life::{Grid, GridCommand};

pub struct Gol {
    command: Option<(u32, GridCommand)>,

    #[allow(dead_code)]
    key_listener: Option<gloo_events::EventListener>,

    autoplay_interval: u32
}

pub enum Msg {
    Command(GridCommand),
    UpdateAutoplay(String)
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
                    link.send_message(Msg::Command(GridCommand::Advance));
                }
            }
        );

        Self {
            command: None,
            key_listener: Some(key_listener),
            autoplay_interval: 0
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Command(g) => {
                let id = if let Some((id, _)) = self.command {
                    id
                } else {
                    0
                };

                self.command = Some((id, g));
            },

            Msg::UpdateAutoplay(new_value) => {
                self.autoplay_interval = new_value.parse::<u32>().unwrap();
            }
        };


        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let reset = ctx.link().callback(|_| Msg::Command(GridCommand::Reset));
        let advance = ctx.link().callback(|_| Msg::Command(GridCommand::Advance));
        let randomize = ctx.link().callback(|_| Msg::Command(GridCommand::Randomize));

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
                        command={self.command.clone()}
                        autoplay_interval={self.autoplay_interval}
                    />
                </div>
            </>
        }
    }
}