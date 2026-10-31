use super::*;
use crate::field::Field;

/// Test pattern: Blinker
/// Oscillates between horizontal and vertical
#[test]
fn test_blinker_pattern() {
    let mut field = Field::new(5, 5);
    let rule = standard_rule::StandardRule::new();

    // Create horizontal blinker
    field.set_cell(2, 1, true).unwrap();
    field.set_cell(2, 2, true).unwrap();
    field.set_cell(2, 3, true).unwrap();

    // Apply rule - should become vertical
    let field = rule.apply(&field);
    assert!(field.get_cell(1, 2).unwrap());
    assert!(field.get_cell(2, 2).unwrap());
    assert!(field.get_cell(3, 2).unwrap());
    assert!(!field.get_cell(2, 1).unwrap());
    assert!(!field.get_cell(2, 3).unwrap());

    // Apply rule again - should become horizontal again
    let field = rule.apply(&field);
    assert!(field.get_cell(2, 1).unwrap());
    assert!(field.get_cell(2, 2).unwrap());
    assert!(field.get_cell(2, 3).unwrap());
    assert!(!field.get_cell(1, 2).unwrap());
    assert!(!field.get_cell(3, 2).unwrap());
}

/// Test pattern: Block (still life)
/// Should remain unchanged
#[test]
fn test_block_pattern() {
    let mut field = Field::new(4, 4);
    let rule = standard_rule::StandardRule::new();

    // Create 2x2 block
    field.set_cell(1, 1, true).unwrap();
    field.set_cell(1, 2, true).unwrap();
    field.set_cell(2, 1, true).unwrap();
    field.set_cell(2, 2, true).unwrap();

    // Apply rule - should remain unchanged
    let next_field = rule.apply(&field);
    assert!(next_field.get_cell(1, 1).unwrap());
    assert!(next_field.get_cell(1, 2).unwrap());
    assert!(next_field.get_cell(2, 1).unwrap());
    assert!(next_field.get_cell(2, 2).unwrap());

    // Check that other cells remain dead
    assert!(!next_field.get_cell(0, 0).unwrap());
    assert!(!next_field.get_cell(0, 1).unwrap());
    assert!(!next_field.get_cell(3, 3).unwrap());
}

/// Neighbor offsets around the center cell (2, 2) of a 5x5 field.
const NEIGHBORS: [(usize, usize); 8] = [
    (1, 1),
    (1, 2),
    (1, 3),
    (2, 1),
    (2, 3),
    (3, 1),
    (3, 2),
    (3, 3),
];

/// Build a 5x5 field whose center has exactly `count` live neighbors.
fn field_with_center_neighbors(center_alive: bool, count: usize) -> Field {
    let mut field = Field::new(5, 5);
    field.set_cell(2, 2, center_alive).unwrap();
    for &(row, col) in NEIGHBORS.iter().take(count) {
        field.set_cell(row, col, true).unwrap();
    }
    field
}

/// B3/S23: every (state, neighbor count) combination for the center cell.
#[test]
fn test_birth_and_survival_table() {
    let rule = standard_rule::StandardRule::new();
    for count in 0..=8 {
        let next = rule.apply(&field_with_center_neighbors(true, count));
        assert_eq!(
            next.get_cell(2, 2).unwrap(),
            count == 2 || count == 3,
            "live cell with {count} neighbors"
        );

        let next = rule.apply(&field_with_center_neighbors(false, count));
        assert_eq!(
            next.get_cell(2, 2).unwrap(),
            count == 3,
            "dead cell with {count} neighbors"
        );
    }
}

/// The next generation is computed from the previous one only
/// (cells updated earlier in the same tick must not affect later cells).
#[test]
fn test_apply_does_not_mutate_input_and_keeps_size() {
    let rule = standard_rule::StandardRule::new();
    let mut field = Field::new(6, 4);
    field.set_cell(1, 1, true).unwrap();
    field.set_cell(1, 2, true).unwrap();
    field.set_cell(1, 3, true).unwrap();
    let before = field.clone();

    let next = rule.apply(&field);
    assert_eq!(field, before);
    assert_eq!(next.width(), 6);
    assert_eq!(next.height(), 4);
    let alive: Vec<(usize, usize)> = (0..4)
        .flat_map(|r| (0..6).map(move |c| (r, c)))
        .filter(|&(r, c)| next.get_cell(r, c).unwrap())
        .collect();
    assert_eq!(alive, vec![(0, 2), (1, 2), (2, 2)]);
}

/// A glider returns to its shape shifted by (1, 1) every 4 generations,
/// and wraps around the torus edges.
#[test]
fn test_glider_wraps_around_torus() {
    let rule = standard_rule::StandardRule::new();
    let glider = [(0, 1), (1, 2), (2, 0), (2, 1), (2, 2)];
    let size = 6;
    let mut field = Field::new(size, size);
    for &(r, c) in &glider {
        field.set_cell(r, c, true).unwrap();
    }

    // After 4 * size generations the glider has travelled once around the torus.
    for step in 1..=4 * size {
        field = rule.apply(&field);
        if step % 4 == 0 {
            let shift = step / 4;
            let mut expected = Field::new(size, size);
            for &(r, c) in &glider {
                expected
                    .set_cell((r + shift) % size, (c + shift) % size, true)
                    .unwrap();
            }
            assert_eq!(field, expected, "generation {step}");
        }
    }
}
