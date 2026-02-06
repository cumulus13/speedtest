# Platform Support

This document details the platform support matrix for speedtest.

## ✅ Tier 1 Support (Fully Tested & Guaranteed)

These platforms are tested on every commit and guaranteed to work:

| Platform | Architecture | Target Triple | Notes |
|----------|-------------|---------------|-------|
| Linux | x86_64 | `x86_64-unknown-linux-gnu` | glibc 2.17+ |
| Linux | x86_64 | `x86_64-unknown-linux-musl` | Static binary |
| Windows | x86_64 | `x86_64-pc-windows-msvc` | Windows 7+ |
| macOS | x86_64 | `x86_64-apple-darwin` | macOS 10.12+ |
| macOS | ARM64 | `aarch64-apple-darwin` | macOS 11.0+ (M1/M2/M3) |

## ✨ Tier 2 Support (Tested via Cross-compilation)

These platforms are built and tested via cross-compilation:

### Linux
| Architecture | Target Triple | Notes |
|-------------|---------------|-------|
| x86 32-bit | `i686-unknown-linux-gnu` | Legacy support |
| x86 32-bit | `i686-unknown-linux-musl` | Static binary |
| ARM64 | `aarch64-unknown-linux-gnu` | Raspberry Pi 4, ARM servers |
| ARM64 | `aarch64-unknown-linux-musl` | Static binary |
| ARMv7 | `armv7-unknown-linux-gnueabihf` | Raspberry Pi 2/3 |
| ARMv7 | `armv7-unknown-linux-musleabihf` | Static binary |
| ARMv6 | `arm-unknown-linux-gnueabihf` | Raspberry Pi 1/Zero |
| ARMv6 | `arm-unknown-linux-musleabihf` | Static binary |
| RISC-V 64 | `riscv64gc-unknown-linux-gnu` | RISC-V boards |
| PowerPC 64 | `powerpc64le-unknown-linux-gnu` | IBM POWER8+ |
| MIPS 64 | `mips64el-unknown-linux-gnuabi64` | MIPS64 systems |
| S390X | `s390x-unknown-linux-gnu` | IBM Z mainframes |

### Windows
| Architecture | Target Triple | Notes |
|-------------|---------------|-------|
| x86_64 GNU | `x86_64-pc-windows-gnu` | MinGW toolchain |
| x86 32-bit | `i686-pc-windows-msvc` | Legacy Windows |
| ARM64 | `aarch64-pc-windows-msvc` | Windows on ARM |

### BSD
| Platform | Target Triple | Notes |
|----------|---------------|-------|
| FreeBSD | `x86_64-unknown-freebsd` | FreeBSD 12+ |
| NetBSD | `x86_64-unknown-netbsd` | NetBSD 9+ |

### Android
| Architecture | Target Triple | Notes |
|-------------|---------------|-------|
| ARM64 | `aarch64-linux-android` | Android 5.0+ (API 21+) |
| ARMv7 | `armv7-linux-androideabi` | Android 4.0+ (API 16+) |
| x86_64 | `x86_64-linux-android` | Android emulators |

## 🔧 Tier 3 Support (May Work, Not Tested)

These platforms should theoretically work but are not regularly tested:

- OpenBSD (x86_64)
- DragonFly BSD (x86_64)
- Solaris/illumos (x86_64)
- Fuchsia
- WASM32 (with network support)

## ⚙️ Platform-Specific Notes

### Linux MUSL
- Uses `rustls` instead of `native-tls` for pure Rust TLS
- Produces fully static binaries with no dependencies
- Slightly larger binary size but maximum portability
- Recommended for containers and embedded systems

### Windows ARM64
- Requires Windows 10 on ARM or Windows 11
- Native ARM64 execution on Snapdragon PCs
- Full feature parity with x86_64 version

### macOS Apple Silicon
- Native ARM64 performance on M1/M2/M3 chips
- Requires macOS 11.0 (Big Sur) or later
- Rosetta 2 not needed

### Android
- Requires network permissions in AndroidManifest.xml:
  ```xml
  <uses-permission android:name="android.permission.INTERNET" />
  <uses-permission android:name="android.permission.ACCESS_NETWORK_STATE" />
  ```
- CLI may need Termux or similar terminal emulator

