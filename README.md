# 🦀 ruoka-rs

Since `ruoka-rs` is written in Rust, it is a tool to **quickly** get the menu from [mayk.fi/tietoa-meista/ruokailu](https://mayk.fi/tietoa-meista/ruokailu).
The program `curl`s the html, parses it with Regex and makes it more _colorful_.

## Installation

Run the following in your terminal:

```bash
cargo build # compile
mv ./target/debug/ruoka-rs /usr/bin # move the program to bin (superuser)
```

### **OR**

Install the [package](https://crates.io/crates/ruoka-rs) via `cargo`:

```bash
cargo install ruoka-rs
```

## Missing `cargo`?

### 🦀 On **Linux** and **MacOS** systems, run:

```bash
curl https://sh.rustup.rs -sSf | sh
```

### 🦀 On **Windows**, download and run **[rustup-init.exe](https://win.rustup.rs)**. It will start the installation in a console.

For other installation options and information, visit the [install](https://www.rust-lang.org/tools/install) page of the Rust website.

### 🦀 **Build and Install Cargo from Source**

Alternatively, you can [build `Cargo` from source](https://github.com/rust-lang/cargo#compiling-from-source).

## **Enjoy!**

(Also, feel free to contact [zeriaxdev](https://github.com/zeriaxdev) if you have any issues or questions.)
