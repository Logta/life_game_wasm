mod constants;
mod error;
mod field;
mod renderer;
mod rules;

use field::Field;
use renderer::Renderer;
use rules::{Rule, standard_rule::StandardRule};
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
        self.field.randomize_with(js_sys::Math::random);
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

#[cfg(test)]
mod tests {
    use super::*;

    fn alive_cells(game: &GameOfLife) -> Vec<(usize, usize)> {
        let field = &game.field;
        (0..field.height())
            .flat_map(|r| (0..field.width()).map(move |c| (r, c)))
            .filter(|&(r, c)| field.get_cell(r, c).unwrap())
            .collect()
    }

    #[test]
    fn test_new_game_dimensions_and_generation() {
        // 幅と高さを変えて取り違えを検出する
        let game = GameOfLife::new(7, 5);
        assert_eq!(game.width(), 7);
        assert_eq!(game.height(), 5);
        assert_eq!(game.field.width(), 7);
        assert_eq!(game.field.height(), 5);
        assert_eq!(game.generation(), 0);
        assert!(alive_cells(&game).is_empty());
    }

    #[test]
    fn test_tick_advances_generation_and_field() {
        let mut game = GameOfLife::new(5, 5);
        game.toggle_cell(2, 1);
        game.toggle_cell(2, 2);
        game.toggle_cell(2, 3);

        game.tick();
        assert_eq!(game.generation(), 1);
        assert_eq!(alive_cells(&game), vec![(1, 2), (2, 2), (3, 2)]);

        game.tick();
        game.tick();
        assert_eq!(game.generation(), 3);
        assert_eq!(alive_cells(&game), vec![(1, 2), (2, 2), (3, 2)]);
    }

    #[test]
    fn test_toggle_cell_flips_state_and_ignores_out_of_range() {
        let mut game = GameOfLife::new(4, 3);
        game.toggle_cell(2, 3);
        assert_eq!(alive_cells(&game), vec![(2, 3)]);

        // 範囲外は無視される（パニックしない・他のセルを変えない）
        game.toggle_cell(3, 0);
        game.toggle_cell(0, 4);
        assert_eq!(alive_cells(&game), vec![(2, 3)]);

        game.toggle_cell(2, 3);
        assert!(alive_cells(&game).is_empty());
    }

    #[test]
    fn test_clear_resets_cells_and_generation() {
        let mut game = GameOfLife::new(4, 4);
        game.toggle_cell(1, 1);
        game.toggle_cell(1, 2);
        game.toggle_cell(2, 1);
        game.toggle_cell(2, 2);
        game.tick();
        game.tick();
        assert_eq!(game.generation(), 2);
        assert_eq!(alive_cells(&game).len(), 4);

        game.clear();
        assert_eq!(game.generation(), 0);
        assert!(alive_cells(&game).is_empty());
    }
}
