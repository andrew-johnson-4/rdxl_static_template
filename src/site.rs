pub fn run() {
    rdxl_static::dot_to_file("/index.html",&crate::index()).unwrap();
    rdxl_static::dot_to_file("/page1.html",&crate::page1()).unwrap();
}
