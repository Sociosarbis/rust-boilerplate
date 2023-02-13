use anyhow::Result;
use once_cell::sync::OnceCell;
use quickjs_wasm_rs::Context;

use crate::io::{get_input_string, set_output_value, get_input_value};

#[cfg(feature = "console")]
mod context;
mod io;

static mut JS_CONTEXT: OnceCell<Context> = OnceCell::new();
static SCRIPT_NAME: &str = "script.js";

#[export_name = "wizer.initialize"]
pub extern "C" fn init() {
    unsafe {
        let context = Context::default();

        #[cfg(feature = "console")]
        context::set_quickjs_globals(&context).unwrap();

        JS_CONTEXT.set(context).unwrap();
    }
}

fn main() -> Result<()> {
    match get_input_string()? {
        Some(input) => {
            let context = unsafe {
                JS_CONTEXT.get_or_init(Context::default) 
            };
            if let Some(value) = get_input_value(context)? {
                context.global_object()?.set_property("data", value)?;
            }
            let output = context.eval_global(SCRIPT_NAME, &input)?;
            set_output_value(Some(output))
        },
        None => set_output_value(None),
    }
}
