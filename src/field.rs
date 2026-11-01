use crate::error::{GameError, Result};

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    cells: Vec<bool>,
    width: usize,
    height: usize,
}

impl Field {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            cells: vec![false; width * height],
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn cells(&self) -> &[bool] {
        &self.cells
    }

    pub fn get_index(&self, row: usize, col: usize) -> Result<usize> {
        if row >= self.height || col >= self.width {
            return Err(GameError::new(&format!(
                "セルの位置 ({}, {}) はサイズ {}x{} のフィールド範囲外です",
                row, col, self.height, self.width
            )));
        }
        Ok(row * self.width + col)
    }

    fn get_index_unchecked(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }

    #[allow(dead_code)]
    pub fn get_cell(&self, row: usize, col: usize) -> Result<bool> {
        let idx = self.get_index(row, col)?;
        Ok(self.cells[idx])
    }

    pub fn get_cell_unchecked(&self, row: usize, col: usize) -> bool {
        let idx = self.get_index_unchecked(row, col);
        self.cells[idx]
    }

    #[allow(dead_code)]
    pub fn set_cell(&mut self, row: usize, col: usize, alive: bool) -> Result<()> {
        let idx = self.get_index(row, col)?;
        self.cells[idx] = alive;
        Ok(())
    }

    pub fn set_cell_unchecked(&mut self, row: usize, col: usize, alive: bool) {
        let idx = self.get_index_unchecked(row, col);
        self.cells[idx] = alive;
    }

    pub fn toggle_cell(&mut self, row: usize, col: usize) -> Result<()> {
        let idx = self.get_index(row, col)?;
        self.cells[idx] = !self.cells[idx];
        Ok(())
    }

    pub fn clear(&mut self) {
        self.cells.fill(false);
    }

    /// 乱数源 `rng`（0.0 以上 1.0 未満の値を返す関数）を使ってフィールドをランダム化する
    ///
    /// 乱数源を引数で受け取ることで、ブラウザ API（`Math.random`）に依存せずにテストできる。
    /// `rng()` が 0.5 より大きいセルを生きたセルにする。
    pub fn randomize_with<F: FnMut() -> f64>(&mut self, mut rng: F) {
        for cell in &mut self.cells {
            *cell = rng() > 0.5;
        }
    }

