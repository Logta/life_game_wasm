use crate::config::Color;

pub trait Rule {
    fn burth(vec: Vec<Color>) -> bool;
    fn stay(vec: Vec<Color>) -> bool;
    fn target_cells(x: usize, y: usize, max_rows: usize, max_columns: usize)
        -> Vec<(usize, usize)>;
}
