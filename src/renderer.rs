use web_sys::CanvasRenderingContext2d;

use crate::constants::{ALIVE_COLOR, CELL_SIZE, DEAD_COLOR, GRID_COLOR};
use crate::field::Field;

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
    /// デフォルト設定で新しいレンダラーを作成
    pub fn new() -> Self {
        Self::default()
    }

    /// ゲームフィールド全体を描画
    pub fn render(&self, ctx: &CanvasRenderingContext2d, field: &Field) {
        self.draw_grid(ctx, field.width(), field.height());
        self.draw_cells(ctx, field);
    }

    /// グリッド線を描画
    fn draw_grid(&self, ctx: &CanvasRenderingContext2d, width: usize, height: usize) {
        ctx.begin_path();
        ctx.set_stroke_style_str(&self.grid_color);

        // 垂直線
        for i in 0..=width {
            let x = (i as u32 * self.cell_size) as f64;
            ctx.move_to(x, 0.0);
            ctx.line_to(x, (height as u32 * self.cell_size) as f64);
        }

        // 水平線
        for j in 0..=height {
            let y = (j as u32 * self.cell_size) as f64;
            ctx.move_to(0.0, y);
            ctx.line_to((width as u32 * self.cell_size) as f64, y);
        }

        ctx.stroke();
    }

    /// すべてのセルを描画
    fn draw_cells(&self, ctx: &CanvasRenderingContext2d, field: &Field) {
        let width = field.width();
        let height = field.height();
        let cells = field.cells();

        // パフォーマンス向上のため描画操作をバッチ処理
        ctx.begin_path();

        for row in 0..height {
            for col in 0..width {
                let idx = row * width + col;
                let is_alive = cells[idx];

                ctx.set_fill_style_str(if is_alive {
                    &self.alive_color
                } else {
                    &self.dead_color
                });

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
}
