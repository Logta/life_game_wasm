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

    /// ゲームを1世代進める
    pub fn tick(&mut self) {
        self.field = self.rule.apply(&self.field);
        self.generation += 1;
    }

    /// セルの状態を切り替える
    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        // 範囲外クリックのエラーを無視
        let _ = self.field.toggle_cell(row as usize, col as usize);
    }

    /// すべてのセルをクリア
    pub fn clear(&mut self) {
        self.field.clear();
        self.generation = 0;
    }

    /// フィールドをランダム化
    pub fn randomize(&mut self) {
        self.field.randomize();
        self.generation = 0;
    }

    /// ゲームフィールドをキャンバスコンテキストに描画
    pub fn render(&self, ctx: &CanvasRenderingContext2d) {
        self.renderer.render(ctx, &self.field);
    }
}

/// パニックデバッグ用のユーティリティ関数
mod utils {
    pub fn set_panic_hook() {
        // `console_error_panic_hook`機能が有効な場合、初期化時に少なくとも一度
        // `set_panic_hook`関数を呼び出すことで、コードがパニックした際により良い
        // エラーメッセージを取得できます。
        #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();
    }
}

#[wasm_bindgen]
pub fn init() {
    utils::set_panic_hook();
    web_sys::console::log_1(
        &"Conway's Game of Life initialized! ライフゲームが初期化されました！".into(),
    );
}
