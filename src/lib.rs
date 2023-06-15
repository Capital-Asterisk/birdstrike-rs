mod utils;
pub mod links;

pub use links::*;
use links::dsl::*;

use std::f64;
use wasm_bindgen::prelude::*;
use web_sys::console;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct AppStuff {
	graph: Option<SpaghettiLinkGraph>,
	error: String
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
    
    fn drawSpaghettiHighlight(tlx: f64, tly: f64, brx: f64, bry: f64);
    fn drawSpaghettiText(x: f64, y: f64, w: f64, h: f64, press: bool);
    fn drawSpaghettiButton(tlx: f64, tly: f64, brx: f64, bry: f64);
}

#[wasm_bindgen]
pub fn from_dsl(source: &str) -> AppStuff {
    match make_spaghetti(source) {
        Ok(graph) => AppStuff{ graph: Some(graph), error: String::new() },
        Err((offset, msg)) => AppStuff{ graph: None, error: make_err_msg(source, offset, &msg) },
    }
}

#[wasm_bindgen]
pub fn read_err(a: &AppStuff) -> String {
    a.error.clone()
}

#[wasm_bindgen]
pub fn gwah(a: &mut AppStuff) {
    alert("AAAAAAAAA!");
    //use web_sys::console;
    //console::log_1(&JsValue::from_str(&a.java));
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, birdstrikers!");

}

#[wasm_bindgen]
pub fn draw(a: &AppStuff) {

    // draw all buttons lol
    {
        let Some(graph) = &a.graph else {
            return;
        };
        
        for &elem_any in graph.elements.elem_type_data[3].elem_local_to_any.iter() {
            let Some(ports) = graph.values_point.connections.elem_to_port_to_link.get(elem_any) else {
                continue;
            };
            let Some(&link_tl) = ports.get(0) else {
                continue;
            };
            let Some(&link_br) = ports.get(1) else {
                continue;
            };
            
            let point_tl: &Point = &graph.values_point.values.link_values[link_tl];
            let point_br: &Point = &graph.values_point.values.link_values[link_br];
            
            drawSpaghettiButton(point_tl.x, point_tl.y, point_br.x, point_br.y);
        }
    }


    // draw all highlights lol
    {
        let Some(graph) = &a.graph else {
            return;
        };
        
        for &elem_any in graph.elements.elem_type_data[0].elem_local_to_any.iter() {
            let Some(ports) = graph.values_point.connections.elem_to_port_to_link.get(elem_any) else {
                continue;
            };
            let Some(&link_tl) = ports.get(0) else {
                continue;
            };
            let Some(&link_br) = ports.get(1) else {
                continue;
            };
            
            let point_tl: &Point = &graph.values_point.values.link_values[link_tl];
            let point_br: &Point = &graph.values_point.values.link_values[link_br];
            
            drawSpaghettiHighlight(point_tl.x, point_tl.y, point_br.x, point_br.y);
        }
    }
    
    
    
    
    
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
    
    



}
