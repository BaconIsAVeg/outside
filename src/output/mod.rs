pub mod detailed;
pub mod json;
pub mod simple;
pub mod waybar;

use crate::context::Context;
use crate::Settings;

pub trait Output {
    fn new(context: Context, settings: Settings) -> Self;
    fn render(&self) -> String;
}

// TODO: Figure out how this works
pub fn render_output<O: Output>(context: Context, settings: Settings) -> String {
    let output = O::new(context, settings);
    output.render()
}
