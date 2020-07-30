use rdxl_static::*;

pub mod template;

#[test]
pub fn index() {
   let (fp,f) = dot_include!("pages/index.html");
   assert_eq!(fp, "www/index.html");
   assert!(f().starts_with("<html>"));
}

#[test]
pub fn page1() {
   let (fp,f) = dot_include!("pages/page1.html");
   assert_eq!(fp, "www/page1.html");
   assert!(f().starts_with("<!DOCTYPE html>"));
}
