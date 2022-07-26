# 🦀 ruoka-rs

A tool to **quickly** get the menu from [mayk.fi/tietoa-meista/ruokailu](https://mayk.fi/tietoa-meista/ruokailu).
The program `curl`s the html, parses it with Regex and makes it more _colorful_.

<p><img src=".github/assets/demo.gif" alt="demo.gif" width="500"></p>

## Installation

### **Linux** & **MacOS**

Run the following in your terminal:

```sh
cargo build                           # compile
mv ./target/debug/ruoka-rs /usr/bin   # move the program to bin
```

### **Other systems**

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
