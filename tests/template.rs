use rdxl::xhtml;
use rdxl_static::dot_template;

#[dot_template]
pub fn default(title: String, description: String, xhtml: String) -> String {
   xhtml!(
     <html>
       <head>
         <title>{{ title }}</title>
         <meta name="description" content={{description}}/>
       </head>
       <body>{{ xhtml }}</body>
     </html>
   )
}

#[dot_template]
pub fn alternate1(title: String, description: String, xhtml: String) -> String {
   xhtml!(
     "<!DOCTYPE html>"
     <html>
       <head>
         <title>{{ title }}</title>
         <meta name="description" content={{description}}/>
       </head>
       <body>{{ xhtml }}</body>
     </html>
   )
}
