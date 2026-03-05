use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d, HtmlElement};

pub struct Grid {
    canvas_ref: NodeRef,
    grid: Vec<bool>,
    cols: i32,
}

#[derive(Properties, PartialEq)]
pub struct GridProps {
    pub color: Option<String>,
    pub reset_trigger: u32,
    pub autoplay_interval: u32
}

pub enum Msg {
    CellClicked(MouseEvent),
    Reset,
}

impl Grid {
    fn color(ctx: &Context<Self>) -> String {
        ctx.props().color
            .clone()
            .unwrap_or_else(|| String::from("#5e5848"))
    }

    fn draw(&self, ctx: &Context<Self>) {
        let canvas = self.canvas_ref.cast::<HtmlCanvasElement>().unwrap();
        let cell_size = 24.0;

        let rows = (canvas.height() as f64 / cell_size).floor() as i32;
        let render_ctx: CanvasRenderingContext2d = canvas
            .get_context("2d").unwrap().unwrap()
            .dyn_into().unwrap();

        render_ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
        render_ctx.set_stroke_style_str(&Self::color(ctx));
        render_ctx.set_line_width(0.5);
        render_ctx.set_fill_style_str(&Self::color(ctx));

        for x in 0..self.cols {
            for y in 0..rows {
                let index = (y * self.cols + x) as usize;

                render_ctx.stroke_rect(
                    x as f64 * cell_size,
                    y as f64 * cell_size,
                    cell_size,
                    cell_size,
                );

                if self.grid.get(index).copied().unwrap_or(false) {
                    render_ctx.fill_rect(
                        x as f64 * cell_size,
                        y as f64 * cell_size,
                        cell_size,
                        cell_size,
                    );
                }
            }
        }
    }

    fn advance_game_state(&self, ctx: &Context<Self>) {
        todo!()
    }
}

impl Component for Grid {
    type Message = Msg;
    type Properties = GridProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            canvas_ref: NodeRef::default(),
            grid: Vec::new(),
            cols: 0,
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if !first_render {
            return;
        }

        let canvas = self.canvas_ref.cast::<HtmlCanvasElement>().unwrap();
        let elem: &HtmlElement = canvas.as_ref();
        let rect = elem.get_bounding_client_rect();

        canvas.set_width(rect.width() as u32);
        canvas.set_height(rect.height() as u32);

        self.cols = (rect.width() / 24.0).floor() as i32;
        let rows = (rect.height() / 24.0).floor() as i32;

        self.grid = vec![false; (self.cols * rows) as usize];

        self.draw(ctx);
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::CellClicked(e) => {
                let cell_size = 24.0;

                let canvas = self.canvas_ref.cast::<HtmlCanvasElement>().unwrap();

                let rect = canvas.get_bounding_client_rect();

                let x = e.client_x() as f64 - rect.left();
                let y = e.client_y() as f64 - rect.top();

                let cell_x = (x / cell_size).floor() as i32;
                let cell_y = (y / cell_size).floor() as i32;
                let index = (cell_y * self.cols + cell_x) as usize;

                if let Some(cell) = self.grid.get_mut(index) {
                    *cell = !*cell;
                }

                self.draw(ctx);

                false
            }

            Msg::Reset => {
                self.grid.fill(false);
                self.draw(ctx);

                false
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        if ctx.props().reset_trigger != old_props.reset_trigger {
            ctx.link().send_message(Msg::Reset);
        }

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(Msg::CellClicked);

        html! {
            <canvas ref={self.canvas_ref.clone()} {onclick} class="size-full" />
        }
    }
}