mod field;
mod rules;

use field::Field;
use rules::standard_rule::StandardRule;
use wasm_bindgen::prelude::*;
use web_sys::{console, CanvasRenderingContext2d};


const CELL_SIZE: u32 = 10;
const GRID_COLOR: &str = "#333333";
const DEAD_COLOR: &str = "#000000";
const ALIVE_COLOR: &str = "#ffffff";

#[wasm_bindgen]
pub struct GameOfLife {
    field: Field,
    width: u32,
    height: u32,
    generation: u32,
}

#[wasm_bindgen]
impl GameOfLife {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32) -> GameOfLife {
        let field = Field::new(width as usize, height as usize);
        GameOfLife {
            field,
            width,
            height,
            generation: 0,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn generation(&self) -> u32 {
        self.generation
    }

    pub fn cells(&self) -> *const bool {
        self.field.cells().as_ptr()
    }

    pub fn tick(&mut self) {
        let rule = StandardRule::new();
        self.field = rule.apply(&self.field);
        self.generation += 1;
    }

    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        self.field.toggle_cell(row as usize, col as usize);
    }

    pub fn clear(&mut self) {
        self.field.clear();
        self.generation = 0;
    }

    pub fn randomize(&mut self) {
        self.field.randomize();
        self.generation = 0;
    }

    pub fn render(&self, ctx: &CanvasRenderingContext2d) {
        self.draw_grid(&ctx);
        self.draw_cells(&ctx);
    }

    fn draw_grid(&self, ctx: &CanvasRenderingContext2d) {
        ctx.begin_path();
        ctx.set_stroke_style(&JsValue::from_str(GRID_COLOR));

        for i in 0..=self.width {
            let x = (i * CELL_SIZE) as f64;
            ctx.move_to(x, 0.0);
            ctx.line_to(x, (self.height * CELL_SIZE) as f64);
        }

        for j in 0..=self.height {
            let y = (j * CELL_SIZE) as f64;
            ctx.move_to(0.0, y);
            ctx.line_to((self.width * CELL_SIZE) as f64, y);
        }

        ctx.stroke();
    }

    fn draw_cells(&self, ctx: &CanvasRenderingContext2d) {
        ctx.begin_path();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let is_alive = self.field.cells()[idx];

                ctx.set_fill_style(&JsValue::from_str(if is_alive {
                    ALIVE_COLOR
                } else {
                    DEAD_COLOR
                }));

                ctx.fill_rect(
                    (col * CELL_SIZE + 1) as f64,
                    (row * CELL_SIZE + 1) as f64,
                    (CELL_SIZE - 1) as f64,
                    (CELL_SIZE - 1) as f64,
                );
            }
        }

        ctx.stroke();
    }

    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    console::log_1(&"Conway's Game of Life initialized!".into());
}
