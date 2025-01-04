# WineBridge-PoC

## Prerequisites

Before you begin, ensure that you have met the following requirements:

1. **Install Rust via `rustup`:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. **Install the MinGW-w64 toolchain**
```bash
sudo dnf install mingw64-gcc mingw64-gcc-c++ mingw64-binutils
```

Also download the rust toolchain for the MinGW target:
```bash
rustup target add x86_64-pc-windows-gnu
```

3. Install **Wine**
```bash
sudo dnf install wine-core
```

## Building
```bash
cargo build --release
```

Use Wine to execute the compiled binary:

```bash
wine target/x86_64-pc-windows-gnu/release/winebridge_poc.exe
```
