use anyhow::*;
use web_sys::{
    CanvasRenderingContext2d,
    HtmlCanvasElement,
    console,
    window,
};

use wasm_bindgen::{
    JsCast
};

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn get_canvas_by_id(canvas_id: String) -> Result<HtmlCanvasElement> {
    let document = window()
        .context("Draw::from_canvas_id: couldn't find the HTML window object")?
        .document()
        .context("Draw::from_canvas_id: couldn't find the HTML document element")?;

    let canvas = document.get_element_by_id(&canvas_id)
        .context(format!("Draw::from_canvas_id: couldn't find the HTML canvas element with id {}", canvas_id))?;

    let canvas: HtmlCanvasElement = canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| anyhow!("Draw::from_canvas_id: the HTML element with id {} isn't a Canvas", canvas_id))?;
        
    Ok(canvas)
}


// A macro to provide `println!(..)`-style syntax for `console.log` logging.
#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        unsafe {
            console::log_1(&format!( $( $t )* ).into());
        }
    }
}