    /// 8方向の生きた隣接セルの数を数える
    ///
    /// フィールドはトーラス状（上下左右がループする）に扱われる。
    /// `height - 1` / `width - 1` を加えて `% height` / `% width` することで、
    /// 端のセルの隣接セルは反対側の端としてラップアラウンドする。
    pub fn count_live_neighbors(&self, row: usize, col: usize) -> u8 {
        let mut count = 0;

        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (col + delta_col) % self.width;

                if self.get_cell_unchecked(neighbor_row, neighbor_col) {
                    count += 1;
                }
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_field_creation() {
        let field = Field::new(10, 10);
        assert_eq!(field.width(), 10);
        assert_eq!(field.height(), 10);
        assert_eq!(field.cells().len(), 100);

        // 初期状態ではすべてのセルが死んでいるべき
        for cell in field.cells() {
            assert!(!cell);
        }
    }

    #[test]
    fn test_cell_operations() {
        let mut field = Field::new(3, 3);

        // 初期状態をテスト
        assert!(!field.get_cell(1, 1).unwrap());

        // set_cellをテスト
        field.set_cell(1, 1, true).unwrap();
        assert!(field.get_cell(1, 1).unwrap());

        // toggle_cellをテスト
        field.toggle_cell(1, 1).unwrap();
        assert!(!field.get_cell(1, 1).unwrap());

        // 範囲外テスト
        assert!(field.get_cell(10, 10).is_err());
        assert!(field.set_cell(10, 10, true).is_err());
        assert!(field.toggle_cell(10, 10).is_err());
    }

    #[test]
    fn test_clear() {
        let mut field = Field::new(3, 3);

        // いくつかのセルを生きた状態に設定
        field.set_cell(0, 0, true).unwrap();
        field.set_cell(1, 1, true).unwrap();
        field.set_cell(2, 2, true).unwrap();

        // フィールドをクリア
        field.clear();

        // すべてのセルが死んでいるべき
        for row in 0..3 {
            for col in 0..3 {
                assert!(!field.get_cell(row, col).unwrap());
            }
        }
    }

    #[test]
    fn test_count_live_neighbors() {
        let mut field = Field::new(3, 3);

        // パターンを作成:
        // X O X
        // O X O
        // X O X
        field.set_cell(0, 1, true).unwrap();
        field.set_cell(1, 0, true).unwrap();
        field.set_cell(1, 2, true).unwrap();
        field.set_cell(2, 1, true).unwrap();

        // 中央のセルは4つの生きた隣接セルを持つべき
        assert_eq!(field.count_live_neighbors(1, 1), 4);

        // 角のセルはラップアラウンド動作を見るべき
        // (0,0) の隣接セル: (2,2),(2,0),(2,1),(0,2),(0,1),(1,2),(1,0),(1,1)
        // のうち生きているのは (2,1),(0,1),(1,2),(1,0) の4つ
        assert_eq!(field.count_live_neighbors(0, 0), 4);
    }

    #[test]
    fn test_get_index_row_major() {
        let field = Field::new(4, 3);
        assert_eq!(field.get_index(0, 0).unwrap(), 0);
        assert_eq!(field.get_index(0, 3).unwrap(), 3);
        assert_eq!(field.get_index(1, 0).unwrap(), 4);
        assert_eq!(field.get_index(2, 3).unwrap(), 11);
    }

    #[test]
    fn test_get_index_out_of_range_on_single_axis() {
        // 幅4・高さ3: 行だけ・列だけが範囲外の場合もエラーになる
        let field = Field::new(4, 3);
        assert!(field.get_index(3, 0).is_err());
        assert!(field.get_index(0, 4).is_err());
        assert!(field.get_index(2, 3).is_ok());
    }

    #[test]
    fn test_set_and_toggle_affect_only_target_cell() {
        let mut field = Field::new(4, 3);
        field.set_cell(1, 2, true).unwrap();
        field.toggle_cell(2, 3).unwrap();

        let alive: Vec<usize> = field
            .cells()
            .iter()
            .enumerate()
            .filter(|(_, alive)| **alive)
            .map(|(idx, _)| idx)
            .collect();
        assert_eq!(alive, vec![6, 11]);

        // toggle は死んだセルを生き返らせ、生きたセルを殺す
        field.toggle_cell(1, 2).unwrap();
        assert!(!field.get_cell(1, 2).unwrap());
        assert!(field.get_cell(2, 3).unwrap());
    }

    #[test]
    fn test_count_live_neighbors_all_eight_directions() {
        // 5x5 の中央 (2,2) の周囲8マスをすべて生かす
        let mut field = Field::new(5, 5);
        for row in 1..=3 {
            for col in 1..=3 {
                if (row, col) != (2, 2) {
                    field.set_cell(row, col, true).unwrap();
                }
            }
        }
        assert_eq!(field.count_live_neighbors(2, 2), 8);

        // 自分自身は数えない
        field.set_cell(2, 2, true).unwrap();
        assert_eq!(field.count_live_neighbors(2, 2), 8);

        // (0,0) から見て隣接しているのは (1,1) だけ（他は離れている）
        assert_eq!(field.count_live_neighbors(0, 0), 1);
    }

    #[test]
    fn test_count_live_neighbors_single_cell_each_direction() {
        // 各方向に生きたセルが1つだけある場合に1と数える
        // (幅と高さを変えて、行と列の取り違えも検出する)
        for (dr, dc) in [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ] {
            let mut field = Field::new(7, 5);
            let row = (2 + dr) as usize;
            let col = (3 + dc) as usize;
            field.set_cell(row, col, true).unwrap();
            assert_eq!(
                field.count_live_neighbors(2, 3),
                1,
                "direction ({dr}, {dc}) should count as a neighbor"
            );
        }
    }

    #[test]
    fn test_count_live_neighbors_wraps_around_edges() {
        // 幅6・高さ4のトーラス: 右下の角 (3,5) の隣接セルは左端・上端にラップする
        let mut field = Field::new(6, 4);
        field.set_cell(0, 0, true).unwrap(); // 右下 + 1,+1 方向
        field.set_cell(0, 5, true).unwrap(); // 下方向
        field.set_cell(3, 0, true).unwrap(); // 右方向
        assert_eq!(field.count_live_neighbors(3, 5), 3);

        // 左上の角 (0,0) からは右下側が隣接セルになる
        let mut field = Field::new(6, 4);
        field.set_cell(3, 5, true).unwrap();
        field.set_cell(3, 0, true).unwrap();
        field.set_cell(0, 5, true).unwrap();
        assert_eq!(field.count_live_neighbors(0, 0), 3);

        // ラップしない遠いセルは数えない
        field.set_cell(2, 3, true).unwrap();
        assert_eq!(field.count_live_neighbors(0, 0), 3);
    }

    #[test]
    fn test_randomize_with_threshold() {
        let mut field = Field::new(2, 2);
        // 0.5 より大きい値のときだけ生きたセルになる（0.5 ちょうどは死んだセル）
        let values = [0.9, 0.5, 0.1, 0.51];
        let mut iter = values.iter().copied();
        field.randomize_with(|| iter.next().unwrap());
        assert_eq!(field.cells(), &[true, false, false, true]);

        // 既存の状態は上書きされる
        field.randomize_with(|| 0.0);
        assert!(field.cells().iter().all(|alive| !alive));
        field.randomize_with(|| 0.99);
        assert!(field.cells().iter().all(|alive| *alive));
    }
}
