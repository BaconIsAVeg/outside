pub mod detailed;
pub mod json;
pub mod simple;
pub mod waybar;

use crate::context::Context;

pub trait Output {
    fn new(context: Context) -> Self;
    fn render(&self) -> String;
}

// TODO: Figure out how this works
pub fn render_output<O: Output>(context: Context) -> String {
    let output = O::new(context);
    output.render()
}
