use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d, HtmlElement};

pub struct Grid {
    canvas_ref: NodeRef,
    grid: Vec<u8>,
    cols: i32,
    rows: i32,
}

#[derive(Clone, PartialEq)]
pub enum GridCommand {
    Advance,
    Reset,
    Randomize
}

#[derive(Properties, PartialEq)]
pub struct GridProps {
    pub command: Option<(u32, GridCommand)>,
    pub color: Option<String>,
}

pub enum Msg {
    CellClicked(MouseEvent),
    Command(GridCommand)
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

        let render_ctx: CanvasRenderingContext2d = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into().unwrap();

        render_ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
        render_ctx.set_stroke_style_str(&Self::color(ctx));
        render_ctx.set_line_width(0.5);
        render_ctx.set_fill_style_str(&Self::color(ctx));

        for x in 0..self.cols {
            for y in 0..self.rows {
                let index = (y * self.cols + x) as usize;

                render_ctx.stroke_rect(
                    x as f64 * cell_size,
                    y as f64 * cell_size,
                    cell_size,
                    cell_size,
                );

                if self.grid.get(index).copied().unwrap_or(0) > 0 {
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

    fn get_neighbors(&self, index: isize) -> Vec<u8> {
        let cols = self.cols as isize;
        let len = self.grid.len() as isize;

        let offsets = [
            (-cols - 1), (-cols), (-cols + 1),
            (-1),         /* (0, 0) */ 1,
            (cols - 1),  cols,  (cols + 1)
        ];

        offsets.into_iter()
            .filter_map(|off| {
                let superindex = off + index;

                if superindex < 0 || superindex >= len {
                    return None;
                }

                Some(self.grid[superindex as usize])
            })
            .collect()
    }

    fn randomize(&mut self) -> &mut Self {
        let mut new_list = vec![0; self.grid.len()];

        crate::log!("{:?}", self.grid.len());

        for idx in 0..self.grid.len() {
            new_list[idx] = rand::random::<bool>() as u8;
        }

        self.grid = new_list;

        self
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
            rows: 0
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
        self.rows = (rect.height() / 24.0).floor() as i32;

        self.grid = vec![0; (self.cols * self.rows) as usize];

        self.draw(ctx);
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Command(GridCommand::Advance) => {

                let mut new_grid = vec![0; self.grid.len()];

                for idx in 0..self.grid.len() {
                    let neighbours = self.get_neighbors(idx as isize);

                    let alive = {
                        let mut c = 0;

                        for n in neighbours.iter() {
                            if *n > 0 { c += 1 }
                        }

                        c
                    };

                    // apply rules of an alive cell
                    if self.grid[idx] >  0 {
                        new_grid[idx] = if alive < 2 || alive > 3 {
                            0
                        } else {
                            1
                        };
                    } else {

                        if alive == 3 {
                            new_grid[idx] = 1;
                        }
                    }
                }

                    self.grid = new_grid;
                    self.draw(ctx);

                    false
            },

            Msg::Command(GridCommand::Reset) => {
                self.grid.fill(0);
                self.draw(ctx);

                false
            },

            Msg::Command(GridCommand::Randomize) => {
                self.randomize();
                self.draw(ctx);

                false
            }

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
                    *cell = *cell ^ 1;

                    // if *cell == 1 {
                    //     crate::log!("{:?}", self.get_neighbors(index as isize));
                    // }

                }

                self.draw(ctx);

                false
            },
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        if ctx.props().command != old_props.command {
            if let Some((_id, c)) = &ctx.props().command {
                ctx.link().send_message(Msg::Command(c.clone()));
            }
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