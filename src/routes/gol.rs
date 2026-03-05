/* game of life  */

use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};

use crate::components::game_of_life::{
    Cell
};

#[component(Gol)]
pub fn gol() -> Html {
    let canvas_ref = use_node_ref();

    {
        let canvas_ref = canvas_ref.clone();

        use_effect(move || {
            let canvas: HtmlCanvasElement = canvas_ref.cast().unwrap();

            let window = web_sys::window().unwrap();
            let width = window.inner_width().unwrap().as_f64().unwrap();
            let height = window.inner_height().unwrap().as_f64().unwrap();

            canvas.set_width(width as u32);
            canvas.set_height(height as u32);

            let ctx: CanvasRenderingContext2d =
                canvas.get_context("2d").unwrap().unwrap()
                .dyn_into().unwrap();

            let cell = 10.0;

            let cols = (width / cell).floor() as i32;
            let rows = (height / cell).floor() as i32;

            for x in 0..cols {
                for y in 0..rows {
                    ctx.stroke_rect(
                        x as f64 * cell,
                        y as f64 * cell,
                        cell,
                        cell,
                    );
                }
            }

            || ()
        });
    }

    html! {
        <canvas ref={canvas_ref} class="w-screen h-screen"></canvas>
    }
}