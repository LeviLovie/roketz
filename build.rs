fn main() {
    compile_assets();
    println!("cargo:rerun-if-changed=assets/*");
    println!("cargo:rerun-if-changed=assets.yaml");

    copy_scripts();
    println!("cargo:rerun-if-changed=scripts/*");
}

fn compile_assets() {
    let compiled_assets = assets::declare::compile_assets().expect("Failed to compile assets");
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let target_dir = std::path::Path::new(&out_dir).ancestors().nth(3).unwrap();
    if !target_dir.exists() {
        std::fs::create_dir_all(&out_dir).expect("Failed to create OUT_DIR");
    }
    let out_path = std::path::Path::new(&target_dir).join("assets.bin");
    std::fs::write(out_path, compiled_assets).expect("Failed to write assets to file");
}

fn copy_scripts() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let target_dir = std::path::Path::new(&out_dir).ancestors().nth(3).unwrap();
    let scripts_dir = target_dir.join("scripts");
    if !scripts_dir.exists() {
        std::fs::create_dir_all(&scripts_dir).expect("Failed to create scripts directory");
    }
    for entry in std::fs::read_dir("scripts").expect("Failed to read scripts directory") {
        let entry = entry.expect("Failed to read entry");
        let src_path = entry.path();
        if src_path.is_file() {
            let dest_path = scripts_dir.join(src_path.file_name().unwrap());
            std::fs::copy(src_path, dest_path).expect("Failed to copy script file");
        }
    }
}
