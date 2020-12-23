# HiFive1 Rev B Cargo Template

A template for running Rust on RISC-V based [HiFive1 boards](https://www.sifive.com/boards/hifive1-rev-b) with [SEGGER Embedded Studio](https://www.segger.com/products/development-tools/embedded-studio/editions/risc-v/). See the [`riscv-rust-quickstart`](https://github.com/riscv-rust/riscv-rust-quickstart) repository for examples and other templates you can use to get started.

## Prerequisites

To get started, you'll need to:

- grab [a prebuilt RISC-V toolchain](https://www.sifive.com/software) for your host platform and add the `riscv64-unknown-elf-gcc` binary to your `PATH`.
- install [SEGGER Embedded Studio](https://www.segger.com/products/development-tools/embedded-studio/editions/risc-v/).

## Installing the template

You can use the [`cargo generate`](https://crates.io/crates/cargo-generate) tool to clone this repository:

```
cargo generate --git https://github.com/KodrAus/hifive1-revb-template
```

You can then open the `.emProject` file in Embedded Studio, which can take care of building, flashing, and debugging your program on an attached device.
