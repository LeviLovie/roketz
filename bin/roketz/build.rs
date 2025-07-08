use anyhow::{Context, Result};
use std::{path::PathBuf, sync::LazyLock};

const ROOT: &str = "../..";

macro_rules! relative_path {
    ($name: ident, $path: expr) => {
        static $name: LazyLock<String> = LazyLock::new(|| format!("{}/{}", ROOT, $path));
    };
}

macro_rules! path {
    ($name: expr) => {
        &(*$name)
    };
}

relative_path!(ASSETS_DIR, "assets");
static ASSETS_FILE: &str = "assets.rdss";

relative_path!(BANKS_DIR, "fmod/Build/Desktop");
relative_path!(BANKS_DEST, "assets/sound");

fn main() {
    if let Err(e) = try_main() {
        eprintln!("Error: {:?}", e);
        std::process::exit(1);
    }
}

fn try_main() -> Result<()> {
    compile_assets().context("Failed to compile assets")?;
    copy_banks().context("Failed to copy banks")?;

    Ok(())
}

fn watch_dir(dir: impl Into<std::path::PathBuf>) {
    let entries = std::fs::read_dir(dir.into())
        .expect("Failed to read directory")
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .collect::<Vec<_>>();
    for entry in entries {
        if entry.is_dir() {
            watch_dir(entry);
        } else {
            println!("cargo:rerun-if-changed={}", entry.display());
        }
    }
}

fn get_out_dir() -> Result<PathBuf> {
    let mut out = std::path::PathBuf::from(
        std::env::var("OUT_DIR").context("OUT_DIR environment variable not set")?,
    );
    for _ in 1..4 {
        out.pop();
    }
    Ok(out)
}

fn compile_assets() -> Result<()> {
    let out = get_out_dir().context("Failed to get output directory")?;
    let compiler = rdss::Compiler::builder()
        .from_sources(path!(ASSETS_DIR))
        .save_to(out.join(path!(ASSETS_FILE)))
        .build()
        .context("Failed to build compiler")?;
    compiler.compile().context("Compilation failed")?;
    watch_dir(path!(ASSETS_DIR));
    Ok(())
}

fn copy_banks() -> Result<()> {
    for entry in std::fs::read_dir(path!(BANKS_DIR))
        .context(format!("Failed to read directory {}", path!(BANKS_DIR)))?
        .filter_map(Result::ok)
    {
        let src = entry.path();
        if src.is_file() {
            let dest = std::path::PathBuf::from(path!(BANKS_DEST)).join(src.file_name().unwrap());
            std::fs::copy(&src, &dest).context(format!(
                "Failed to copy bank file from {} to {}",
                src.display(),
                dest.display()
            ))?;
        }
    }
    watch_dir(path!(BANKS_DIR));
    Ok(())
}
