use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, window};
use yew::prelude::*;

use crate::components::game_of_life::{Grid, GridCommand};

pub struct Gol {
    command: Option<(u32, GridCommand)>,

    #[allow(dead_code)]
    key_listener: Option<gloo::events::EventListener>,

    autoplay: Option<Autoplay>
}

struct Autoplay {
    interval: u32,
    last_update: f64
}

pub enum Msg {
    Command(GridCommand),
    UpdateAutoplay(String)
}

fn now() -> f64 {
    web_sys::window().unwrap().performance().unwrap().now()
}

impl Component for Gol {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();

        let key_listener = gloo::events::EventListener::new(
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
            autoplay: None
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
                let interval = if new_value.is_empty() {
                    0u32
                } else {
                    new_value.parse::<u32>().unwrap()
                };

                let now = now();

                self.autoplay = Some(self.autoplay
                    .take()
                    .map_or_else(
                        || {
                            Autoplay {
                                interval,
                                last_update: now,
                            }
                        },

                        |mut ap| {
                            ap.interval = interval;
                            ap.last_update = now;

                            ap
                        }
                    ));
            }
        };


        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let reset = ctx.link().callback(|_| Msg::Command(GridCommand::Reset));
        let advance = ctx.link().callback(|_| Msg::Command(GridCommand::Advance));
        let randomize = ctx.link().callback(|_| Msg::Command(GridCommand::Randomize));

        let on_autoplay_input = ctx.link().callback(|e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();

            Msg::UpdateAutoplay(input.value())
        });

        html! {
            <>
                <div class="h-12 flex justify-between">
                    <div class="inline-flex items-center">
                        <input value={self.autoplay.as_ref().map(|v| v.interval).unwrap_or(0).to_string()} oninput={on_autoplay_input} placeholder="Autoplay (ms)" type="number" class="px-2 rounded-l-lg border h-[calc(100%-1rem)]" />
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
                        autoplay_interval={self.autoplay.as_ref().map(|v| v.interval).unwrap_or(0)}
                    />
                </div>
            </>
        }
    }
}