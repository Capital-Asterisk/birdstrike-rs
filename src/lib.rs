mod utils;
pub mod links;

use std::f64;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct AppStuff {
	dummy: u32,
	java: String
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn rock(inner: u32) -> AppStuff {
    return AppStuff{dummy: inner, java: "hey~".to_string()};
}

#[wasm_bindgen]
pub fn gwah(a: &mut AppStuff) {
    alert("AAAAAAAAA!");
    use web_sys::console;
	console::log_1(&JsValue::from_str(&a.java));
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, birdstrikers!");

}

#[wasm_bindgen(start)]
fn start() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();


    println!("draw!");

    let fish = JsValue::from_str("rgb(255,255,255)");
    
    
    //web_sys::window().unwrap().request_animation_frame();
    
/*
    const renderLoop = () => {
        ctx.set_stroke_style(&fish);
        ctx.begin_path();

        // Draw the outer circle.
        ctx
            .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
            .unwrap();

        // Draw the mouth.
        ctx.move_to(110.0, 75.0);
        ctx.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

        // Draw the left eye.
        ctx.move_to(65.0, 65.0);
        ctx
            .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
            .unwrap();

        // Draw the right eye.
        ctx.move_to(95.0, 65.0);
        ctx
            .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
            .unwrap();

        ctx.rect(20.0, 20.0, 150.0, 100.0);

        ctx.stroke();

        requestAnimationFrame(renderLoop);
    };

    requestAnimationFrame(renderLoop);
*/



}
