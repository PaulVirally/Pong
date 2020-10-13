use std::panic;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;

extern crate console_error_panic_hook;

mod game_object;
mod game_manager;
use game_manager::GameManager;

static VERT_SHADER_SRC: &str = r#"#version 300 es
    layout(location=0) in vec2 attr_position;
    void main() {
        gl_Position = vec4(attr_position, 0.f, 1.f);
    }
"#;

static FRAG_SHADER_SRC: &str = r#"#version 300 es
    precision mediump float;
    out vec4 frag_color;
    void main() {
        frag_color = vec4(1.f, 1.f, 1.f, 1.f);
    }
"#;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("`window` does not have a `document`");
    let body = document.body().expect("`document` does not have a `body`");

    let win_size: (u32, u32) = (window.inner_width()?.as_f64().unwrap() as u32, window.inner_height()?.as_f64().unwrap() as u32);

    let canvas = document.create_element("canvas")?.dyn_into::<web_sys::HtmlCanvasElement>()?;
    canvas.set_width(win_size.0);
    canvas.set_height(win_size.1);
    body.append_child(canvas.as_ref())?;

    let context = canvas.get_context("webgl2")?.expect("Browser does not support webgl2").dyn_into::<WebGl2RenderingContext>()?;

    let mut gm = GameManager::new(context, VERT_SHADER_SRC, FRAG_SHADER_SRC, win_size.0 as f32, win_size.1 as f32)?;
    gm.init_event_handlers(&document)?;
    gm.start_game()?;

    Ok(())
}
