pub fn run() {
    rdxl_static::dot_to_file("/index.html",&crate::index().content).unwrap();
}
