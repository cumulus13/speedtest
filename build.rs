// build.rs - Platform-specific build configuration

use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();

    println!("cargo:rerun-if-changed=build.rs");
    
    // Print build info
    println!("cargo:warning=Building for target: {}", target);
    println!("cargo:warning=Target OS: {}", target_os);
    println!("cargo:warning=Target Architecture: {}", target_arch);
    println!("cargo:warning=Target Environment: {}", target_env);

    // Platform-specific configurations
    match target_os.as_str() {
        "linux" => {
            println!("cargo:rustc-cfg=target_os_linux");
            
            // Use rustls for MUSL targets
            if target_env == "musl" {
                println!("cargo:rustc-cfg=use_rustls");
            }
        }
        "windows" => {
            println!("cargo:rustc-cfg=target_os_windows");
            
            // Windows-specific settings
            if target_arch == "aarch64" {
                println!("cargo:warning=Building for Windows ARM64");
            }
        }
        "macos" => {
            println!("cargo:rustc-cfg=target_os_macos");
        }
        "freebsd" | "netbsd" | "openbsd" | "dragonfly" => {
            println!("cargo:rustc-cfg=target_os_bsd");
        }
        "android" => {
            println!("cargo:rustc-cfg=target_os_android");
        }
        _ => {
            println!("cargo:warning=Building for uncommon platform: {}", target_os);
        }
    }

    // Architecture-specific configurations
    match target_arch.as_str() {
        "x86_64" | "x86" => {
            println!("cargo:rustc-cfg=target_arch_x86");
        }
        "aarch64" | "arm" | "armv7" => {
            println!("cargo:rustc-cfg=target_arch_arm");
        }
        "riscv64" | "riscv32" => {
            println!("cargo:rustc-cfg=target_arch_riscv");
        }
        "powerpc64" | "powerpc" => {
            println!("cargo:rustc-cfg=target_arch_powerpc");
        }
        "mips64" | "mips" => {
            println!("cargo:rustc-cfg=target_arch_mips");
        }
        "s390x" => {
            println!("cargo:rustc-cfg=target_arch_s390x");
        }
        _ => {}
    }

    // Endianness detection
    if target.contains("le") || target_arch == "x86_64" || target_arch == "aarch64" {
        println!("cargo:rustc-cfg=target_endian_little");
    } else if target.contains("be") {
        println!("cargo:rustc-cfg=target_endian_big");
    }

    // Check for specific features we might need
    check_network_features(&target_os);
}

fn check_network_features(target_os: &str) {
    // Ensure network capabilities are available
    match target_os {
        "linux" | "macos" | "freebsd" | "netbsd" | "openbsd" => {
            println!("cargo:rustc-cfg=has_unix_sockets");
        }
        "windows" => {
            println!("cargo:rustc-cfg=has_windows_sockets");
        }
        "android" => {
            println!("cargo:rustc-cfg=has_unix_sockets");
            println!("cargo:warning=Building for Android - ensure permissions are set");
        }
        _ => {}
    }
}
