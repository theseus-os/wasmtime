use std::env;

fn main() {
    println!("cargo:rerun-if-changed=src/helpers.c");
    let mut build = cc::Build::new();
    build.warnings(true)
        .define(
            &format!("CFG_TARGET_OS_{}", env::var("CARGO_CFG_TARGET_OS").unwrap()),
            None,
        )
        .file("src/helpers.c");
    
        if env::var("CARGO_CFG_TARGET_OS").unwrap().contains("theseus") {
            // When building Theseus on a macOS host machine,
            // we need to use the "x86_64-elf-" cross compiler toolchain,
            // and also tell that compiler where the sysroot include directory is
            // such that it can find the required header files.
            if env::var("HOST").unwrap().contains("darwin") {
                let xcrun_output = std::process::Command::new("xcrun")
                    .arg("--show-sdk-path")
                    .output()
                    .expect("Failed to run \"xcrun --show-sdk-path\"")
                    .stdout;
                let sdk_path = String::from_utf8(xcrun_output).unwrap();
                let include_path = format!("{}/usr/include", sdk_path.trim());
                build
                    .compiler("x86_64-elf-gcc")
                    .archiver("x86_64-elf-ar")
                    .ranlib("x86_64-elf-ranlib")
                    .include(include_path);
            }

            build
                .use_plt(false)
                .pic(false)
                // Theseus doesn't yet support the GOT (Global Offset Table),
                // so we disable usage of the GOT by forcing no PLT and PIC.
                // In Rust, this is accomplished using `-C relocation-model=static`.
                .flag("-fno-plt")
                .flag("-fno-pic")
                // Theseus currently uses the large code model.
                .flag("-mcmodel=large") 
                // Disable stack smashing protection
                .flag("-fno-stack-protector");

                // eprintln!("Theseus build: {:#?}\n{:#?}", build, build.get_compiler());
                // eprintln!("Theseus build command: {:?}", build.get_compiler().to_command());
                // eprintln!("Theseus $(HOST)={:?}", env::var("HOST"));
        }

    build.compile("wasmtime-helpers");
}
