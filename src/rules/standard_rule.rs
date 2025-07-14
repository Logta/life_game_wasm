use crate::field::Field;
use crate::rules::Rule;

#[derive(Debug, Default, Clone)]
pub struct StandardRule;

impl StandardRule {
    pub fn new() -> Self {
        Self
    }
}

impl Rule for StandardRule {
    fn apply(&self, field: &Field) -> Field {
        let mut next_field = field.clone();

        for row in 0..field.height() {
            for col in 0..field.width() {
                let live_neighbors = field.count_live_neighbors(row, col);
                let is_alive = field.get_cell_unchecked(row, col);

                let next_state = match (is_alive, live_neighbors) {
                    // 2-3の隣接セルを持つ生きたセルは生き残る
                    (true, 2) | (true, 3) => true,
                    // 死んだセルがち3つの隣接セルを持つと生き返る
                    (false, 3) => true,
                    // 他のすべての場合: セルは死んでいるか死んだまま
                    _ => false,
                };

                next_field.set_cell_unchecked(row, col, next_state);
            }
        }

        next_field
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::field::Field;

    #[test]
    fn test_standard_rule() {
        let mut field = Field::new(5, 5);
        let rule = StandardRule::new();

        // ブリンカーパターンを作成
        field.set_cell(2, 1, true);
        field.set_cell(2, 2, true);
        field.set_cell(2, 3, true);

        // ルールを一度適用
        let next_field = rule.apply(&field);

        // パターンが回転したかチェック
        assert!(next_field.get_cell(1, 2).unwrap());
        assert!(next_field.get_cell(2, 2).unwrap());
        assert!(next_field.get_cell(3, 2).unwrap());
        assert!(!next_field.get_cell(2, 1).unwrap());
        assert!(!next_field.get_cell(2, 3).unwrap());
    }
}
