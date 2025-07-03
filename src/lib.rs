mod constants;
mod error;
mod field;
mod renderer;
mod rules;

use field::Field;
use renderer::Renderer;
use rules::{standard_rule::StandardRule, Rule};
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

/// The main Game of Life structure exposed to JavaScript
/// JavaScriptに公開されるメインのライフゲーム構造体
#[wasm_bindgen]
pub struct GameOfLife {
    field: Field,
    width: u32,
    height: u32,
    generation: u32,
    renderer: Renderer,
    rule: Box<dyn Rule>,
}

#[wasm_bindgen]
impl GameOfLife {
    /// Create a new Game of Life instance
    /// 新しいライフゲームインスタンスを作成
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32) -> GameOfLife {
        crate::utils::set_panic_hook();
        
        let field = Field::new(width as usize, height as usize);
        GameOfLife {
            field,
            width,
            height,
            generation: 0,
            renderer: Renderer::new(),
            rule: Box::new(StandardRule::new()),
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

    /// Advance the game by one generation
    /// ゲームを1世代進める
    pub fn tick(&mut self) {
        self.field = self.rule.apply(&self.field);
        self.generation += 1;
    }

    /// Toggle a cell's state
    /// セルの状態を切り替える
    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        // Ignore errors for out-of-bounds clicks
        let _ = self.field.toggle_cell(row as usize, col as usize);
    }

    /// Clear all cells
    /// すべてのセルをクリア
    pub fn clear(&mut self) {
        self.field.clear();
        self.generation = 0;
    }

    /// Randomize the field
    /// フィールドをランダム化
    pub fn randomize(&mut self) {
        self.field.randomize();
        self.generation = 0;
    }

    /// Render the game field to a canvas context
    /// ゲームフィールドをキャンバスコンテキストに描画
    pub fn render(&self, ctx: &CanvasRenderingContext2d) {
        self.renderer.render(ctx, &self.field);
    }
}

/// Utility functions for panic debugging
/// パニックデバッグ用のユーティリティ関数
mod utils {
    pub fn set_panic_hook() {
        // When the `console_error_panic_hook` feature is enabled, we can call the
        // `set_panic_hook` function at least once during initialization, and then
        // we will get better error messages if our code ever panics.
        #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();
    }

}

#[wasm_bindgen]
pub fn init() {
    utils::set_panic_hook();
    web_sys::console::log_1(&"Conway's Game of Life initialized! ライフゲームが初期化されました！".into());
}
