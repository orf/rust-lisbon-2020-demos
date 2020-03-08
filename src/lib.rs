use wasm_bindgen::prelude::*;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // window.document.body:
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let p: web_sys::Node = document.create_element("p")?.into();
    p.set_text_content(Some("Hello Lisbon from Rust, WebAssembly, and Webpack!"));

    body.append_child(&p)?;

    Ok(())
}
