pub mod standard_rule;

#[cfg(test)]
mod tests;

use crate::field::Field;

/// ライフゲームの異なるルールを実装するためのトレイト
pub trait Rule {
    /// 現在のフィールドにルールを適用し、次世代を返す
    fn apply(&self, field: &Field) -> Field;
}
