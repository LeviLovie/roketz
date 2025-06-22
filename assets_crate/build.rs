fn main() {
    watch_dir_recursive("../assets");
}

fn watch_dir_recursive<P: AsRef<std::path::Path>>(path: P) {
    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            watch_dir_recursive(&path);
        } else {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }
}
