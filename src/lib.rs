use rand::random;
use wasm_bindgen::prelude::*;


// Note: this is not the most efficient rendering approach of
// the results, but simple enough to understand.
// A better alternative would be to use a buffer for the points and then
// render the buffer to the canvas in JavaScript.

#[wasm_bindgen]
pub fn monte_carlo_pi(n_samples: u32) -> f64 {
    let context = get_canvas_context();    
    let mut n_inside = 0;
    for _ in 0..n_samples {
        // Generate a random point in the unit square
        let x = 2.0 * random::<f64>() - 1.0;
        let y = 2.0 * random::<f64>() - 1.0;
        if x * x + y * y < 1.0 {
            n_inside += 1;
            context.set_fill_style(&JsValue::from_str("red"));
        } else {
            context.set_fill_style(&JsValue::from_str("blue"));
        }
        // Draw a "point"
        context.fill_rect(x, y, 0.01, 0.01);
    }
    
    4.0 * (n_inside as f64) / (n_samples as f64)
}

fn get_canvas_context() -> web_sys::CanvasRenderingContext2d {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    // Clear and change the coordinate system to a usual Cartesian plane
    context.reset_transform().unwrap();

    let sx = canvas.width() as f64;
    let sy = canvas.height() as f64;

    context.clear_rect(0.0, 0.0, sx, sy);

    context.translate(sx / 2.0, sy / 2.0).unwrap();
    context.scale(sx / 2.0, -sy / 2.0 ).unwrap();

    context
}
