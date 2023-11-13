mod config;
mod creator;
mod field;
mod player;
mod rules;

extern crate futures;
use config::Color;
use creator::creator_trait::Creator;
use creator::table_creator;
use futures::Future;
use js_sys::Promise;
use player::Player;
use rules::standard_rule;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::console;
// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub async fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let p = Player::init(standard_rule::StandardRule {});
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let div = document.get_element_by_id("life-game").unwrap();

    let base = "
    <table>
        <tbody id='life-game-table'>
            <tr>
                <td>The table body</td>
                <td>with two columns</td>
            </tr>
        </tbody>
    </table>
    ";
    div.set_inner_html(base);
    sleep(5000).await;
    let v = vec![
        vec![Color::Black, Color::White],
        vec![Color::Black, Color::White],
    ];

    let base = table_creator::TableCreator::builder(v);
    console::log_1(&JsValue::from_str(&(base.clone())));
    div.set_inner_html(&base);
    let table = document.create_element("life-game-table").unwrap();

    Ok(())
}

pub fn sleep(ms: i32) -> impl Future {
    let p = Promise::new(&mut |resolve, _| {
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms)
            .unwrap();
    });
    JsFuture::from(p)
}
