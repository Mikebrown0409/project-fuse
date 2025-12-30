//! Build script for fuse-core
//!
//! This script checks if the guest program ELF binary exists and
//! sets up the environment for including it in the host program.

fn main() {
    // Check if guest program ELF exists
    // Try workspace target directory first (default for cargo workspaces)
    let guest_elf_paths = [
        "../target/riscv32im-risc0-zkvm-elf/release/fuse-guest",  // Workspace target (primary)
        "../fuse-guest/target/riscv32im-risc0-zkvm-elf/release/fuse-guest",  // Package target (fallback)
    ];
    
    let mut found = false;
    for guest_elf_path in &guest_elf_paths {
        let path = std::path::Path::new(guest_elf_path);
        if path.exists() {
            println!("cargo:rustc-cfg=guest_program_built");
            println!("cargo:rerun-if-changed={}", guest_elf_path);
            eprintln!("[build.rs] ✅ Found guest ELF at: {}", guest_elf_path);
            found = true;
            break;
        } else {
            eprintln!("[build.rs] ❌ Not found: {}", guest_elf_path);
        }
    }
    
    if !found {
        eprintln!("[build.rs] ⚠️  Guest program not built. Run: cargo build -p fuse-guest --release --target riscv32im-risc0-zkvm-elf");
        println!("cargo:warning=Guest program not built. Run: cargo build -p fuse-guest --release --target riscv32im-risc0-zkvm-elf");
    }
}

