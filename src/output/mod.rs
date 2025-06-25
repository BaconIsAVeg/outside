pub mod detailed;
pub mod json;
pub mod simple;
pub mod waybar;

use crate::context::Context;
use crate::Settings;

use std::fmt::Write;
use tinytemplate::TinyTemplate;

pub trait Output {
    fn new(context: Context, settings: Settings) -> Self;
    fn render(&self) -> String;
    fn tt() -> TinyTemplate<'static> {
        let mut tt = TinyTemplate::new();
        tt.add_formatter("round", |value, output| {
            write!(output, "{:.0}", value.as_f64().unwrap_or(0.0).round())?;
            Ok(())
        });
        tt
    }
}

// TODO: Figure out how this works
pub fn render_output<O: Output>(context: Context, settings: Settings) -> String {
    let output = O::new(context, settings);
    output.render()
}