### Raspberry Pi
- **Pi 4/400**: Use `aarch64-unknown-linux-gnu` (64-bit OS)
- **Pi 2/3**: Use `armv7-unknown-linux-gnueabihf`
- **Pi 1/Zero**: Use `arm-unknown-linux-gnueabihf`
- Consider MUSL variants for older OS versions

### RISC-V
- Tested on QEMU, should work on real hardware
- Requires RISC-V with C and D extensions
- Limited real hardware availability for testing

### PowerPC
- Little-endian only (ppc64le)
- Tested on IBM POWER8/POWER9
- Should work on modern OpenPOWER systems

### IBM Z (s390x)
- Tested on z/VM and Linux on Z
- Requires IBM z13 or newer
- Full 64-bit mainframe support

## 🔍 Feature Availability by Platform

| Feature | Linux | Windows | macOS | BSD | Android |
|---------|-------|---------|-------|-----|---------|
| HTTP/HTTPS | ✅ | ✅ | ✅ | ✅ | ✅ |
| Parallel Downloads | ✅ | ✅ | ✅ | ✅ | ✅ |
| Progress Bars | ✅ | ✅ | ✅ | ✅ | ⚠️¹ |
| JSON Output | ✅ | ✅ | ✅ | ✅ | ✅ |
| CSV Output | ✅ | ✅ | ✅ | ✅ | ✅ |
| Share Results | ✅ | ✅ | ✅ | ✅ | ✅ |
| Source Binding | ✅ | ⚠️² | ✅ | ✅ | ⚠️³ |

¹ Progress bars may not display correctly in some Android terminals
² Source address binding has limited support on Windows
³ Requires root/system permissions on Android

## 🛠️ Build Requirements by Platform

### Native Builds
- **Linux**: gcc/clang, pkg-config, openssl-dev (glibc) or musl-dev (musl)
- **Windows**: MSVC Build Tools or MinGW-w64
- **macOS**: Xcode Command Line Tools
- **FreeBSD**: clang, pkg-config
- **Android**: Android NDK

### Cross-compilation
All cross-compilation uses the `cross` tool:
```bash
cargo install cross --git https://github.com/cross-rs/cross
cross build --release --target <target-triple>
```

## 📦 Binary Sizes

Approximate binary sizes for release builds with LTO and strip:

| Platform | Size (Stripped) | Size (w/ Debug) |
|----------|----------------|-----------------|
| Linux x86_64 (glibc) | ~3.5 MB | ~18 MB |
| Linux x86_64 (musl) | ~4.2 MB | ~20 MB |
| Windows x86_64 | ~3.8 MB | ~22 MB |
| macOS x86_64 | ~3.6 MB | ~19 MB |
| macOS ARM64 | ~3.4 MB | ~18 MB |
| Linux ARM64 | ~3.8 MB | ~20 MB |
| Android ARM64 | ~4.0 MB | ~21 MB |

## 🐛 Known Issues

### MUSL Targets
- DNS resolution may be slower than glibc
- Some older musl versions (<1.2.0) may have issues
- **Workaround**: Use recent musl toolchains

### Windows ARM64
- Limited testing hardware availability
- Some third-party network tools may not work
- **Status**: Should work but not extensively tested

### Android
- Terminal UI may not work in all terminal emulators
- **Workaround**: Use `--simple` or `--json` output

### BSD Variants
- OpenSSL linking may vary by BSD version
- **Workaround**: Use static linking or specify OpenSSL path

## 🧪 Testing

### Run Platform Tests
```bash
# Native platform
cargo test

# Cross-platform (requires cross)
cross test --target <target-triple>
```

### Verify Binary
```bash
# Linux
ldd ./target/release/speedtest  # Check dependencies

# Check platform
file ./target/release/speedtest

# Test run
./target/release/speedtest --version
```

## 📝 Reporting Issues

If you encounter issues on a specific platform:

1. Check if it's a Tier 1/2/3 platform
2. Include:
   - Platform and architecture
   - OS version
   - Rust version
   - Full error output
   - Output of `speedtest --version`
3. Open an issue on GitHub with the `platform` label

## 🚀 Future Platform Support

Platforms under consideration:

- **Tier 2 → Tier 1**: Android ARM64, FreeBSD
- **Tier 3 → Tier 2**: OpenBSD, Fuchsia
- **New Platforms**: LoongArch, SPARC64

Contributions for new platforms are welcome!
