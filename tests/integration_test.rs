// Import from the compiled library
use life_game_wasm::GameOfLife;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_game_creation() {
    let game = GameOfLife::new(10, 10);
    assert_eq!(game.width(), 10);
    assert_eq!(game.height(), 10);
    assert_eq!(game.generation(), 0);
}

#[wasm_bindgen_test]
fn test_game_tick() {
    let mut game = GameOfLife::new(10, 10);
    let initial_generation = game.generation();
    
    game.tick();
    assert_eq!(game.generation(), initial_generation + 1);
    
    game.tick();
    assert_eq!(game.generation(), initial_generation + 2);
}

#[wasm_bindgen_test]
fn test_game_clear() {
    let mut game = GameOfLife::new(10, 10);
    
    // Advance the game a few generations
    for _ in 0..5 {
        game.tick();
    }
    assert_eq!(game.generation(), 5);
    
    // Clear should reset generation to 0
    game.clear();
    assert_eq!(game.generation(), 0);
}

#[wasm_bindgen_test]
fn test_toggle_cell() {
    let mut game = GameOfLife::new(10, 10);
    
    // Toggle a cell (we can't directly test the state, but we can ensure it doesn't panic)
    game.toggle_cell(5, 5);
    game.toggle_cell(0, 0);
    game.toggle_cell(9, 9);
    
    // Toggle out of bounds should not panic
    game.toggle_cell(100, 100);
}

#[wasm_bindgen_test]
fn test_randomize() {
    let mut game = GameOfLife::new(10, 10);
    
    // Advance the game
    for _ in 0..3 {
        game.tick();
    }
    assert_eq!(game.generation(), 3);
    
    // Randomize should reset generation
    game.randomize();
    assert_eq!(game.generation(), 0);
}