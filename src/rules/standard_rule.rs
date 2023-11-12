use crate::config;
use config::Color;

use super::rule_trait;

pub struct StandardRule {}

impl rule_trait::Rule for StandardRule {
    fn target_cells(
        x: usize,
        y: usize,
        max_rows: usize,
        max_columns: usize,
    ) -> Vec<(usize, usize)> {
        let xs = match x {
            0 => vec![x, x + 1],
            n if n == max_columns => vec![x - 1, x],
            _ => vec![x - 1, x, x + 1],
        };
        let ys = match y {
            0 => vec![y, y + 1],
            n if n == max_rows => vec![y - 1, y],
            _ => vec![y - 1, y, y + 1],
        };
        let mut v: Vec<(usize, usize)> = vec![];
        for xx in xs {
            for yy in ys.clone() {
                v.push((xx, yy))
            }
        }
        v
    }

    fn burth(vec: Vec<Color>) -> bool {
        let point = color_point_sum(vec);
        point == 3
    }

    fn stay(vec: Vec<Color>) -> bool {
        let point = color_point_sum(vec);
        point == 2 || point == 3
    }
}

fn color_point_sum(vec: Vec<Color>) -> u64 {
    vec.iter()
        .map(|c| if c == &Color::Black { 1 } else { 0 })
        .sum()
}

#[cfg(test)]
mod test {
    use crate::rules::rule_trait::Rule;
    use crate::rules::standard_rule::StandardRule;

    #[test]
    fn test_target_cells() {
        // Test
        assert_eq!(StandardRule::target_cells(0, 0, 100, 100).len(), 4);
        assert_eq!(StandardRule::target_cells(0, 1, 100, 100).len(), 6);
        assert_eq!(StandardRule::target_cells(1, 0, 100, 100).len(), 6);
        assert_eq!(StandardRule::target_cells(1, 1, 100, 100).len(), 9);

        // Test
        assert_eq!(StandardRule::target_cells(100, 100, 100, 100).len(), 4);
        assert_eq!(StandardRule::target_cells(100, 99, 100, 100).len(), 6);
        assert_eq!(StandardRule::target_cells(99, 100, 100, 100).len(), 6);
        assert_eq!(StandardRule::target_cells(99, 99, 100, 100).len(), 9);
    }
}
