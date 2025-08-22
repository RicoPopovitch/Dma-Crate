
use std::env;
use std::path::Path;

fn main() {
    println!("building...");
    
    // Link the DMA library DLLs
    println!("cargo:rustc-link-search=native=src/vendor/DMALibrary/libs");
    println!("cargo:rustc-link-lib=dylib=vmm");
    println!("cargo:rustc-link-lib=dylib=FTD3XX");
    println!("cargo:rustc-link-lib=dylib=leechcore");
    
    // Add the cc crate dependency
    cc::Build::new()
        .cpp(true)
        .std("c++20")
        .include("src/vendor")
        .include("src/vendor/DMALibrary")
        .include("src/vendor/DMALibrary/libs")
        .file("src/vendor/DMALibrary/Memory/Memory.cpp")
        .file("src/vendor/DMALibrary/Memory/InputManager.cpp")
        .file("src/vendor/DMALibrary/Memory/Registry.cpp")
        .file("src/vendor/DMALibrary/Memory/Shellcode.cpp")
        .file("src/vendor/DMALibrary/Memory/wrapper.cpp")
        .compile("dma");
    
    // Copy DLLs to output directory so they're found at runtime
    let out_dir = env::var("OUT_DIR").unwrap();
    let target_dir = Path::new(&out_dir).ancestors().nth(3).unwrap();
    
    // Create target directory if it doesn't exist
    std::fs::create_dir_all(&target_dir).ok();
    
    // Copy each DLL
    for dll in &["vmm.dll", "FTD3XX.dll", "leechcore.dll"] {
        let src = format!("src/vendor/DMALibrary/libs/{}", dll);
        let dst = target_dir.join(dll);
        println!("cargo:warning=Copying {} to {}", src, dst.display());
        if let Err(e) = std::fs::copy(&src, &dst) {
            println!("cargo:warning=Failed to copy {}: {}", dll, e);
        } else {
            println!("cargo:warning=Successfully copied {}", dll);
        }
    }
        
    println!("done...");
} 