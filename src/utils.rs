use anyhow::*;
use web_sys::{
    HtmlCanvasElement,
    window
};

use wasm_bindgen::{
    JsCast
};

pub fn set_panic_hook() {
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
        // Note: web_sys::console::log_1 is unsafe, but the macro doesn't recognize it
        #[allow(unused_unsafe)]
        unsafe {
            web_sys::console::log_1(&format!( $( $t )* ).into());
        }
    }
}