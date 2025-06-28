use anyhow::{Context, Result};

fn main() -> Result<()> {
    if let Err(err) = try_main() {
        eprintln!("Error: {:?}", err);
        std::process::exit(1);
    }
    Ok(())
}

fn try_main() -> Result<()> {
    compile_assets().context("Failed to compile assets")?;
    watch_dir_recursive("../../assets").context("Failed to watch assets directory recursively")?;
    Ok(())
}

fn compile_assets() -> Result<()> {
    let compiled_assets = assets::declare::compile_assets().context("Failed to compile assets")?;
    let out_dir = std::env::var("OUT_DIR").context("OUT_DIR environment variable not set")?;
    let target_dir = std::path::Path::new(&out_dir)
        .ancestors()
        .nth(3)
        .context("Failed to get target directory")?;
    if !target_dir.exists() {
        std::fs::create_dir_all(&out_dir).context("Failed to create target directory")?;
    }
    let out_path = std::path::Path::new(&target_dir).join("assets.bin");
    std::fs::write(out_path, compiled_assets).context("Failed to write compiled assets")?;
    Ok(())
}

fn watch_dir_recursive<P: Into<std::path::PathBuf>>(path: P) -> Result<()> {
    let path: std::path::PathBuf = path.into();
    let dir = std::fs::read_dir(&path).context(format!(
        "Failed to read directory: {}",
        path.canonicalize()?.display()
    ))?;
    for entry in dir {
        let entry = entry.context("Failed to read directory entry")?;
        let path = entry.path();
        if path.is_dir() {
            watch_dir_recursive(&path).context("Failed to watch directory recursively")?;
        } else {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }
    Ok(())
}
