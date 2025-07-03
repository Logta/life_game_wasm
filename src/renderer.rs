use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

use crate::constants::{ALIVE_COLOR, CELL_SIZE, DEAD_COLOR, GRID_COLOR};
use crate::field::Field;

/// Canvas rendering functionality for the Game of Life
/// ライフゲームのCanvas描画機能
pub struct Renderer {
    cell_size: u32,
    grid_color: String,
    dead_color: String,
    alive_color: String,
}

impl Default for Renderer {
    fn default() -> Self {
        Self {
            cell_size: CELL_SIZE,
            grid_color: GRID_COLOR.to_string(),
            dead_color: DEAD_COLOR.to_string(),
            alive_color: ALIVE_COLOR.to_string(),
        }
    }
}

impl Renderer {
    /// Create a new renderer with default settings
    /// デフォルト設定で新しいレンダラーを作成
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a renderer with custom colors
    /// カスタムカラーでレンダラーを作成
    pub fn with_colors(grid: &str, dead: &str, alive: &str) -> Self {
        Self {
            grid_color: grid.to_string(),
            dead_color: dead.to_string(),
            alive_color: alive.to_string(),
            ..Self::default()
        }
    }

    /// Render the entire game field
    /// ゲームフィールド全体を描画
    pub fn render(&self, ctx: &CanvasRenderingContext2d, field: &Field) {
        self.draw_grid(ctx, field.width(), field.height());
        self.draw_cells(ctx, field);
    }

    /// Draw the grid lines
    /// グリッド線を描画
    fn draw_grid(&self, ctx: &CanvasRenderingContext2d, width: usize, height: usize) {
        ctx.begin_path();
        ctx.set_stroke_style(&JsValue::from_str(&self.grid_color));

        // Vertical lines / 垂直線
        for i in 0..=width {
            let x = (i as u32 * self.cell_size) as f64;
            ctx.move_to(x, 0.0);
            ctx.line_to(x, (height as u32 * self.cell_size) as f64);
        }

        // Horizontal lines / 水平線
        for j in 0..=height {
            let y = (j as u32 * self.cell_size) as f64;
            ctx.move_to(0.0, y);
            ctx.line_to((width as u32 * self.cell_size) as f64, y);
        }

        ctx.stroke();
    }

    /// Draw all cells
    /// すべてのセルを描画
    fn draw_cells(&self, ctx: &CanvasRenderingContext2d, field: &Field) {
        let width = field.width();
        let height = field.height();
        let cells = field.cells();

        // Batch draw operations for better performance
        // パフォーマンス向上のため描画操作をバッチ処理
        ctx.begin_path();

        for row in 0..height {
            for col in 0..width {
                let idx = row * width + col;
                let is_alive = cells[idx];

                ctx.set_fill_style(&JsValue::from_str(if is_alive {
                    &self.alive_color
                } else {
                    &self.dead_color
                }));

                ctx.fill_rect(
                    (col as u32 * self.cell_size + 1) as f64,
                    (row as u32 * self.cell_size + 1) as f64,
                    (self.cell_size - 1) as f64,
                    (self.cell_size - 1) as f64,
                );
            }
        }

        ctx.stroke();
    }

    /// Draw a single cell (for optimized updates)
    /// 単一のセルを描画（最適化された更新用）
    pub fn draw_cell(&self, ctx: &CanvasRenderingContext2d, row: usize, col: usize, alive: bool) {
        ctx.set_fill_style(&JsValue::from_str(if alive {
            &self.alive_color
        } else {
            &self.dead_color
        }));

        ctx.fill_rect(
            (col as u32 * self.cell_size + 1) as f64,
            (row as u32 * self.cell_size + 1) as f64,
            (self.cell_size - 1) as f64,
            (self.cell_size - 1) as f64,
        );
    }

    /// Get the cell size
    /// セルサイズを取得
    pub fn cell_size(&self) -> u32 {
        self.cell_size
    }
}