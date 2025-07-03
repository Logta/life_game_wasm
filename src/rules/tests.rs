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