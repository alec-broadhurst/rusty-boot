use std::path::Path;

fn main() {
    let mut build = cc::Build::new();

    let asm_dir = Path::new("src/asm");
    println!("cargo:rerun-if-changed={}", asm_dir.display());

    if asm_dir.exists() && asm_dir.is_dir() {
        for entry in asm_dir.read_dir().expect("Failed to read asm directory") {
            let entry = entry.expect("Failed to read directory entry");
            let path = entry.path();

            if let Some(ext) = path.extension() {
                if ext == "s" {
                    println!("cargo:rerun-if-changed={}", path.display());
                    build.file(path);
                }
            }
        }
    }

    build
        .compiler("avr-gcc")
        .archiver("avr-ar")
        .include("src/asm")
        .flag("-mmcu=atmega328p")
        .compile("asm");
}
