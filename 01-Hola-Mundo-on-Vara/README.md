# Hola mundo tutorial


## Paso 1: Crear una libreria de Rust

**comando:**
```bash
cargo new hello-world --lib
```

## Paso 2: En en cargo.toml, colocar las siguientes dependencias:

**comando:**

```bash
[package]
name = "hello-world"
version = "0.1.0"
edition = "2021"

[dependencies]
gstd = { git = "https://github.com/gear-tech/gear.git", tag = "v1.0.2", features = ["debug"] }

[build-dependencies]
gear-wasm-builder = { git = "https://github.com/gear-tech/gear.git", tag = "v1.0.2", features = ["wasm-opt"] }

[dev-dependencies]
gtest = { git = "https://github.com/gear-tech/gear.git", rev = "946ac47" }

```

## Paso 3: Crear el archivo de compilación build.rs:

**comando:**
```bash
fn main() {
    gear_wasm_builder::build();
}

```

## Paso 4: Crear el archivo de rust-toolchain.toml

**comando:**
```bash
[toolchain]
channel = "nightly-2023-09-18"
targets = ["wasm32-unknown-unknown"]
profile = "default"
```


##  Paso 5: Crear el archivo lib.rs:

**comando:**
```bash
#![no_std]
use gstd::{msg, prelude::*};

#[no_mangle]
extern "C" fn handle() {
    msg::reply(String::from("Hello"), 0)
        .expect("Error in sending a reply message");
}
```

##  Paso 6: Compilar el contrato inteligente usando el comando:

**comando:**
```bash
cargo build --release
```


## Paso 7: Deplegar en el gear-Idea 





