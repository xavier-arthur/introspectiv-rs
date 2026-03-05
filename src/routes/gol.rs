/* game of life  */

use lucide_yew::Mouse;
use yew::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d, HtmlElement};

#[derive(Properties, PartialEq)]
pub struct GolProps {
    pub color: Option<String>
}

#[component(Gol)]
pub fn gol(props: &GolProps) -> Html {
    let canvas_ref = use_node_ref();

    let color = props.color
        .as_ref()
        .map(|c| c.clone())
        .unwrap_or_else(|| String::from("#5e5848"));

    let cell_size = 24.0;

    let grid_state = use_state(|| Vec::new());
    let col_state = use_state(|| 0);
    let mounted = use_state(|| false);

    {
        let canvas_ref = canvas_ref.clone();
        let grid_state_clone_effect = grid_state.clone();
        let col_state_clone_effect = col_state.clone();

        use_effect_with(
            (),
            move |_| {
                crate::log!("running");

                let canvas = canvas_ref.cast::<HtmlCanvasElement>().unwrap();

                let elem: &HtmlElement = canvas.as_ref();
                let rect = elem.get_bounding_client_rect();

                let width = rect.width();
                let height = rect.height();

                canvas.set_width(width as u32);
                canvas.set_height(height as u32);

                let ctx: CanvasRenderingContext2d =
                    canvas.get_context("2d").unwrap().unwrap()
                    .dyn_into().unwrap();

                let cols = (width / cell_size).floor() as i32;
                let rows = (height / cell_size).floor() as i32;

                col_state_clone_effect.set(cols);

                let initial_grid = vec![false; (cols * rows) as usize];
                grid_state_clone_effect.set(initial_grid);

                #[allow(deprecated)]
                ctx.set_stroke_style_str(&color);
                ctx.set_line_width(0.5);
                ctx.set_fill_style_str(&color);

                for x in 0..cols {
                    for y in 0..rows {
                        let index = (y * cols + x) as usize;

                        ctx.stroke_rect(
                            x as f64 * cell_size,
                            y as f64 * cell_size,
                            cell_size,
                            cell_size,
                        );

                        if grid_state_clone_effect[index] {
                            ctx.fill_rect(
                                x as f64 * cell_size,
                                y as f64 * cell_size,
                                cell_size,
                                cell_size
                            );
                        }
                    }
                };

                mounted.set(true);
            }
        );
    }

    let onclick = {
        let canvas_ref = canvas_ref.clone();
        let col_state_clone = col_state.clone();
        let grid_state_clone = grid_state.clone();

        Callback::from(move |e: MouseEvent| {
            let canvas = canvas_ref.cast::<HtmlCanvasElement>().unwrap();
            let rect = canvas.get_bounding_client_rect();

            let x = e.client_x() as f64 - rect.left();
            let y = e.client_y() as f64 - rect.top();

            let cell_x = (x / cell_size).floor() as usize;
            let cell_y = (y / cell_size).floor() as usize;

            let cols = *col_state_clone as usize;
            let index = cell_y * cols + cell_x;

            let mut new_grid = (*grid_state_clone).clone();
            new_grid[index] = !new_grid[index];
            grid_state_clone.set(new_grid);

            crate::log!("{:?}", *grid_state_clone);
        })
    };

    html! {
        <canvas onclick={onclick} ref={canvas_ref} class="size-full"></canvas>
    }
}