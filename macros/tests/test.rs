use macros::parse_jsx;


#[test]
fn test_parse_jsx() {
  parse_jsx!(<div a={24}><span b={45}><a /></span></div>);
}