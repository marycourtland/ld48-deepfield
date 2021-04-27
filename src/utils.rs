use anyhow::*;
use web_sys::{
    Document,
    HtmlElement,
    Element,
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

pub fn get_document() -> Result<Document> {
    let document: Document = window()
        .context("get_document: couldn't find the HTML window object")?
        .document()
        .context("get_document: couldn't find the HTML document element")?;
    Ok(document)
}


pub fn query_html(selector: &str) -> Result<HtmlElement> {
    let document = get_document()?;
    let element: Element = document.query_selector(selector)
        .unwrap().context(format!("query_html: failed to call document.query_selector"))
        .unwrap();

    // TODO: better error handling
    Ok(element.dyn_into::<HtmlElement>().unwrap())

}

pub fn get_canvas_by_id(canvas_id: String) -> Result<HtmlCanvasElement> {
    let document = get_document()?;

    let canvas = document.get_element_by_id(&canvas_id)
        .context(format!("get_canvas_by_id: couldn't find the HTML canvas element with id {}", canvas_id))?;

    let canvas: HtmlCanvasElement = canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| anyhow!("get_canvas_by_id: the HTML element with id {} isn't a Canvas", canvas_id))?;

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