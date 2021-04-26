use anyhow::*;
use web_sys::{
    HtmlCanvasElement,
    console,
    window
};

use wasm_bindgen::{
    JsCast,
    JsValue
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

pub unsafe fn log1(s: &JsValue) {
    console::log_1(s);
}

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        unsafe {
            utils::log1(&format!( $( $t )* ).into());
        }
    }
}