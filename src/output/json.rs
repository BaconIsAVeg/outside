use crate::context::Context;
use crate::output::Output;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonOutput {
    pub template: String,
}

impl Output for JsonOutput {
    fn new(context: Context) -> Self {
        let template = serde_json::to_string(&context)
            .unwrap_or_else(|_| "{{\"error\": \"Unable to serialize Context\"}}".to_string());
        JsonOutput { template }
    }

    fn render(&self) -> String {
        self.template.clone()
    }
}
