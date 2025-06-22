pub mod detailed;
pub mod json;
pub mod simple;
pub mod waybar;

use crate::context::Context;

// TODO: Add TinyTemplate support for rendering templates
pub trait Output {
    fn new(context: Context) -> Self;
    fn render(&self) -> String;
}

pub fn render_output<O: Output>(context: Context) -> String {
    let output = O::new(context);
    output.render()
}
