use rdxl_static::*;

pub mod site;

#[dot]
fn index() -> DotHtml {
   dot_html!(
     <p>This will become index.html</p>
   )
}
