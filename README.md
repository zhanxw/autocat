# autocat

![Build](https://img.shields.io/badge/build-passing-brightgreen)
![License: MIT](https://img.shields.io/badge/License-MIT-blue)
![Rust](https://img.shields.io/badge/Rust-stable-orange)

A powerful Rust CLI tool that transparently reads and decompresses file contents to **stdout**. It automatically handles multiple formats (gzip, bzip2, zstd, lzma) and supports both file paths and **stdin** — perfect for inspecting compressed data without manual decompression.

---

## 🧩 Installation

### Option 1 — From local source

```bash
cargo install --path .
```

This installs `autocat` into Cargo’s bin directory (usually `~/.cargo/bin`).

---

### Option 2 — Directly from GitHub

Install the latest version straight from the repository:

```bash
cargo install --git https://github.com/zhanxw/autocat
```

---

## 🛠️ Build from source

If you just want to build without installing:

```bash
cargo build --release
```

The compiled binary will be at:
```
target/release/autocat
```

---

## 🖼️ Preview

Here’s `autocat` in action, showing transparent decompression:

```bash
$ echo "Hello World" | gzip > test.gz
$ autocat test.gz
Hello World

$ autocat data.txt.gz config.json.zst logs.bz2
[... combined decompressed output ...]
```

---

## ⚙️ Supported Formats

| Format | Extension | Detection |
|--------|-----------|-----------|
| **Plain Text** | (any) | Content-based |
| **Gzip** | `.gz` | Magic bytes |
| **Bgzip** | `.gz` | Magic bytes |
| **Bzip2** | `.bz2` | Magic bytes |
| **Zstandard** | `.zst` | Magic bytes |
| **XZ / LZMA** | `.xz` | Magic bytes |

---

## 🧮 Usage examples and piping

You can use `autocat` just like `cat`, but for compressed data.  
Here are some practical examples:

### 1️⃣ From multiple compressed files
Seamlessly read through different archive types:

```bash
autocat README.md log.bz2 records.zst
```

---

### 2️⃣ Pipe from stdin
Process compressed streams from other programs:

```bash
cat archive.xz | autocat
```

---

### 3️⃣ Process data from a URL
Combine with `curl` for quick inspection of remote files:

```bash
curl -L https://example.com/data.tar.gz | autocat
```

---

## 📄 License

MIT License © 2026

This program is free software: you can redistribute it and/or modify
it under the terms of the MIT License.
