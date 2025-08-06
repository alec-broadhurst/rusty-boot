use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mcu = "-mmcu=atmega328p";

    for src in &[
        "src/asm/start.S",
        "src/asm/erase_page.S",
        "src/asm/write_page.S",
        "src/asm/spm_poll.S",
        "src/asm/fill_page_buffer.S",
        "src/asm/reenable_rww.S",
        "src/asm/jmp_to_app.S",
    ] {
        let obj = out.join(src.split('/').last().unwrap().replace(".S", ".o"));
        Command::new("avr-gcc")
            .args(&[mcu, "-c", src, "-o"])
            .arg(&obj)
            .status()
            .expect("Compile failed");
    }

    let mut ar = Command::new("avr-ar");
    let lib = out.join("libasm.a");
    ar.arg("crs").arg(&lib);
    for file in out.read_dir().unwrap() {
        let path = file.unwrap().path();
        if path.extension().and_then(|e| e.to_str()) == Some("o") {
            ar.arg(path);
        }
    }
    ar.status().expect("Archive failed");

    println!("cargo:rustc-link-search=native={}", out.display());
    println!("cargo:rustc-link-lib=static=asm");
    println!("cargo:rustc-link-lib=gcc");
    println!("cargo:rustc-link-lib=c");
}
