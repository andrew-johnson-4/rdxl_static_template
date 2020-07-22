use rdxl_static::*;

pub mod site;
pub mod template;

#[dot]
fn index() -> String {
   dot_html!(
     <p>This will become index.html</p>
   )
}

#[dot]
fn page1() -> String {
   dot_html!(
      template=alternate1,
      title="ABC".to_string(),
      <p>This will become page1.html</p>
   )
}
