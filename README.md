# autocat

`autocat` is a Rust-based command-line tool that transparently decompresses and outputs file contents to stdout. It supports various compression formats, making it a versatile tool for inspecting compressed files without needing to remember specific decompression commands.

## Features

- **Multi-format detection**: Automatically detects and decompresses multiple formats.
- **Supported formats**:
    - Plain text
    - Gzip (`.gz`)
    - Bgzip
    - Bzip2 (`.bz2`)
    - Zstandard (`.zst`)
    - XZ / LZMA (`.xz`)
- **Flexible input**: Supports reading from multiple files or streaming from stdin.
- **Fast and efficient**: Built with Rust for performance.

## Installation

Ensure you have Rust and Cargo installed, then:

```bash
git clone https://github.com/zhanxw/autocat.git
cd autocat
cargo build --release
```

The binary will be available at `target/release/autocat`.

## Usage Examples

### Read a single compressed file
```bash
autocat data.gz
```

### Read multiple files of different formats
```bash
autocat README.md log.bz2 records.zst
```

### Pipe from stdin
```bash
cat archive.xz | autocat
```

### Process data from a URL
```bash
curl -L https://example.com/data.tar.gz | autocat
```

## Dependencies

- [niffler](https://crates.io/crates/niffler): Transparent decompression core.
- [clap](https://crates.io/crates/clap): CLI argument parsing.
- [anyhow](https://crates.io/crates/anyhow): Error handling.
