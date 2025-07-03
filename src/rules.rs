pub mod standard_rule;

#[cfg(test)]
mod tests;

use crate::field::Field;

/// Trait for implementing different Game of Life rules
/// ライフゲームの異なるルールを実装するためのトレイト
pub trait Rule {
    /// Apply the rule to the current field and return the next generation
    /// 現在のフィールドにルールを適用し、次世代を返す
    fn apply(&self, field: &Field) -> Field;
}
