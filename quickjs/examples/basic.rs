use anyhow::Result;
use quickjs::QuickJS;


fn main() -> Result<()> {
  let quickjs = QuickJS::default();

  let script = include_str!("../js/basic.js");

  let data = include_str!("../data/basic.json");

  let res = quickjs.try_execute(script, Some(data), true, true)?;

  println!("{}", res.unwrap_or_else(|| "None".to_string()));

  Ok(())
}
