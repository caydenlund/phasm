# Phasm

**Phasm** is a fast and flexible assembler written in Rust.
It aims to support encoding and decoding assembly instructions for x86 and ARM, with ELF integration.


## Goals

- **Encoding & Decoding**: Convert assembly code to machine code and vice versa.
- **Multi-Architecture Support**: Supports **x86** and **ARM** architectures.
- **ELF Integration**: Work with ELF binaries.
- **Modular Design**: Enable only the features you need.


## Installation

Add `phasm` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
phasm = "0.1"
```

To use specific features:

```toml
[dependencies]
phasm = { version = "0.1", features = ["x86", "encode"] }
```


## Usage

### Encoding an Instruction

```rust
use phasm::arm::ArmAssembler;

let machine_code = ArmAssembler::encode("mov r0, r1").unwrap();
println!("{:x?}", machine_code);
```

### Decoding an Instruction

```rust
use phasm::x86::X86Assembler;

let assembly = X86Assembler::decode(&[0x89, 0xD8]).unwrap();
println!("{:?}", assembly);
```


## Feature Flags

|  Feature   |  Description                        |
|------------|-------------------------------------|
|  `encode`  |  Enables instruction encoding       |
|  `decode`  |  Enables instruction decoding       |
|  `x86`     |  Adds support for x86 architecture  |
|  `arm`     |  Adds support for ARM architecture  |
|  `elf`     |  Enables ELF binary integration     |
