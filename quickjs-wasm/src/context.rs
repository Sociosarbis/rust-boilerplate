use anyhow::Result;
use std::io::{Write, stdout, stderr};
use quickjs_wasm_rs::{Context, Value};

pub fn set_quickjs_globals(context: &Context) -> Result<()> {
  let global = context.global_object()?;
  let console_log_callback = context.wrap_callback(console_log_to(stdout()))?;
  let console_error_callback = context.wrap_callback(console_log_to(stderr()))?;
  let console_object = context.object_value()?;
  console_object.set_property("log", console_log_callback)?;
  console_object.set_property("error", console_error_callback)?;
  global.set_property("console", console_object)?;
  Ok(())
}

fn console_log_to<T>(mut stream: T) -> impl FnMut(&Context, &Value, &[Value]) -> Result<Value>
where
  T: Write + 'static
{
  move |ctx: &Context, _this: &Value, args: &[Value]| {
    for (i, arg) in args.iter().enumerate() {
      if i != 0 {
        write!(stream, " ")?
      }
      stream.write_all(arg.as_str()?.as_bytes())?
    }
    writeln!(stream)?;
    ctx.undefined_value()
  }
}